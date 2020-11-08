use crate::parser::astnode::{AstNode, CalcOp, Function, LogicalOperatorType, ComparisonlOperatorType};
use crate::value::oran_value::{OranValue, FunctionDefine};
use crate::value::oran_variable::{OranVariable, OranVariableValue};
use crate::value::oran_string::OranString;
use crate::value::var_type::{VarType, FunctionOrValueType};
use std::collections::HashMap;
use std::io::{self, Write};
use std::borrow::Cow;

pub fn interp_expr<'a>(scope: usize, env : &mut HashMap<(usize, FunctionOrValueType, OranString<'a>), OranValue<'a>>, reduced_expr: &'a AstNode, var_type: FunctionOrValueType) -> OranValue<'a> {
    match reduced_expr {
        AstNode::Number(double) => OranValue::Float(*double),
        AstNode::Calc (verb, lhs, rhs) => {
            match verb {
                CalcOp::Plus => { interp_expr(scope, env, lhs, var_type) + interp_expr(scope, env, rhs, var_type) }
                CalcOp::Minus => { interp_expr(scope, env, lhs, var_type) - interp_expr(scope, env, rhs, var_type) }
                CalcOp::Times => { interp_expr(scope, env, lhs, var_type) * interp_expr(scope, env, rhs, var_type) }
                CalcOp::Divide => { interp_expr(scope, env, lhs, var_type) / interp_expr(scope, env, rhs, var_type) }
                CalcOp::Modulus => { interp_expr(scope, env, lhs, var_type) % interp_expr(scope, env, rhs, var_type) }
                CalcOp::Power => {
                    let base: f64 = f64::from(interp_expr(scope, env, lhs, var_type));
                    let pow: f64 = f64::from(interp_expr(scope, env, rhs, var_type));
                    let val = base.powf(pow);
                    OranValue::Float(val)
                }
            }
        }
        AstNode::Ident(ident) => {
            let val = &*env.get(
                &(
                    scope,
                    var_type,
                    OranString::from(ident)
                )
            ).unwrap_or_else(
                || panic!("The variable \"{}\" is not defined.", ident)
            );
            val.clone()
        }
        AstNode::Assign(variable_type, ident, expr) => {
            is_mutable(scope, env, ident, variable_type);
            let oran_val = OranValue::Variable(OranVariable {
                var_type: *variable_type,
                name: ident,
                value: OranVariableValue::from(&interp_expr(scope, env, expr, var_type)),
            });
            env.insert((scope, var_type, OranString::from(ident)), oran_val.clone());
            oran_val
        }
        AstNode::FunctionCall(func_ast, name, arg_values) => {
            match func_ast {
                Function::Print => {
                    let mut text = "".to_string();
                    for str in arg_values {
                        text.push_str(&String::from(&interp_expr(scope, env, &str, var_type)))
                    }
                    print!("{}", text);
                    io::stdout().flush().unwrap();
                    OranValue::Boolean(true)
                },
                Function::Println => {
                    let mut text = "".to_string();
                    for str in arg_values {
                        text.push_str(&String::from(&interp_expr(scope, env, &str, var_type)))
                    }
                    println!("{}", text);
                    OranValue::Boolean(true)
                },
                Function::NotDefault => {
                    let func = *&env.get(&(scope, FunctionOrValueType::Function, OranString::from(name)));
                    let func = match func {
                        None => panic!("Function {} is not defined.", name),
                        _ => func.unwrap()
                    };
                    let func = FunctionDefine::from(func);
                    for i in 0..func.args.len() {
                        let arg_name = interp_expr(scope+1, env, func.args.into_iter().nth(i).unwrap(), var_type);
                        let arg_ast = arg_values.into_iter().nth(i).unwrap_or_else(|| panic!("Argument is necessary but not supplied."));
                        let val = interp_expr(scope, env, arg_ast, var_type);
                        env.insert((scope+1, var_type, OranString::from(arg_name)), val);
                    }
                    for body in func.body {
                        interp_expr(scope+1, env, &body, var_type);
                    }
                    let val = interp_expr(scope+1, env, func.fn_return, var_type);
                    // delete unnecessary data when exiting a scope
                    // TODO garbage colloctor
                    // TODO conditional return in functions
                    env.retain(|(s, __k, _label), _val| *s != scope+1);
                    val
                }
            }
        }
        AstNode::FunctionDefine(func_name, args, astnodes, fn_return) => {
            let val = OranValue::Function(FunctionDefine {
                name: func_name,
                args: args,
                body: astnodes,
                fn_return: fn_return
            });
            env.insert((scope, FunctionOrValueType::Function, OranString::from(func_name)), val.clone());
            val
        }
        AstNode::Argument(argument_name, val) => {
            let val = interp_expr(scope, env, val, var_type);
            env.insert((scope, FunctionOrValueType::Value, OranString::from(argument_name)), val);
            OranValue::Str(OranString::from(argument_name))
        }
        AstNode::Str (str_val) => {
            OranValue::Str(OranString::from(str_val))
        }
        AstNode::Strs (strs) => {
            let mut text = "".to_string();
            for str in strs {
                text.push_str(&String::from(interp_expr(scope, env, &str, var_type)))
            }
            OranValue::Str(OranString {
                val_str: Cow::from(text)
            })
        }
        AstNode::Condition (c, e, o) => {
            let e = interp_expr(scope, env, e, var_type);
            let o = interp_expr(scope, env, o, var_type);
            match c {
                ComparisonlOperatorType::AND => {
                    if bool::from(e) && bool::from(o) {
                        return OranValue::Boolean(true);
                    }
                    OranValue::Boolean(false)
                }
                ComparisonlOperatorType::OR => {
                    if bool::from(e) || bool::from(o) {
                        return OranValue::Boolean(true);
                    }
                    OranValue::Boolean(false)
                }
            }
        }
        AstNode::Comparison (e, c, o) => {
            let e = interp_expr(scope, env, e, var_type);
            let o = interp_expr(scope, env, o, var_type);

            let is_num_e  = match Result::<f64, String>::from(&e) {
                Ok(_v) => true,
                Err(_e) => false
            };
            let is_num_o = match Result::<f64, String>::from(&o) {
                Ok(_v) => true,
                Err(_e) => false
            };

            if !is_num_e || !is_num_o {
                match c {
                    LogicalOperatorType::Equal => {
                        if e.to_string() == o.to_string() {
                            return OranValue::Boolean(true);
                        }
                        OranValue::Boolean(false)
                    },
                    _ => panic!("One of these values are not Number: \"{}\", \"{}\"", e, o)
                }
            } else {
                match c {
                    LogicalOperatorType::Equal => {
                        if e == o {
                            return OranValue::Boolean(true);
                        }
                        OranValue::Boolean(false)
                    },
                    LogicalOperatorType::BiggerThan => {
                        if e > o {
                            return OranValue::Boolean(true);
                        }
                        OranValue::Boolean(false)
                    },
                    LogicalOperatorType::SmallerThan => {
                        if e < o {
                            return OranValue::Boolean(true);
                        }
                        OranValue::Boolean(false)
                    },
                    LogicalOperatorType::EbiggerThan => {
                        if e >= o {
                            return OranValue::Boolean(true);
                        }
                        OranValue::Boolean(false)
                    },
                    LogicalOperatorType::EsmallerThan => {
                        if e <= o {
                            return OranValue::Boolean(true);
                        }
                        OranValue::Boolean(false)
                    },
                }
            }
        }
        AstNode::IF(if_conditions, body, else_if_bodies_conditions, else_bodies) => {
            // if
            let condition_result = interp_expr(scope, env, if_conditions, var_type);
            if bool::from(condition_result) {
                for astnode in body {
                    interp_expr(scope, env, &astnode, var_type);
                }
                return OranValue::Null;
            }
            // else if
            let mut _is_all_false = true;
            if !else_if_bodies_conditions.is_empty() {
                for (conditions, else_if_body) in else_if_bodies_conditions {
                    for c in conditions {
                        let result = interp_expr(scope, env, &c, var_type);
                        if bool::from(result) {
                            _is_all_false = false;
                            for astnode in else_if_body {
                                interp_expr(scope, env, &astnode, var_type);
                            }
                            return OranValue::Null;
                        }
                    }
                    
                }
                if _is_all_false == false {
                    return OranValue::Null;
                }
            }
            // else
            if !else_bodies.is_empty() {
                for astnode in else_bodies {
                    interp_expr(scope, env, astnode, var_type);
                }
            }
            //env.retain(|(_s, k, _label), _val| *k != FunctionOrValueType::Temp);
            OranValue::Null
        }
        AstNode::Bool (b) => {
            OranValue::Boolean(*b)
        }
        AstNode::ForLoop(is_inclusive, var_type, i, first, last, stmts) => {
            let first = interp_expr(scope, env, first, FunctionOrValueType::Value);
            let first = f64::from(first).round() as i64;
            let last = interp_expr(scope, env, last, FunctionOrValueType::Value);
            let last = f64::from(last).round() as i64;
            let i_name = OranString::from(i);
            match is_inclusive {
                true => {
                    for num in first..=last {
                        &env.insert(
                            (scope, FunctionOrValueType::Value, i_name.clone()),
                            OranValue::Variable(OranVariable {
                                var_type: *var_type,
                                name: i,
                                value: OranVariableValue::Float(num as f64)
                            })
                        );
                        for stmt in stmts {
                            interp_expr(scope, env, stmt, FunctionOrValueType::Value);
                        }
                        //env.retain(|(_s, k, _label), _val| *k != FunctionOrValueType::Temp);
                    }
                }
                false => {
                    for num in first..last {
                        &env.insert(
                            (scope, FunctionOrValueType::Value, i_name.clone()),
                            OranValue::Variable(OranVariable {
                                var_type: *var_type,
                                name: i,
                                value: OranVariableValue::Float(num as f64)
                            })
                        );
                        for stmt in stmts {
                            interp_expr(scope, env, stmt, FunctionOrValueType::Value);
                        }
                        //env.retain(|(_s, k, _label), _val| *k != FunctionOrValueType::Temp);
                    }
                }
            }
            
            OranValue::Null
        }
        AstNode::Null => OranValue::Null,
        //_ => unreachable!("{:?}", reduced_expr)
    }
}

fn is_mutable<'a> (scope: usize, env : &HashMap<(usize, FunctionOrValueType, OranString<'a>), OranValue<'a>>, ident: &str, variable_type: &VarType) -> bool {
    let val = env.get(
        &(
            scope,
            FunctionOrValueType::Value,
            OranString::from(ident)
        )
    );
    match val {
        Some(v) => {
            if *variable_type == VarType::VariableReAssigned && OranVariable::from(v).var_type == VarType::Constant {
                panic!("You can't assign value twice to a constant variable.");
            }
        },
        None => {
            if *variable_type == VarType::VariableReAssigned {
                panic!("You can't assign value without 'let'.");
            }
        }
    }
    true
}
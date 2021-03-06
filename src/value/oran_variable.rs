use std::fmt;
use std::ops::{Add, Sub, Div, Mul, Rem};
use crate::value::var_type::VarType;
use super::oran_string::OranString;

#[derive(Clone, Debug)]
pub struct OranVariable<'a> {
    pub var_type: VarType,
    pub name: &'a str,
    pub value: OranVariableValue<'a>
}

impl PartialEq for OranVariable<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[derive(Debug)]
pub enum OranVariableValue<'a> {
    Float(f64),
    Str(OranString<'a>),
    Boolean(bool),
    Null
}

impl Clone for OranVariableValue<'_> {
    fn clone(&self) -> Self {
        match self {
            OranVariableValue::Float(a) => OranVariableValue::Float(*a),
            OranVariableValue::Str(a) => OranVariableValue::Str(a.clone()),
            OranVariableValue::Boolean(a) => OranVariableValue::Boolean(*a),
            OranVariableValue::Null => OranVariableValue::Null
        }
    }
}

impl fmt::Display for OranVariableValue<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OranVariableValue::Float(ref fl) => write!(f, "{}", fl),
            OranVariableValue::Str(ref s) => write!(f, "{}", s.val_str.as_ref()),
            OranVariableValue::Boolean(ref b) => write!(f, "{}", b),
            OranVariableValue::Null => write!(f, ""),
        }
    }
}

impl PartialEq for OranVariableValue<'_> {
    fn eq(&self, other: &OranVariableValue) -> bool {
        match *self {
            OranVariableValue::Float(ref fl) => *fl == f64::from(other),
            OranVariableValue::Str(ref s) => s.val_str.as_ref().to_string() == other.to_string(),
            OranVariableValue::Boolean(ref b) => bool::from(*b) == bool::from(other),
            OranVariableValue::Null => {
                match other {
                    OranVariableValue::Null => true,
                    _ => false
                }
            }
        }
    }
}

impl From<OranVariableValue<'_>> for bool {
    fn from(val: OranVariableValue) -> Self {
        match val {
            OranVariableValue::Str(ref s) => {
                if s.val_str.as_ref().to_string() == "true" {
                    return true;
                } else if s.val_str.as_ref().to_string() == "" {
                    return false;
                }
                true
            },
            OranVariableValue::Float(ref fl) => {
                if *fl == f64::from(0) {
                    return false;
                }
                true
            },
            OranVariableValue::Boolean(ref bl) => { *bl },
            OranVariableValue::Null => false,
        }
    }
}

impl From<&OranVariableValue<'_>> for bool {
    fn from(val: &OranVariableValue) -> Self {
        match val {
            OranVariableValue::Str(ref s) => {
                if s.val_str.as_ref().to_string() == "true" {
                    return true;
                } else if s.val_str.as_ref().to_string() == "" {
                    return false;
                }
                true
            },
            OranVariableValue::Float(ref fl) => {
                if *fl == f64::from(0) {
                    return false;
                }
                true
            },
            OranVariableValue::Boolean(ref bl) => { *bl },
            OranVariableValue::Null => false,
        }
    }
}

impl Sub for OranVariableValue<'_> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        match self {
            OranVariableValue::Float(ref fl) => { OranVariableValue::Float(fl - f64::from(other)) },
            _ => panic!("Variable types are not Number: {:?}", self)
        }
    }
}

impl Add for OranVariableValue<'_> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match self {
            OranVariableValue::Float(ref fl) => { OranVariableValue::Float(fl + f64::from(other)) },
            _ => panic!("Variable types are not Number: {:?}", self)
        }
    }
}

impl Div for OranVariableValue<'_> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        match self {
            OranVariableValue::Float(ref fl) => { OranVariableValue::Float(fl / f64::from(other)) },
            _ => panic!("Variable types are not Number: {:?}", self)
        }
    }
}

impl Mul for OranVariableValue<'_> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        match self {
            OranVariableValue::Float(ref fl) => { OranVariableValue::Float(fl * f64::from(other)) },
            _ => panic!("Variable types are not Number: {:?}", self)
        }
    }
}

impl Rem for OranVariableValue<'_> {
    type Output = Self;

    fn rem(self, other: Self) -> Self::Output {
        match self {
            OranVariableValue::Float(ref fl) => { OranVariableValue::Float(fl % f64::from(other)) },
            _ => panic!("Variable types are not Number: {:?}", self)
        }
    }
}

impl From<OranVariableValue<'_>> for f64 {
    fn from(val: OranVariableValue) -> Self {
        match val {
            OranVariableValue::Float(ref fl) => { *fl },
            OranVariableValue::Str(ref s) => s.val_str.as_ref().parse().unwrap_or_else(|e| panic!("{}", e)),
            OranVariableValue::Null => { f64::from(0) }
            _ => panic!("Variable types are not Number: {:?}", val)
        }
    }
}

impl From<OranVariableValue<'_>> for String {
    fn from(val: OranVariableValue) -> Self {
        match val {
            OranVariableValue::Str(ref s) => s.val_str.as_ref().to_string(),
            OranVariableValue::Float(ref fl) => { fl.to_string() },
            OranVariableValue::Boolean(ref bl) => { bl.to_string() },
            OranVariableValue::Null => { "".to_string() }
        }
    }
}

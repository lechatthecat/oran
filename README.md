# Oran
Programming language written by rust  
**Still in development**

# dependencies
You need llvm8.

# compatibility
tested in Ubuntu 18.04.3 LTS.

# run the project
You must specify where the llvm8 is by `LLVM_SYS_80_PREFIX=`.
```
$ git clone https://github.com/lechatthecat/oran.git
$ cd oran
$ LLVM_SYS_80_PREFIX=$HOME/llvm-8.0.0 cargo build
$ ./target/debug/oran
```

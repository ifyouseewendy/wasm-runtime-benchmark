Use the following command to compile src/fibonacci.rs to fibonacci.wasm

```
rustc +nightly --target wasm32-unknown-unknown --crate-type cdylib src/fibonacci.rs -o fibonacci.wasm
```

To make wasmer/LLVM backend working, first install LLVM https://gitlab.com/taricorp/llvm-sys.rs#compiling-llvm and
`export LLVM_SYS_80_PREFIX=~/llvm-8.0.0`

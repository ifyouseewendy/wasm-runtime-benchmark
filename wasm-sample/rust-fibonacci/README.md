```
$ rustc --target wasm32-unknown-unknown --crate-type cdylib src/lib.rs -o fibonacci.wasm
$ wasm-strip fibonacci.wasm
$ mv fibonacci.wasm ../
```

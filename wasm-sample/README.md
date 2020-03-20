### `add-one.wasm`

`add-one.wasm` is compiled by [add-one](./add-one)

```
$ cd add-one
$ npm install
$ npx asc assembly/index.ts -b add-one.wasm --validate --optimize
$ mv add-one.wasm ../
```

### `fibonacci.wasm`

`fibonacci.wasm` is compiled by [rust-fibonacci](./rust-fibonacci)

```
$ cd rust-fibonacci
$ rustc --target wasm32-unknown-unknown --crate-type cdylib src/lib.rs -o fibonacci.wasm
$ wasm-strip fibonacci.wasm
$ mv fibonacci.wasm ../
```

### `discount-script-mruby.wasm`

`discount-script-mruby.wasm` is compiled by
[artichoke](https://github.com/artichoke/artichoke), which doesn't support
`wasm32-unknown-unknown` yet. We have a hack for compiling a discount mruby
script. The wasm file is compiled
[here](https://github.com/ifyouseewendy/artichoke/tree/master/mruby-sys/vendor/mruby-bc7c5d3).
To be noted, we are sending a plain integer to this wasm in benchmark,
which won't function properly, but should serve the purpose for compiling and
executing a large (1.2M) and complicate wasm.

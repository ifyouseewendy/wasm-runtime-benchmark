#### `add-one.wasm`

source:

```js
export function run(a: i32): i32 {
  return a + 1;
}
```

compile:

```
$ cd add-one
$ npm install
$ npx asc assembly/index.ts -b add-one.wasm --validate --optimize --use abort= --runtime full
$ mv add-one.wasm ../
```

size: 2.6k


#### `fibonacci.wasm`

source:

```rust
fn run(n: u32) -> u32 {
    if n < 2 {
        1
    } else {
        run(n - 1) + run(n - 2)
    }
}
```

compile:

```
$ cd fibonacci/
$ rustc +nightly --target wasm32-unknown-unknown --crate-type cdylib src/lib.rs -o fibonacci.wasm
$ wasm-strip fibonacci.wasm
$ mv fibonacci.wasm ../
```

size: 16k

#### `nbody.wasm`

source: https://github.com/wasmerio/wasmer-bench/blob/master/benchmarks/src/nbody.rs

compile:

```
$ cd nbody/
$ rustc +nightly --target wasm32-unknown-unknown --crate-type cdylib src/lib.rs -o nbody.wasm
$ wasm-strip nbody.wasm
$ mv nbody.wasm ../
```

size: 9.3k

#### `mruby-script.wasm`

source:
[entry_discount.c](https://github.com/ifyouseewendy/artichoke/blob/master/mruby-sys/vendor/mruby-bc7c5d3/entry_discount.c)

compile:

It is compiled by
[artichoke](https://github.com/artichoke/artichoke), which doesn't support
`wasm32-unknown-unknown` yet. We have a hack for compiling a discount mruby
script. The wasm file is compiled
[here](https://github.com/ifyouseewendy/artichoke/tree/master/mruby-sys/vendor/mruby-bc7c5d3).
To be noted, we are sending a plain integer to this wasm in benchmark,
which won't function properly, but should serve the purpose for compiling and
executing a large and complicate wasm.

size: 1.2M

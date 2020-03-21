1. `add-one.wasm`

source:

```js
export function run(a: i32): i32 {
  return a + 1;
}
```

size: 2.6k


2. `fibonacci.wasm`

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

size: 16k

3. `mruby-script.wasm`

source:
[entry_discount.c](https://github.com/ifyouseewendy/artichoke/blob/master/mruby-sys/vendor/mruby-bc7c5d3/entry_discount.c)

It is compiled by
[artichoke](https://github.com/artichoke/artichoke), which doesn't support
`wasm32-unknown-unknown` yet. We have a hack for compiling a discount mruby
script. The wasm file is compiled
[here](https://github.com/ifyouseewendy/artichoke/tree/master/mruby-sys/vendor/mruby-bc7c5d3).
To be noted, we are sending a plain integer to this wasm in benchmark,
which won't function properly, but should serve the purpose for compiling and
executing a large and complicate wasm.

size: 1.2M

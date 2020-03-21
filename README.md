## Context

### What happens when a runtime executes a WebAseembly file?

> When a user executes a WebAssembly file with Wasmer, the following happens:
>
> * Parsing: Wasmer parses the wasm file, assuring its validity according to the
>   spec.
> * Function compilation: the function bodies defined in WebAssembly bytecode are
>   compiled to machine code via a compiler framework (Cranelift, LLVM, …), so
>   they can be executed at native speed. This process generates a Module object.
> * Instantiation: at this step, we create the memories (where data lives) and
>   tables (where functions pointers are stored) that the
>   WebAssembly Instance will use. Once we have everything in place, we call the
>   start function in the instance (usually that’s the main function in C, C++ or
>   Rust)

from [Running WebAssembly 100x faster - Wasmer](https://medium.com/wasmer/running-webassembly-100x-faster-%EF%B8%8F-a8237e9a372d)

We can generally consider a WebAssembly runtime does a four step job:

- a. parsing
- b. compilation
- c. instantiation
- d. execution

**When we benchmark a runtime, we want to know how long each step takes.** For
compilation, there is also a space concern.

### What about compilation/execution modes?

Different runtimes have different designs based on the targeted use cases, eg.
Wasmer lets you exchange the compiler backend and Lucet is designed for
ahead-of-time compilation.

When we benchmark a **JIT** (or interpretation) runtime, it is about the performance of `a+b+c+d`.

When we benchmark a AOT runtime, given it splits the whole process into two separate stages (compile and execute), an overhead of handling the compiled object is introduced:

* **AOT compile**: `a+b+b'` in which `b'` means saving the compiled object into a
  storage (or cache)
* **AOT execute**: `c'+c+d` in which `c'` means loading the compiled object from a
  storage (or cache)

In addition, when we compare the performance with native, we are specifically looking at step
`d`, **pure execution**.

## Benchmark Plan

### Individual

For one WebAssembly runtime, we want to measure the performance of

- JIT: `a+b+c+d`
- AOT compile: `a+b+b'` (also measuring space for `b'`)
- AOT execute: `c'+c+d`
- pure execution: `d`

To be noted, given AOT is trading space for speed compared to JIT, we may want
to know the overhead of using an AOT solution.

### Comparison

For a bunch of WebAssembly runtimes, in addition to comparing the above metrics, we should also consider

- performance with samples of different size and complexity

- performance with addition features, like WASI

## Implementation (WIP)

### Generate samples

We should prepare a few wasm samples of different size and complication. You
can check [wasm-sample](./wasm-sample/) for how to compile the sample programs
into wasm.

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
$ npx asc assembly/index.ts -b add-one.wasm --validate --optimize
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
$ rustc +nightly --target wasm32-unknown-unknown --crate-type cdylib src/fibonacci.rs -o fibonacci.wasm
$ wasm-strip fibonacci.wasm
$ mv fibonacci.wasm ../
```

size: 16k

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

### Run benchmark

To enable LLVM backend for Wasmer, follow https://gitlab.com/taricorp/llvm-sys.rs#compiling-llvm to install LLVM and
`export LLVM_SYS_80_PREFIX=YOUR_PATH_TO_LLVM_DIR`

Run by

```
$ cargo bench
```

Check result

```
$ open target/criterion/report/index.html
```

TODO

- [ ] Add WAVM, in addition to Wasmer and Lucet.
- [ ] Bench WASI

## Report

### Individual - Wasmer/Singlepass

```
fibonacci/wasmer-singlepass/ab-compile      time:   [7.5588 ms 7.6799 ms 7.8605 ms]
fibonacci/wasmer-singlepass/c-instantiate   time:   [18.345 us 18.656 us 19.149 us]
fibonacci/wasmer-singlepass/d-call          time:   [6.2023 us 6.2977 us 6.3575 us]
```

* Parsing happens in compilation

### Individual - Wasmer/Cranelift

```
fibonacci/wasmer-cranelift/ab-compile       time:   [23.255 ms 23.894 ms 24.275 ms]
fibonacci/wasmer-cranelift/c-instantiate    time:   [18.794 us 19.590 us 20.477 us]
fibonacci/wasmer-cranelift/d-call           time:   [2.2077 us 2.2318 us 2.2778 us]
```

### Individual - Wasmer/LLVM

```
fibonacci/wasmer-llvm/ab-compile            time:   [8.9680 s 9.0965 s 9.3370 s]
fibonacci/wasmer-llvm/c-instantiate         time:   [34.642 us 35.391 us 36.364 us]
fibonacci/wasmer-llvm/d-call                time:   [1.5052 us 1.5275 us 1.5430 us]
```

* The data point is gathered by waiting more than 10 min. I suspect there is something weird happening. I've reported to WASMER team.
* The performance of `call` is the best so far, but the `compile` time is too slow to be accepted

### Individual - Lucet

```
fibonacci/lucet/ab-compile      time:   [102.94 ms 114.69 ms 124.16 ms]
fibonacci/lucet/c-instantiate   time:   [167.15 us 174.82 us 185.01 us]
fibonacci/lucet/d-call          time:   [10.260 us 10.361 us 10.513 us]
```

### Comparison - JIT

> a+b+c+d

```
fibonacci/wasmer-singlepass   time:   [7.7525 ms 8.3134 ms 9.3078 ms]
fibonacci/wasmer-cranelift    time:   [32.647 ms 34.320 ms 35.592 ms]
fibonacci/wasmer-llvm         time:   [8.9640 s  9.0053 s  9.0913 s ]
```

* Lucet doens't support or design for JIT. We can use its AOT total performance as a comparison

### Comparison - AOT

#### AOT total

> a+b+b'+c'+c+d

```
fibonacci/wasmer-singlepass   time:   [28.142 ms 29.992 ms 31.050 ms]
fibonacci/wasmer-cranelift    time:   [53.070 ms 59.347 ms 68.896 ms]
fibonacci/wasmer-llvm         time:   [9.4447 s  9.6621 s  9.8105 s ]
fibonacci/lucet               time:   [107.60 ms 110.18 ms 113.12 ms]
```

* By comparing AOT total with JIT, we can see overhead introduced by `b'+c'` for Singlepass and Cranelift case.

#### AOT compile (time)

> a+b+b'

```
fibonacci/wasmer-singlepass   time:   [21.753 ms 22.970 ms 24.043 ms]
fibonacci/wasmer-cranelift    time:   [44.587 ms 47.660 ms 49.897 ms]
fibonacci/wasmer-llvm         time:   [11.878 s  12.099 s  12.393 s ]
fibonacci/lucet               time:   [104.69 ms 106.19 ms 108.53 ms]
```

#### AOT compile (space)

> b'

```
fibonacci/wasmer-singlepass   size: 2.2M
fibonacci/wasmer-cranelift    size: 1.9M
fibonacci/wasmer-llvm         size: 1.8M
fibonacci/lucet               size: 92K
```

With stripped wasm (`wasm-strip fibonacci.wasm`), removing the unnecessary custom data
section and reducing the file size from 1.8M to 16K

```
fibonacci/wasmer-singlepass   size: 362k
fibonacci/wasmer-cranelift    size: 98k
fibonacci/wasmer-llvm         size: 43k
fibonacci/lucet               size: 86K
```

#### AOT execution

> c'+c+d

```
fibonacci/wasmer-singlepass   time:   [8.0749 ms 8.2386 ms 8.3627 ms]
fibonacci/wasmer-cranelift    time:   [6.1068 ms 6.2310 ms 6.4169 ms]
fibonacci/wasmer-llvm         time:   [5.9600 ms 6.0994 ms 6.2290 ms]
fibonacci/lucet               time:   [197.21 us 201.77 us 206.27 us]
```

* `c'` here means a DLopen for lucet and loading from file cache for Wasmer.
* Lucet runs amazingly fast, even this has dynamic linking included. I wonder
  if there is any compiler optimization kicks in.
- [ ] Double check lucet number

### Comparison - Pure execution

> d

```
fibonacci/rust-native         time:   [181.79 ns 192.94 ns 203.50 ns]
fibonacci/wasmer-singlepass   time:   [5.9822 us 6.0379 us 6.1314 us]
fibonacci/wasmer-cranelift    time:   [2.3717 us 2.4430 us 2.5936 us]
fibonacci/wasmer-llvm         time:   [1.5595 us 1.5826 us 1.6181 us]
fibonacci/lucet               time:   [10.748 us 10.884 us 10.999 us]
```

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

## Implementation

### Generate samples

We should prepare a few wasm samples of different size and complication. You
can check [wasm-sample](./wasm-sample/) for how to compile the sample programs into wasm.

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

It is compiled by [artichoke](https://github.com/artichoke/artichoke), which doesn't support `wasm32-unknown-unknown` yet. We have a hack for compiling a discount mruby script. The wasm file is compiled [here](https://github.com/ifyouseewendy/artichoke/tree/master/mruby-sys/vendor/mruby-bc7c5d3). To be noted, we are sending a plain integer to this wasm in benchmark,
which won't function properly, but should serve the purpose for compiling and
executing a large and complicate wasm.

size: 1.2M

### Run benchmark

Note:

* To enable LLVM backend for Wasmer, follow https://gitlab.com/taricorp/llvm-sys.rs#compiling-llvm to install LLVM and
  `export LLVM_SYS_80_PREFIX=YOUR_PATH_TO_LLVM_DIR`
* Benchmark with LLVM involved usually takes >10 mins
* Configure `criterion_group!` in [benches/my_benchmark.rs](./benches/my_benchmark.rs) to run benchmark selectively
* Create a `tmp` and `tmp/lucet` for the AOT cases.

Run

```
$ cargo bench
```

Check result at STDOUT or

```
$ open target/criterion/report/index.html
```

TODO

- [ ] Add WAVM, in addition to Wasmer and Lucet.
- [ ] Bench WASI

## Report

### Individual - Wasmer/Singlepass

|           | ab. compile | c. instantiate | d. execute |
| --------- | ----------- | -------------- | ---------- |
| add-one   | 907.89 us   | 11.880 us      | 1.5622 us  |
| nobody    | 3.7359 ms   | 15.936 us      | 57.256 us  |
| fibonacci | 6.7746 ms   | 18.522 us      | 6.8601 us  |

Parsing happens in compilation

### Individual - Wasmer/Cranelift

|           | ab. compile | c. instantiate | d. execute |
| --------- | ----------- | -------------- | ---------- |
| add-one   | 2.7364 ms   | 11.717 us      | 764.69 ns  |
| nobody    | 8.1322 ms   | 15.696 us      | 25.079 us  |
| fibonacci | 16.133 ms   | 17.718 us      | 2.4047 us  |

### Individual - Wasmer/LLVM

```
fibonacci/wasmer-llvm/ab-compile            time:   [8.9680 s 9.0965 s 9.3370 s]
fibonacci/wasmer-llvm/c-instantiate         time:   [34.642 us 35.391 us 36.364 us]
fibonacci/wasmer-llvm/d-call                time:   [1.5052 us 1.5275 us 1.5430 us]
```

|           | ab. compile | c. instantiate | d. execute |
| --------- | ----------- | -------------- | ---------- |
| add-one   | 1.1674 s    | 17.606 us      | 780.84 ns  |
| nobody    | 5.8321 s    | 35.037 us      | 13.068 us  |
| fibonacci | 9.1097 s    | 35.125 us      | 1.9549 us  |

The performance of `execute` is the best so far, but the `compile` time is too slow to be accepted

### Individual - Lucet

|           | ab. compile | c. instantiate | d. execute |
| --------- | ----------- | -------------- | ---------- |
| add-one   | 19.614 ms   | 160.52 us      | 9.9157 us  |
| nobody    | 52.497 ms   | 156.13 us      | 27.367 us  |
| fibonacci | 101.63 ms   | 157.93 us      | 11.104 us  |

### Comparison - JIT

> a+b+c+d

|           | wasmer/singlepass | wasmer/cranelift | wasmer/llvm | lucet |
| --------- | ----------------- | ---------------- | ----------- | ----- |
| add-one   | 1.1253 ms         | 3.4631 ms        | 1.2624 s    | NA    |
| nobody    | 4.8221 ms         | 10.350 ms        | 5.9734 s    | NA    |
| fibonacci | 7.2267 ms         | 19.296 ms        | 10.183 s    | NA    |

Lucet doens't support or is not designed for JIT

### Comparison - AOT

#### AOT total

> a+b+b'+c'+c+d

|           | wasmer/singlepass | wasmer/cranelift | wasmer/llvm | lucet     |
| --------- | ----------------- | ---------------- | ----------- | --------- |
| add-one   | 1.7526 ms         | 4.0685 ms        | 1.1734 s    | 18.195 ms |
| nobody    | 6.4503 ms         | 11.218 ms        | 5.2861 s    | 49.371 ms |
| fibonacci | 11.781 ms         | 19.107 ms        | 9.9057 s    | 104.42 ms |

By comparing AOT total with JIT, we can see the overhead introduced by `b'+c'` 

#### AOT compile (time)

> a+b+b'

|              | wasmer/singlepass | wasmer/cranelift | wasmer/llvm | lucet     |
| ------------ | ----------------- | ---------------- | ----------- | --------- |
| add-one      | 1.0323 ms         | 3.0077 ms        | 1.1484 s    | 16.965 ms |
| nobody       | 5.4153 ms         | 8.9157 ms        | 5.2498 s    | 47.605 ms |
| fibonacci    | 8.7047 ms         | 19.099 ms        | 9.6198 s    | 102.75 ms |
| mruby-script | 561.88 ms         | ~38.57 s         | ~34.24 s    | ~34.46 s  |

`~` means an estimation based on the bench log output. The actual bench program is not finished.

#### AOT compile (space)

> b'

The size of intermediate files different runtimes compile to, which is configured to be in `tmp/`. The following numbers are generated by clean the `tmp/` (You need to create an empty `tmp/lucet` for lucet cache), run the bench, and check the cached file size in `tmp/`. 

|              | source | wasmer/singlepass | wasmer/cranelift | wasmer/llvm | lucet |
| ------------ | ------ | ----------------- | ---------------- | ----------- | ----- |
| add-one      | 2.1 K  | 41 K              | 18 K             | 13 K        | 21 K  |
| nobody       | 9.3 K  | 222 K             | 62 K             | 30 K        | 58 K  |
| fibonacci    | 16 K   | 362 K             | 98 K             | 43 K        | 86 K  |
| mruby-script | 1.2 M  | 24 M              | /                | /           | /     |

#### AOT execution

> c'+c+d

|           | wasmer/singlepass | wasmer/cranelift | wasmer/llvm | lucet     |
| --------- | ----------------- | ---------------- | ----------- | --------- |
| add-one   | 244.24 us         | 89.810 us        | 904.55 us   | 175.81 us |
| nobody    | 1.2892 ms         | 202.61 us        | 2.1254 ms   | 205.35 us |
| fibonacci | 1.9484 ms         | 221.65 us        | 2.0697 ms   | 194.29 us |

### Comparison - Pure execution

> d

|           | native    | wasmer/singlepass | wasmer/cranelift | wasmer/llvm | lucet     |
| --------- | --------- | ----------------- | ---------------- | ----------- | --------- |
| add-one   | 702.20 ps | 1.4755 us         | 754.35 ns        | 757.57 ns   | 9.8288 us |
| fibonacci | 950.04 ns | 5.9847 us         | 2.1972 us        | 1.5783 us   | 10.981 us |
| nobody    | 950.85 ns | 54.357 us         | 24.906 us        | 12.975 us   | 28.883 us |


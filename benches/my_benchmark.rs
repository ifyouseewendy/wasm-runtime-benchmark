use criterion::*;
use lazy_static::lazy_static;
use rust_fibonacci as fibonacci;
use wasm_runtime_benchmark::{lucet_runner, wasmer_runner};

use std::collections::HashMap;

lazy_static! {
    static ref SAMPLES: HashMap<&'static str, &'static [u8]> = {
        let mut map = HashMap::new();
        map.insert(
            "fibonacci",
            &include_bytes!("../wasm-sample/fibonacci.wasm")[..],
        );
        map.insert(
            "mruby-script",
            &include_bytes!("../wasm-sample/discount-script-mruby.wasm")[..],
        );
        map
    };
}

static WASM: &'static [u8] = include_bytes!("../wasm-sample/fibonacci.wasm");

fn jit(c: &mut Criterion) {
    for (name, wasm) in SAMPLES.iter() {
        let mut group = c.benchmark_group("jit");

        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-singlepass"),
            wasm,
            |b, &wasm| {
                b.iter(|| {
                    let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Singlepass);
                    wrapper.jit(&wasm, black_box(10))
                });
            },
        );
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-cranelift"),
            wasm,
            |b, &wasm| {
                b.iter(|| {
                    let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
                    wrapper.jit(&wasm, black_box(10))
                });
            },
        );
        // group.sample_size(10).bench_with_input(
        //     BenchmarkId::new(name.to_owned(), "wasmer-llvm"),
        //     wasm,
        //     |b, &wasm| {
        //         b.iter(|| {
        //             let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
        //             wrapper.jit(&wasm, black_box(10))
        //         });
        //     },
        // );

        group.finish();
    }
}

fn aot_c(c: &mut Criterion) {
    let benchmark = Benchmark::new("rust-native", |b| b.iter(|| black_box(fibonacci::run(10))))
        .sample_size(10)
        .with_function("wasmer-singlepass", |b| {
            let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Singlepass);
            b.iter(|| black_box(wrapper.aot_c(&WASM).unwrap()))
        })
        .with_function("wasmer-cranelift", |b| {
            let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
            b.iter(|| black_box(wrapper.aot_c(&WASM).unwrap()))
        })
        .with_function("wasmer-llvm", |b| {
            let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
            b.iter(|| black_box(wrapper.aot_c(&WASM)))
        })
        .with_function("lucet", |b| {
            b.iter(|| black_box(lucet_runner::aot_c(&WASM)))
        });

    c.bench("fibonacci-aot-c", benchmark);
}

fn aot_e(c: &mut Criterion) {
    let benchmark = Benchmark::new("rust-native", |b| b.iter(|| black_box(fibonacci::run(10))))
        .sample_size(10)
        .with_function("wasmer-singlepass", |b| {
            let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Singlepass);
            let key = wrapper.aot_c(&WASM).unwrap();
            b.iter(|| wrapper.aot_e(&key, black_box(10)))
        })
        .with_function("wasmer-cranelift", |b| {
            let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
            let key = wrapper.aot_c(&WASM).unwrap();
            b.iter(|| wrapper.aot_e(&key, black_box(10)))
        })
        .with_function("lucet", |b| {
            let moduleid = lucet_runner::aot_c(&WASM);
            b.iter(|| lucet_runner::aot_e(&moduleid, black_box(10)))
        })
        .with_function("wasmer-llvm", |b| {
            let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
            let key = wrapper.aot_c(&WASM).unwrap();
            b.iter(|| wrapper.aot_e(&key, black_box(10)))
        });

    c.bench("fibonacci-aot-e", benchmark);
}

fn aot_t(c: &mut Criterion) {
    let benchmark = Benchmark::new("rust-native", |b| b.iter(|| black_box(fibonacci::run(10))))
        .sample_size(10)
        .with_function("wasmer-singlepass", |b| {
            b.iter(|| {
                let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Singlepass);
                wrapper.aot_t(&WASM, black_box(10))
            })
        })
        .with_function("wasmer-cranelift", |b| {
            b.iter(|| {
                let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
                wrapper.aot_t(&WASM, black_box(10))
            })
        })
        .with_function("wasmer-llvm", |b| {
            b.iter(|| {
                let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
                wrapper.aot_t(&WASM, black_box(10))
            })
        })
        .with_function("lucet", |b| {
            b.iter(|| lucet_runner::aot_t(&WASM, black_box(10)))
        });

    c.bench("fibonacci-aot", benchmark);
}

fn call(c: &mut Criterion) {
    let benchmark = Benchmark::new("rust-native", |b| b.iter(|| black_box(fibonacci::run(10))))
        .sample_size(10)
        .with_function("wasmer-singlepass", |b| {
            let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Singlepass);
            let instance = wrapper.prepare(&WASM).unwrap();
            b.iter(|| wrapper.call(&instance, black_box(10)))
        })
        .with_function("wasmer-cranelift", |b| {
            let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
            let instance = wrapper.prepare(&WASM).unwrap();
            b.iter(|| wrapper.call(&instance, black_box(10)))
        })
        .with_function("wasmer-llvm", |b| {
            let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
            let instance = wrapper.prepare(&WASM).unwrap();
            b.iter(|| wrapper.call(&instance, black_box(10)))
        })
        .with_function("lucet", |b| {
            let mut instance = lucet_runner::prepare(&WASM);
            b.iter(|| lucet_runner::call(&mut instance, 10))
        });

    c.bench("fibonacci-call", benchmark);
}

fn wasmer_singlepass(c: &mut Criterion) {
    let benchmark = Benchmark::new("compile", |b| {
        let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Singlepass);
        b.iter(|| black_box(wrapper.compile(&WASM)))
    })
    .sample_size(10)
    .with_function("instantiate", |b| {
        let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Singlepass);
        let module = wrapper.compile(&WASM);
        b.iter(|| black_box(wrapper.instantiate(&module)))
    })
    .with_function("call", |b| {
        let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Singlepass);
        let module = wrapper.compile(&WASM);
        let instance = wrapper.instantiate(&module).unwrap();
        b.iter(|| black_box(wrapper.call(&instance, 10)))
    });

    c.bench("fibonacci/wasmer-singlepass", benchmark);
}

fn wasmer_cranelift(c: &mut Criterion) {
    let benchmark = Benchmark::new("compile", |b| {
        let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
        b.iter(|| black_box(wrapper.compile(&WASM)))
    })
    .sample_size(10)
    .with_function("instantiate", |b| {
        let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
        let module = wrapper.compile(&WASM);
        b.iter(|| black_box(wrapper.instantiate(&module)))
    })
    .with_function("call", |b| {
        let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
        let module = wrapper.compile(&WASM);
        let instance = wrapper.instantiate(&module).unwrap();
        b.iter(|| black_box(wrapper.call(&instance, 10)))
    });

    c.bench("fibonacci/wasmer-cranelift", benchmark);
}

fn wasmer_llvm(c: &mut Criterion) {
    let benchmark = Benchmark::new("compile", |b| {
        let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
        b.iter(|| black_box(wrapper.compile(&WASM)))
    })
    .sample_size(10)
    .with_function("instantiate", |b| {
        let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
        let module = wrapper.compile(&WASM);
        b.iter(|| black_box(wrapper.instantiate(&module)))
    })
    .with_function("call", |b| {
        let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
        let module = wrapper.compile(&WASM);
        let instance = wrapper.instantiate(&module).unwrap();
        b.iter(|| black_box(wrapper.call(&instance, 10)))
    });

    c.bench("fibonacci/wasmer-llvm", benchmark);
}

fn lucet(c: &mut Criterion) {
    let benchmark = Benchmark::new("compile", |b| {
        b.iter(|| black_box(lucet_runner::compile(&WASM)))
    })
    .sample_size(10)
    .with_function("instantiate", |b| {
        let moduleid = lucet_runner::compile(&WASM);
        b.iter(|| black_box(lucet_runner::instantiate(&moduleid)))
    })
    .with_function("call", |b| {
        let moduleid = lucet_runner::compile(&WASM);
        let mut instance = lucet_runner::instantiate(&moduleid);
        b.iter(|| black_box(lucet_runner::call(&mut instance, 10)))
    });

    c.bench("fibonacci/lucet", benchmark);
}

criterion_group!(benches, jit);
criterion_main!(benches);

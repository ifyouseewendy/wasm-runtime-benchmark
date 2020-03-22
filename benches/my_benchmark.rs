use criterion::*;
use fibonacci;
use lazy_static::lazy_static;
use nbody;
use wasm_runtime_benchmark::{lucet_runner, wasmer_runner};

use std::collections::HashMap;

lazy_static! {
    static ref SAMPLES: HashMap<&'static str, &'static [u8]> = {
        let mut map = HashMap::new();
        map.insert(
            "add-one",
            &include_bytes!("../wasm-sample/add-one.wasm")[..],
        );
        map.insert(
            "fibonacci",
            &include_bytes!("../wasm-sample/fibonacci.wasm")[..],
        );
        map.insert(
            "nbody",
            &include_bytes!("../wasm-sample/nbody.wasm")[..],
        );
        // Too slow to compile
        // map.insert(
        //     "mruby-script",
        //     &include_bytes!("../wasm-sample/mruby-script.wasm")[..],
        // );
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
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-llvm"),
            wasm,
            |b, &wasm| {
                b.iter(|| {
                    let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
                    wrapper.jit(&wasm, black_box(10))
                });
            },
        );

        group.finish();
    }
}

fn aot_compile(c: &mut Criterion) {
    for (name, wasm) in SAMPLES.iter() {
        let mut group = c.benchmark_group("aot_compile");

        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-singlepass"),
            wasm,
            |b, &wasm| {
                let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Singlepass);
                b.iter(|| black_box(wrapper.aot_c(&wasm).unwrap()))
            },
        );
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-cranelift"),
            wasm,
            |b, &wasm| {
                let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
                b.iter(|| black_box(wrapper.aot_c(&wasm).unwrap()))
            },
        );
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-llvm"),
            wasm,
            |b, &wasm| {
                let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
                b.iter(|| black_box(wrapper.aot_c(&wasm)))
            },
        );
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "lucet"),
            wasm,
            |b, &wasm| b.iter(|| black_box(lucet_runner::aot_c(&wasm))),
        );

        group.finish();
    }
}

fn aot_execute(c: &mut Criterion) {
    for (name, wasm) in SAMPLES.iter() {
        let mut group = c.benchmark_group("aot_execute");

        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-singlepass"),
            wasm,
            |b, &wasm| {
                let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Singlepass);
                let key = wrapper.aot_c(&wasm).unwrap();
                b.iter(|| wrapper.aot_e(&key, black_box(10)))
            },
        );
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-cranelift"),
            wasm,
            |b, &wasm| {
                let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
                let key = wrapper.aot_c(&wasm).unwrap();
                b.iter(|| wrapper.aot_e(&key, black_box(10)))
            },
        );
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-llvm"),
            wasm,
            |b, &wasm| {
                let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
                let key = wrapper.aot_c(&wasm).unwrap();
                b.iter(|| wrapper.aot_e(&key, black_box(10)))
            },
        );
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "lucet"),
            wasm,
            |b, &wasm| {
                let moduleid = lucet_runner::aot_c(&wasm);
                b.iter(|| lucet_runner::aot_e(&moduleid, black_box(10)))
            },
        );

        group.finish();
    }
}

fn aot_total(c: &mut Criterion) {
    for (name, wasm) in SAMPLES.iter() {
        let mut group = c.benchmark_group("aot_total");

        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-singlepass"),
            wasm,
            |b, &wasm| {
                b.iter(|| {
                    let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Singlepass);
                    wrapper.aot_t(&wasm, black_box(10))
                })
            },
        );
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-cranelift"),
            wasm,
            |b, &wasm| {
                b.iter(|| {
                    let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
                    wrapper.aot_t(&wasm, black_box(10))
                })
            },
        );
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-llvm"),
            wasm,
            |b, &wasm| {
                b.iter(|| {
                    let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
                    wrapper.aot_t(&wasm, black_box(10))
                })
            },
        );
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "lucet"),
            wasm,
            |b, &wasm| b.iter(|| lucet_runner::aot_t(&wasm, black_box(10))),
        );

        group.finish();
    }
}

fn execute(c: &mut Criterion) {
    for (name, wasm) in SAMPLES.iter() {
        let mut group = c.benchmark_group("execute");

        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "rust-native"),
            wasm,
            |b, &wasm| match name {
                &"add-one" => {
                    fn add_one(n: u32) -> u32 {
                        n + 1
                    }
                    b.iter(|| black_box(add_one(10)))
                }
                &"fibonacci" => b.iter(|| black_box(fibonacci::run(10))),
                &"nbody" => b.iter(|| unsafe { black_box(nbody::run(10)) }),
                _ => unreachable!(),
            },
        );
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-singlepass"),
            wasm,
            |b, &wasm| {
                let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Singlepass);
                let instance = wrapper.prepare(&wasm).unwrap();
                b.iter(|| wrapper.call(&instance, black_box(10)))
            },
        );
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-cranelift"),
            wasm,
            |b, &wasm| {
                let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
                let instance = wrapper.prepare(&wasm).unwrap();
                b.iter(|| wrapper.call(&instance, black_box(10)))
            },
        );
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "wasmer-llvm"),
            wasm,
            |b, &wasm| {
                let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
                let instance = wrapper.prepare(&wasm).unwrap();
                b.iter(|| wrapper.call(&instance, black_box(10)))
            },
        );
        group.sample_size(10).bench_with_input(
            BenchmarkId::new(name.to_owned(), "lucet"),
            wasm,
            |b, &wasm| {
                let mut instance = lucet_runner::prepare(&wasm);
                b.iter(|| lucet_runner::call(&mut instance, 10))
            },
        );

        group.finish();
    }
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

criterion_group!(
    benches, // jit,
    // aot_compile,
    // aot_execute,
    aot_total,
    // execute
);
criterion_main!(benches);

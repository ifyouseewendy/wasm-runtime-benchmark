// use criterion::{black_box, criterion_group, criterion_main, Benchmark, Criterion};
use criterion::*;
use wasm_runtime_benchmark::{fibonacci, wasmer_runner};

static WASM: &'static [u8] = include_bytes!("../fibonacci.wasm");

fn jit(c: &mut Criterion) {
    let benchmark = Benchmark::new("rust-native", |b| b.iter(|| black_box(fibonacci::run(10))))
        .sample_size(10)
        .with_function("wasmer-singlepass", |b| {
            b.iter(|| {
                let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Singlepass);
                wrapper.jit(&WASM, black_box(10))
            })
        })
        .with_function("wasmer-cranelift", |b| {
            b.iter(|| {
                let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
                wrapper.jit(&WASM, black_box(10))
            })
        });
    // Too slow to finish
    // .with_function("wasmer-llvm", |b| {
    //     b.iter(|| {
    //         let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
    //         wrapper.jit(&WASM, black_box(10))
    //     })
    // });

    c.bench("fibonacci", benchmark);
}

fn aot(c: &mut Criterion) {
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
        });
    // Too slow to finish
    // .with_function("wasmer-llvm", |b| {
    //     b.iter(|| {
    //         let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
    //         wrapper.aot_t(&WASM, black_box(10))
    //     })
    // });

    c.bench("fibonacci", benchmark);
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
        });

    c.bench("fibonacci", benchmark);
}

criterion_group!(benches, jit, aot, call);
criterion_main!(benches);

// use criterion::{black_box, criterion_group, criterion_main, Benchmark, Criterion};
use criterion::*;
use wasm_runtime_benchmark::{fibonacci, lucet_runner, wasmer_runner};

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

    c.bench("fibonacci-jit", benchmark);
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
        });
    // Too slow to finish
    // .with_function("wasmer-llvm", |b| {
    //		let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::Cranelift);
    // 		let key = wrapper.aot_c(&WASM).unwrap();
    // 		b.iter(|| {
    // 		    wrapper.aot_e(&key, black_box(10))
    // 		})
    // });

    c.bench("fibonacci-aot-e", benchmark);
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
        })
        .with_function("lucet", |b| {
            b.iter(|| lucet_runner::aot_t(&WASM, black_box(10)))
        });
    // Too slow to finish
    // .with_function("wasmer-llvm", |b| {
    //     b.iter(|| {
    //         let wrapper = wasmer_runner::Wrapper::new(wasmer_runtime::Backend::LLVM);
    //         wrapper.aot_t(&WASM, black_box(10))
    //     })
    // });

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

criterion_group!(benches, aot, aot_e, call);
criterion_main!(benches);

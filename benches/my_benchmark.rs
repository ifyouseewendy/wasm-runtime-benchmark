use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wasm_runtime_benchmark::wasmer_runner;

fn wasmer_jit_with_singlepass(c: &mut Criterion) {
    c.bench_function("jit with singlepass", |b| {
        b.iter(|| wasmer_runner::jit_with_singlepass("./fibonacci.wasm", black_box(20)))
    });
}
fn wasmer_jit_with_cranelift(c: &mut Criterion) {
    c.bench_function("jit with cranelift", |b| {
        b.iter(|| wasmer_runner::jit_with_cranelift("./fibonacci.wasm", black_box(20)))
    });
}
fn wasmer_aot_with_singlepass(c: &mut Criterion) {
    let key = wasmer_runner::store_with_singlepass("./fibonacci.wasm").unwrap();

    c.bench_function("aot with singlepass", |b| {
        b.iter(|| wasmer_runner::aot_with_singlepass(&key, black_box(20)))
    });
}
fn wasmer_aot_with_cranelift(c: &mut Criterion) {
    let key = wasmer_runner::store_with_cranelift("./fibonacci.wasm").unwrap();

    c.bench_function("aot with cranelift", |b| {
        b.iter(|| wasmer_runner::aot_with_cranelift(&key, black_box(20)))
    });
}
fn wasmer_aot_with_llvm(c: &mut Criterion) {
    let key = wasmer_runner::store_with_llvm("./fibonacci.wasm").unwrap();

    c.bench_function("aot with llvm", |b| {
        b.iter(|| wasmer_runner::aot_with_llvm(&key, black_box(20)))
    });
}

criterion_group!(
    benches,
    wasmer_aot_with_cranelift,
    wasmer_aot_with_singlepass,
    wasmer_aot_with_llvm // wasmer_jit_with_cranelift,
                         // wasmer_jit_with_singlepass
);
criterion_main!(benches);

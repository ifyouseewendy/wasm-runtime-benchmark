#![feature(test)]
extern crate test;
use test::Bencher;

mod fibonacci;
mod wasmer_runner;

#[bench]
fn native(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(10);
        fibonacci::run(n)
    })
}

#[bench]
fn wasmer_jit_with_singlepass(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(3);
        wasmer_runner::jit_with_singlepass("./fibonacci.wasm", n)
    })
}
#[bench]
fn wasmer_jit_with_cranelift(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(3);
        wasmer_runner::jit_with_cranelift("./fibonacci.wasm", n)
    })
}
#[bench]
fn wasmer_jit_with_llvm(b: &mut Bencher) {
    b.iter(|| {
        let n = test::black_box(3);
        wasmer_runner::jit_with_llvm("./fibonacci.wasm", n)
    })
}

// #[bench]
// fn wasmer_llvm(b: &mut Bencher) {
//     b.iter(|| {
//         let n = test::black_box(10);
//         wasmer_runner::with_llvm(n)
//     })
// }
//
// #[bench]
// fn wasmer_aot_with_llvm(b: &mut Bencher) {
//     let key = wasmer_runner::store_module().unwrap();
//     b.iter(|| {
//         let n = test::black_box(10);
//         wasmer_runner::aot_with_llvm(n, &key)
//     })
// }

// AOT bench
// bench on native: by running iterative_fibonacci() directly
// bench on lucet: by using build.rs to compile a fibonacci.rs -> wasm -> so and run it via lucet
// bench on wasmer: by using build.rs to compile a fibonacci.rs -> wasm -> cached object and run it via wasmer
// how to bench wavm?

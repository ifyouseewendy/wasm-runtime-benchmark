#![feature(test)]
extern crate test;
use test::Bencher;

mod fibonacci;
pub mod lucet_runner;
pub mod wasmer_runner;

// #[bench]
// fn native(b: &mut Bencher) {
//     b.iter(|| {
//         let n = test::black_box(10);
//         fibonacci::run(n)
//     })
// }
//
// #[bench]
// fn wasmer_jit_with_singlepass(b: &mut Bencher) {
//     b.iter(|| {
//         let n = test::black_box(3);
//         wasmer_runner::jit_with_singlepass("./fibonacci.wasm", n)
//     })
// }
// #[bench]
// fn wasmer_jit_with_cranelift(b: &mut Bencher) {
//     b.iter(|| {
//         let n = test::black_box(3);
//         wasmer_runner::jit_with_cranelift("./fibonacci.wasm", n)
//     })
// }
// // Runs forever..
// // #[bench]
// // fn wasmer_jit_with_llvm(b: &mut Bencher) {
// //     b.iter(|| {
// //         let n = test::black_box(3);
// //         wasmer_runner::jit_with_llvm("./fibonacci.wasm", n)
// //     })
// // }
//
// #[bench]
// fn wasmer_aot_with_singlepass(b: &mut Bencher) {
//     let key = wasmer_runner::store_module(wasmer_runtime::Backend::Singlepass, "./fibonacci.wasm")
//         .unwrap();
//     b.iter(|| {
//         let n = test::black_box(3);
//         wasmer_runner::aot_with_singlepass(&key, n)
//     })
// }
// #[bench]
// fn wasmer_aot_with_cranelift(b: &mut Bencher) {
//     let key = wasmer_runner::store_module(wasmer_runtime::Backend::Cranelift, "./fibonacci.wasm")
//         .unwrap();
//     b.iter(|| {
//         let n = test::black_box(3);
//         wasmer_runner::aot_with_cranelift(&key, n)
//     })
// }
// #[bench]
// fn wasmer_aot_with_llvm(b: &mut Bencher) {
//     let key =
//         wasmer_runner::store_module(wasmer_runtime::Backend::LLVM, "./fibonacci.wasm").unwrap();
//     b.iter(|| {
//         let n = test::black_box(3);
//         wasmer_runner::aot_with_llvm(&key, n)
//     })
// }

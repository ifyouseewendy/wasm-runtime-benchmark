#![feature(test)]
extern crate test;
use test::Bencher;

mod fibonacci;

#[bench]
fn recursive_fibonacci(b: &mut Bencher) {
    b.iter(fibonacci::recursive_fibonacci)
}

#[bench]
fn iterative_fibonacci(b: &mut Bencher) {
    b.iter(fibonacci::iterative_fibonacci)
}

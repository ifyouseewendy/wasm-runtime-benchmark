// https://www.cs.brandeis.edu/~cs146a/rust/rustbyexample-02-21-2015/bench.html
static BENCH_SIZE: usize = 20;

use std::mem::replace;

// recursive fibonacci
fn fibonacci(n: usize) -> u32 {
    if n < 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

// iterative fibonacci
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let new_next = self.curr + self.next;
        let new_curr = replace(&mut self.next, new_next);

        Some(replace(&mut self.curr, new_curr))
    }
}

fn fibonacci_sequence() -> Fibonacci {
    Fibonacci { curr: 1, next: 1 }
}

pub fn recursive_fibonacci() -> Vec<u32> {
    (0..BENCH_SIZE).map(fibonacci).collect::<Vec<u32>>()
}

pub fn iterative_fibonacci() -> Vec<u32> {
    fibonacci_sequence().take(BENCH_SIZE).collect::<Vec<u32>>()
}

#![feature(test)]

extern crate euler;
extern crate test;

use euler::problems as p;
use euler::tools as t;

use test::Bencher;

const N: u64 = 30;

fn fib_naiv(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_naiv(n - 1) + fib_naiv(n - 2),
    }
}

#[bench]
fn bench_fib(b: &mut Bencher) {
    b.iter(|| t::fib(N));
}

#[bench]
fn bench_fib_naiv(b: &mut Bencher) {
    b.iter(|| fib_naiv(N));
}



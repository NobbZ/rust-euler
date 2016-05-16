#![feature(test)]

extern crate euler;
extern crate test;

use euler::problems as p;
use euler::tools as t;

#[test]
fn problem001 () {
  assert_eq!(233168, p::problem001());
}

#[test]
fn problem002 () {
  assert_eq!(4613732, p::problem002());
}

#[test]
fn problem003 () {
  assert_eq!(6857, p::problem003());
}

#[test]
fn problem004() {
    assert_eq!(906609, p::problem004());
}

#[test]
fn fib () {
  assert_eq!(0, t::fib(0));
  assert_eq!(1, t::fib(1));
  assert_eq!(1, t::fib(2));
  assert_eq!(2, t::fib(3));
  for i in 1u64..15 {
    assert_eq!(fib_naiv(i), t::fib(i));
  }
}

#[test]
fn prime_factors () {
  assert_eq!(vec![2],    t::prime_factors( 2));
  assert_eq!(vec![3],    t::prime_factors( 3));
  assert_eq!(vec![2, 2], t::prime_factors( 4));
  assert_eq!(vec![5],    t::prime_factors( 5));
  assert_eq!(vec![5, 2], t::prime_factors(10));
}

#[bench]
fn bench_fib (b: &mut test::Bencher) {
  b.iter(|| t::fib(30) );
}

#[bench]
fn bench_fib_naiv(b: &mut test::Bencher) {
  b.iter(|| fib_naiv(30) );
}

fn fib_naiv (n: u64) -> u64 {
  match n {
    0 => 0,
    1 => 1,
    _ => fib_naiv(n-1) + fib_naiv(n-2)
  }
}

use std::iter::AdditiveIterator;
use std::iter::IteratorExt;

use ::tools;

pub fn problem001 () -> u64 {
  ( 3 * (333*333 + 333) / 2) + ( 5 * (199*199 + 199) / 2) - (15 * ( 66* 66 +  66) / 2)
}

pub fn problem002 () -> u64 {
  (1u64..90).map(|x| tools::fib(x))
            .filter(|&x| x <= 4_000_000)
            .filter(|&x| x % 2 == 0)
            .sum()
}

pub fn problem003 () -> u64 {
  let factors = tools::prime_factors(600851475143);
  let maxun   = factors.iter().max().unwrap();
  *maxun
}

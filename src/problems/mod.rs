use std::iter::AdditiveIterator;

use ::tools;

pub fn problem001 () -> u64 {
  ( 3 * (333*333 + 333) / 2) + ( 5 * (199*199 + 199) / 2) - (15 * ( 66* 66 +  66) / 2)
}

pub fn problem002 () -> u64 {
  (1..90).map(|x| tools::fib(x))
         .filter(|&x| x <= 4_000_000)
         .filter(|&x| x % 2 == 0)
         .sum()
}
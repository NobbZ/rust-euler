// use std::iter::AdditiveIterator;
use std::iter::Iterator;

use ::tools;

pub fn problem001() -> u64 {
    (3 * (333 * 333 + 333) / 2) + (5 * (199 * 199 + 199) / 2) - (15 * (66 * 66 + 66) / 2)
}

pub fn problem002() -> u64 {
    (1u64..90)
        .map(tools::fib)
        .filter(|&x| x <= 4_000_000)
        .filter(|&x| x % 2 == 0)
        .sum()
}

pub fn problem003() -> u64 {
    let factors = tools::prime_factors(600851475143);
    *factors.iter().max().unwrap()
}

pub fn problem004() -> u64 {
    let xs = 100..1000; // Ranges are exclusive!
    let mut products: Vec<u64> = vec![];
    for x in xs {
        for y in 100..(x + 1) {
            let prod = x * y;
            if tools::is_palindrome(&(format!("{}", prod))) {
                products.push(prod);
            }
        }
    }
    *products.iter().max().unwrap()
}

pub fn problem005() -> u64 {
    let xs = 2..21;
    xs.fold(1, tools::lcm)
}

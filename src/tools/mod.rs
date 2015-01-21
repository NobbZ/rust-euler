pub fn fib (n: u64) -> u64 {
  fib_rec(n, 0, 1)
}

fn fib_rec(n: u64, a: u64, b: u64) -> u64 {
  if n == 0 { a } else { fib_rec(n-1, b, a + b) }
}

pub fn prime_factors (n: u64) -> Vec<u64> {
  prime_factors_rec(2, n)
}

fn prime_factors_rec(n: u64, m: u64) -> Vec<u64> {
  let mut res = vec!();
  if n * n > m {
    res.push(m);
  } else if m % n == 0 {
    res = prime_factors_rec(n, m / n);
    res.push(n);
  } else {
    res = prime_factors_rec(n+1, m);
  }
  res
}
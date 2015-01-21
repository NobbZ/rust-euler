pub fn fib (n: u64) -> u64 {
  fib_rec(n, 0, 1)
}

fn fib_rec(n: u64, a: u64, b: u64) -> u64 {
  if n == 0 { a } else { fib_rec(n-1, b, a + b) }
}

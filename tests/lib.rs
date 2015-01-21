extern crate euler;

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
fn fib () {
  assert_eq!(0, t::fib(0));
  assert_eq!(1, t::fib(1));
  assert_eq!(1, t::fib(2));
  assert_eq!(2, t::fib(3));
}
}
extern crate euler;

use std::os;

use euler::problems;

fn main () {
  let args = os::args();

  println!("Result of problem {:>3}: {:>15}", args[1], match args[1].as_slice() {
    "1" => problems::problem001(),
    "2" => problems::problem002(),
    _   => 0
  });
}

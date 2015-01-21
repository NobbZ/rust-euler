extern crate euler;
extern crate stopwatch;

use std::os;

use euler::problems;

fn main () {
  let args = os::args();

  let sw = stopwatch::Stopwatch::start_new();

  println!("Result of problem {:>3}: {:>15}", args[1], match args[1].as_slice() {
    "1" => problems::problem001(),
    "2" => problems::problem002(),
    "3" => problems::problem003(),
    _   => 0
  });

  println!("Calculation took {}ms", sw.elapsed_ms());
}

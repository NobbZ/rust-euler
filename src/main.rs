extern crate euler;
extern crate stopwatch;

use euler::problems;

fn main () {
    let args = std::env::args().collect::<Vec<_>>();

    let sw = stopwatch::Stopwatch::start_new();

    if args.len() > 1 {
        println!("Result of problem {:>3}: {:>15}", args[1], match args[1].as_str() {
            "1" => problems::problem001(),
            "2" => problems::problem002(),
            "3" => problems::problem003(),
            "4" => problems::problem004(),
            _   => 0
        });
    } else {
        let mut foo: Vec<fn() -> u64> = vec![];
        foo.push(problems::problem001);
        foo.push(problems::problem002);
        foo.push(problems::problem003);
        foo.push(problems::problem004);
        for f in &foo {
            println!("Result is {:>15}", (*f)());
        }
    }

    println!("Calculation took {}ms", sw.elapsed_ms());
}

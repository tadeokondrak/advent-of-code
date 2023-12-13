use std::io::stdin;

use intcode::Intcode;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut intcode = Intcode::new(input.trim()).unwrap();
    intcode.run(|| 1, |p1| println!("Part 1: {}", p1)).unwrap();
    let mut intcode = Intcode::new(input.trim()).unwrap();
    intcode.run(|| 2, |p2| println!("Part 2: {}", p2)).unwrap();
}

use std::io::stdin;

mod intcode;
use intcode::Intcode;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let intcode = Intcode::new(input.trim()).unwrap();
    let (mut p1, mut p2) = (0, 0);
    intcode.clone().run(|| 1, |x| p1 = x).unwrap();
    intcode.clone().run(|| 5, |x| p2 = x).unwrap();
    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

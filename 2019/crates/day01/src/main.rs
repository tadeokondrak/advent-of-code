use std::io::{stdin, Read};

fn fuel(weight: usize) -> usize {
    (weight / 3).saturating_sub(2)
}

fn fuel_rec(weight: usize) -> usize {
    match fuel(weight) {
        0 => 0,
        x => x + fuel_rec(x),
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let (p1, p2) = input
        .lines()
        .map(|line| line.parse().unwrap())
        .map(|num| (fuel(num), fuel_rec(num)))
        .fold((0, 0), |(f1, f2), (a1, a2)| (a1 + f1, a2 + f2));

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

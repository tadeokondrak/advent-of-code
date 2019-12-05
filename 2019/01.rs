use std::io::{stdin, BufRead};

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
    let (p1, p2) = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .map(|num| (fuel(num), fuel_rec(num)))
        .fold((0, 0), |(f1, f2), (a1, a2)| (a1 + f1, a2 + f2));

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve_p1(&input));
    println!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .sum::<i32>()
}

fn solve_p2(input: &str) -> i32 {
    let mut frequency: i32 = 0;
    let mut seen = HashSet::new();
    for number in input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .cycle()
    {
        frequency += number;
        if seen.contains(&frequency) {
            return frequency;
        }
        seen.insert(frequency);
    }
    unreachable!()
}

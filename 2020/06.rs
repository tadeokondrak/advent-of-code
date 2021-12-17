use std::{
    collections::HashMap,
    io::{stdin, Read},
};

fn main() {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let (mut p1, mut p2) = (0, 0);
    for entry in input.split("\n\n") {
        let mut count = 1;
        let mut answers = HashMap::new();
        for c in entry.trim_end_matches('\n').chars() {
            if c == '\n' {
                count += 1;
                continue;
            }
            *answers.entry(c).or_insert(0) += 1;
        }
        p1 += answers.len();
        p2 += answers.iter().filter(|&(_, &c)| c == count).count();
    }
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

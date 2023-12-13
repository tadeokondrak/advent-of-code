use std::io::{stdin, BufRead};

fn main() {
    let lines = stdin()
        .lock()
        .lines()
        .map(|r| r.unwrap())
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect::<Vec<Vec<bool>>>();

    let check = |right, down| {
        lines
            .iter()
            .skip(down)
            .step_by(down)
            .zip((right..).step_by(right))
            .filter(|(line, x)| line[x % line.len()])
            .count()
    };

    let p1 = check(3, 1);
    println!("Part 1: {}", p1);

    let p2 = check(1, 1) * check(3, 1) * check(5, 1) * check(7, 1) * check(1, 2);
    println!("Part 2: {}", p2);
}

use std::io::{stdin, BufRead};

fn main() {
    let (p1, p2) = stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut words = line.split(" ");
            let (min, max) = {
                let first = words.next().unwrap();
                let mut nums = first.split("-");
                (
                    nums.next().unwrap().parse().unwrap(),
                    nums.next().unwrap().parse().unwrap(),
                )
            };
            let letter = words.next().unwrap().chars().next().unwrap();
            let password = words.next().unwrap().to_string();
            (min, max, letter, password)
        })
        .map(|(min, max, letter, password): (usize, usize, _, _)| {
            let count = password.chars().filter(|&c| c == letter).count();
            let a = password.chars().nth(min - 1).unwrap() == letter;
            let b = password.chars().nth(max - 1).unwrap() == letter;
            (count >= min && count <= max, a != b)
        })
        .fold((0, 0), |(first, second), (p1, p2)| {
            (first + p1 as i32, second + p2 as i32)
        });
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

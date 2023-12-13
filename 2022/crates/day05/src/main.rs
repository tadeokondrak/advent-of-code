use std::{
    collections::VecDeque,
    io::{self, Read},
};

use sscanf::sscanf;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let count = (crates.lines().next().unwrap().len() + 1) / 4;
    let mut stacks = Vec::new();
    stacks.resize(count, VecDeque::new());
    for line in crates.lines() {
        for i in 0..count {
            let c = line.chars().nth(4 * i + 1).unwrap();
            if c.is_ascii_uppercase() {
                stacks[i].push_front(c);
            }
        }
    }
    for line in instructions.lines() {
        let (count, from, to) = sscanf!(line, "move {usize} from {usize} to {usize}").unwrap();
        for _ in 0..count {
            let c = stacks[from - 1].pop_back().unwrap();
            stacks[to - 1].push_back(c);
        }
    }
    stacks
        .iter()
        .map(|stack| stack.back().unwrap())
        .collect::<String>()
}

fn part2(input: &str) -> String {
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let count = (crates.lines().next().unwrap().len() + 1) / 4;
    let mut stacks = Vec::new();
    stacks.resize(count, VecDeque::new());
    for line in crates.lines() {
        for i in 0..count {
            let c = line.chars().nth(4 * i + 1).unwrap();
            if c.is_ascii_uppercase() {
                stacks[i].push_front(c);
            }
        }
    }
    for line in instructions.lines() {
        let (count, from, to) = sscanf!(line, "move {usize} from {usize} to {usize}").unwrap();
        let mut temp = Vec::new();
        for _ in 0..count {
            let c = stacks[from - 1].pop_back().unwrap();
            temp.push(c);
        }
        stacks[to - 1].extend(temp.iter().rev());
    }
    stacks
        .iter()
        .map(|stack| stack.back().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "CMZ");
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), "MCD");
    }
}

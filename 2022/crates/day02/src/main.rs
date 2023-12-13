use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

const LEFT_MOVES: &str = "ABC";
const RIGHT_MOVES: &str = "XYZ";

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn part1(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let (left, right) = line.split_once(' ').unwrap();
        let left = i32::try_from(LEFT_MOVES.find(left).unwrap() + 1).unwrap();
        let right = i32::try_from(RIGHT_MOVES.find(right).unwrap() + 1).unwrap();
        score += right
            + match (left, right) {
                _ if left == right => 3,
                (ROCK, PAPER) => 6,
                (PAPER, SCISSORS) => 6,
                (SCISSORS, ROCK) => 6,
                _ => 0,
            };
    }
    score
}

fn part2(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let (left, right) = line.split_once(' ').unwrap();
        let left = i32::try_from(LEFT_MOVES.find(left).unwrap() + 1).unwrap();
        let right = match (left, right) {
            (ROCK, "X") => SCISSORS,
            (PAPER, "X") => ROCK,
            (SCISSORS, "X") => PAPER,
            (ROCK, "Z") => PAPER,
            (PAPER, "Z") => SCISSORS,
            (SCISSORS, "Z") => ROCK,
            (x, "Y") => x,
            _ => panic!(),
        };
        score += right
            + match (left, right) {
                _ if left == right => 3,
                (ROCK, PAPER) => 6,
                (PAPER, SCISSORS) => 6,
                (SCISSORS, ROCK) => 6,
                _ => 0,
            };
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 12);
    }
}

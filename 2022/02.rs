use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    const LEFT_MOVES: &str = "ABC";
    const RIGHT_MOVES: &str = "XYZ";
    let mut score = 0;
    for line in input.lines() {
        let (left, right) = line.split_once(' ').unwrap();
        let left = i32::try_from(LEFT_MOVES.find(left).unwrap() + 1).unwrap();
        let right = i32::try_from(RIGHT_MOVES.find(right).unwrap() + 1).unwrap();
        score += right
            + match (left, right) {
                _ if left == right => 3,
                (1, 2) => 6,
                (2, 3) => 6,
                (3, 1) => 6,
                _ => 0,
            };
    }
    score
}

fn part2(input: &str) -> i32 {
    const LEFT_MOVES: &str = "ABC";
    let mut score = 0;
    for line in input.lines() {
        let (left, right) = line.split_once(' ').unwrap();
        let left = i32::try_from(LEFT_MOVES.find(left).unwrap() + 1).unwrap();
        let right = match (left, right) {
            (1, "X") => 3,
            (2, "X") => 1,
            (3, "X") => 2,
            (1, "Z") => 2,
            (2, "Z") => 3,
            (3, "Z") => 1,
            (x, "Y") => x,
            _ => panic!(),
        };
        score += right
            + match (left, right) {
                _ if left == right => 3,
                (1, 2) => 6,
                (2, 3) => 6,
                (3, 1) => 6,
                _ => 0,
            };
    }
    score
}

#[cfg(test)]
mod test {
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

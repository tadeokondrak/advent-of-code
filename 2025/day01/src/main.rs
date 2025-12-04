use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut count = 0;
    let mut pos = 50i64;
    for line in input.lines() {
        let (lhs, rhs) = line.split_at(1);
        let num = rhs.parse::<i64>().unwrap();
        if lhs == "L" {
            pos += num;
        } else {
            pos -= num;
        }
        while pos < 0 {
            pos += 100;
        }
        while pos >= 100 {
            pos -= 100;
        }
        if pos == 0 {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> i64 {
    let mut count = 0;
    let mut pos = 50i64;
    for line in input.lines() {
        let (lhs, rhs) = line.split_at(1);
        let num = rhs.parse::<i64>().unwrap();
        // dumb
        for _ in 0..num {
            if lhs == "L" {
                pos -= 1;
            } else {
                pos += 1;
            }
            if pos < 0 {
                pos += 100;
            }
            if pos >= 100 {
                pos -= 100;
            }
            if pos == 0 {
                count += 1;
            }
        }

    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 6);
    }
}

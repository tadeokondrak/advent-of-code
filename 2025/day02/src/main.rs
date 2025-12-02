#![feature(test)]
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut total = 0;
    for range in input.split(",") {
        let (lhs, rhs) = range.split_once("-").unwrap();
        let lhs = lhs.parse::<i64>().unwrap();
        let rhs = rhs.parse::<i64>().unwrap();
        for i in lhs..=rhs {
            let s = i.to_string();
            let s = s.as_bytes();
            if s.len() < 2 || s.len() % 2 == 1 {
                continue;
            }
            let l = s.len() / 2;
            if s.chunks(l).all(|chunk| chunk == &s[..l]) {
                total += i;
            }
        }
    }
    total
}

fn part2(input: &str) -> i64 {
    let mut total = 0;
    for range in input.split(",") {
        let (lhs, rhs) = range.split_once("-").unwrap();
        let lhs = lhs.parse::<i64>().unwrap();
        let rhs = rhs.parse::<i64>().unwrap();
        for i in lhs..=rhs {
            let s = i.to_string();
            let s = s.as_bytes();
            if s.len() < 2 {
                continue;
            }
            for l in 1..=s.len() / 2 {
                if s.chunks(l).all(|chunk| chunk == &s[..l]) {
                    total += i;
                    break;
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4174379265);
    }
}

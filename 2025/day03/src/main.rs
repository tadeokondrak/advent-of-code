#![feature(test)]
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    let parsed = parse(input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().into())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

fn part1(lines: &Vec<Vec<i64>>) -> i64 {
    let mut total = 0i64;
    for line in lines {
        let mut largest = 0;
        for i in 0..line.len() {
            for j in i + 1..line.len() {
                largest = largest.max(line[i] * 10 + line[j]);
            }
        }
        total += largest;
    }
    total
}

fn part2(lines: &Vec<Vec<i64>>) -> i64 {
    fn go(line: &[i64], amt: i64, start: usize, remaining: i64, largest_seen: &mut i64) -> i64 {
        let largest_possible = {
            let shifted = amt * 10i64.pow((remaining) as u32);
            let nines = 10i64.pow((remaining) as u32) - 1;
            shifted + nines
        };
        if largest_possible <= *largest_seen {
            return 0;
        }
        if remaining == 0 || line.len() == start {
            return amt;
        }
        let without_first = go(line, amt, start + 1, remaining, largest_seen);
        let with_first = {
            let shifted = amt * 10 + line[start];
            go(line, shifted, start + 1, remaining - 1, largest_seen)
        };
        let result = with_first.max(without_first);
        *largest_seen = (*largest_seen).max(result);
        result
    }
    lines
        .into_iter()
        .map(|line| go(&line, 0, 0, 12, &mut 0))
        .sum()
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(TEST_INPUT)), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(TEST_INPUT)), 3121910778619);
    }

    #[bench]
    fn parsing(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| drop(test::black_box(parse(&input))));
    }

    #[bench]
    fn real_p1(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part1(test::black_box(&parsed)), 17193));
    }

    #[bench]
    fn real_p2(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part2(test::black_box(&parsed)), 171297349921310));
    }
}

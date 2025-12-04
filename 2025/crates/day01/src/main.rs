#![feature(test)]
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    let parsed = parse(&input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn parse(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| {
            let (sign, num) = line.split_at(1);
            let num = num.parse::<i64>().unwrap();
            match sign {
                "L" => -num,
                "R" => num,
                _ => unimplemented!(),
            }
        })
        .collect()
}

fn part1(input: &[i64]) -> i64 {
    let (_pos, count) = input
        .into_iter()
        .fold((50, 0), |(mut pos, mut count), &num| {
            pos = (pos + num).rem_euclid(100);
            if pos == 0 {
                count += 1;
            }
            (pos, count)
        });
    count
}

fn part2(input: &[i64]) -> i64 {
    let (_pos, count) = input
        .into_iter()
        .fold((50, 0), |(mut pos, mut count), &num| {
            if num > 0 && (pos + num % 100) >= 100 {
                count += 1;
            }
            if num < 0 && pos != 0 && (pos + num % 100) <= 0 {
                count += 1;
            }
            count += num.abs() / 100;
            pos = (pos + num).rem_euclid(100);
            (pos, count)
        });
    count
}

#[cfg(test)]
mod tests {
    extern crate test;
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
        assert_eq!(part1(&parse(TEST_INPUT)), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(TEST_INPUT)), 6);
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
        b.iter(|| assert_eq!(part1(test::black_box(&parsed)), 1086));
    }

    #[bench]
    fn real_p2(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part2(test::black_box(&parsed)), 6268));
    }
}

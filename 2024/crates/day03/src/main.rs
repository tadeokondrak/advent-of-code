#![feature(test)]
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut total = 0;
    for i in 0..input.len() {
        if let Some(result) = try_parse_mul(&input[i..]) {
            total += result;
        }
    }
    total
}

fn part2(input: &str) -> i32 {
    let mut total = 0;
    let mut dont = false;
    for i in 0..input.len() {
        if input[i..].starts_with("don't()") {
            dont = true;
        }
        if input[i..].starts_with("do()") {
            dont = false;
        }
        if let Some(result) = try_parse_mul(&input[i..]) {
            if !dont {
                total += result;
            }
        }
    }
    total
}

fn try_parse_mul(mut s: &str) -> Option<i32> {
    if !s.starts_with("mul(") {
        return None;
    }
    s = &s[4..];
    let mut it = s.chars();
    let mut lhs = 0;
    let mut rhs = 0;
    while let Some(c) = it.next() {
        if let Some(d) = c.to_digit(10).map(|it| it as i32) {
            lhs = lhs * 10 + d;
            continue;
        } else {
            match c {
                ',' => break,
                _ => return None,
            }
        }
    }
    while let Some(c) = it.next() {
        if let Some(d) = c.to_digit(10).map(|it| it as i32) {
            rhs = rhs * 10 + d;
            continue;
        } else {
            match c {
                ')' => break,
                _ => return None,
            }
        }
    }
    Some(lhs * rhs)
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }

    #[bench]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(black_box(&input)), 159833790));
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(black_box(&input)), 89349241));
    }
}

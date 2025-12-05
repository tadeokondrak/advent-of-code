#![feature(test)]
use std::{
    io::{self, Read},
    ops::{Range, RangeInclusive},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let (fresh, avail) = input.split_once("\n\n").unwrap();
    let fresh_ranges: Vec<RangeInclusive<i64>> = fresh
        .split('\n')
        .map(|line| {
            let (lhs, rhs) = line.split_once('-').unwrap();
            lhs.parse().unwrap()..=rhs.parse().unwrap()
        })
        .collect();
    let avail: Vec<i64> = avail.split('\n').map(|x| x.parse().unwrap()).collect();
    avail
        .iter()
        .filter(|x| fresh_ranges.iter().any(|range| range.contains(x)))
        .count() as i64
}

fn part2(input: &str) -> i64 {
    fn make_disjoint(mut a: Range<i64>, mut b: Range<i64>) -> (Range<i64>, Range<i64>) {
        // sorted
        assert!(a.start <= b.start);
        // fully contained
        if a.start <= b.start && b.end <= a.end {
            b.end = b.start;
            return (a, b);
        }
        // overlapping
        if b.start <= a.end {
            a.end = b.start;
        }
        (a, b)
    }
    let (fresh, _) = input.split_once("\n\n").unwrap();
    let mut fresh_ranges: Vec<Range<i64>> = fresh
        .split('\n')
        .map(|line| {
            let (lhs, rhs) = line.split_once('-').unwrap();
            lhs.parse().unwrap()..rhs.parse::<i64>().unwrap() + 1
        })
        .collect();
    fresh_ranges.sort_by_key(|x| x.start);
    for i in 0..fresh_ranges.len() {
        for j in i + 1..fresh_ranges.len() {
            (fresh_ranges[i], fresh_ranges[j]) =
                make_disjoint(fresh_ranges[i].clone(), fresh_ranges[j].clone());
        }
    }
    fresh_ranges.iter().map(|x| x.end - x.start).sum()
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 14);
    }

    #[bench]
    fn real_p1(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let input = input.trim();
        b.iter(|| assert_eq!(part1(test::black_box(&input)), 874));
    }

    #[bench]
    fn real_p2(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let input = input.trim();
        b.iter(|| assert_eq!(part2(test::black_box(&input)), 348548952146313));
    }
}

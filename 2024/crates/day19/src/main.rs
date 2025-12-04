#![feature(test)]
use std::{
    collections::HashMap,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {:?}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let (avail, designs) = input.split_once("\n\n").unwrap();
    let avail = avail
        .split(", ")
        .map(|it| it.as_bytes().to_owned())
        .collect::<Vec<Vec<u8>>>();
    let mut cache = HashMap::new();
    designs
        .lines()
        .filter(|design| check(&mut cache, &avail, design.as_bytes()))
        .count() as i64
}

fn check(cache: &mut HashMap<Vec<u8>, bool>, avail: &[Vec<u8>], design: &[u8]) -> bool {
    if let Some(&result) = cache.get(design) {
        return result;
    }
    if avail.iter().any(|s| s == design) {
        return true;
    }
    for i in 1..design.len() {
        let (l, r) = design.split_at(i);
        if check(cache, avail, l) && check(cache, avail, r) {
            cache.insert(design.to_owned(), true);
            return true;
        }
    }
    cache.insert(design.to_owned(), false);
    false
}

fn check2(cache: &mut HashMap<Vec<u8>, i64>, avail: &[Vec<u8>], design: &[u8]) -> i64 {
    if let Some(&result) = cache.get(design) {
        return result;
    }
    let mut count = 0;
    if avail.iter().any(|s| s == design) {
        count += 1;
    }
    for i in 1..design.len() {
        let (l, r) = design.split_at(i);
        if avail.iter().any(|s| s == l) {
            count += check2(cache, avail, r);
        }
    }
    cache.insert(design.to_owned(), count);
    count
}

fn part2(input: &str) -> i64 {
    let (avail, designs) = input.split_once("\n\n").unwrap();
    let avail = avail
        .split(", ")
        .map(|it| it.as_bytes().to_owned())
        .collect::<Vec<Vec<u8>>>();
    let mut cache = HashMap::new();
    designs
        .lines()
        .map(|design| check2(&mut cache, &avail, design.as_bytes()))
        .sum::<i64>() as i64
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;

    const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 6);
    }

    /*
    #[bench]
    #[ignore = "reason"]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        //b.iter(|| assert_eq!(part1(black_box(&input)), 95476));
    }

    #[bench]
    #[ignore = "reason"]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        //b.iter(|| assert_eq!(part2(black_box(&input)), 511));
    }
    */
}

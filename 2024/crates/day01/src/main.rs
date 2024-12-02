#![feature(test)]
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let parsed = parse(&input);
    println!("Part 1: {}", part1(&parsed));
    println!("Part 2: {}", part2(&parsed));
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut l: Vec<u32> = input
        .lines()
        .map(|line| line.split("   ").next().unwrap().parse().unwrap())
        .collect();
    let mut r: Vec<u32> = input
        .lines()
        .map(|line| line.split("   ").skip(1).next().unwrap().parse().unwrap())
        .collect();
    l.sort();
    r.sort();
    (l, r)
}

fn part1((l, r): &(Vec<u32>, Vec<u32>)) -> u32 {
    l.iter()
        .copied()
        .zip(r.iter().copied())
        .map(|(lv, rv)| lv.abs_diff(rv))
        .sum()
}

fn part2((l, r): &(Vec<u32>, Vec<u32>)) -> u32 {
    l.iter()
        .copied()
        .map(|lv| {
            let freq = r.iter().copied().filter(|&rv| rv == lv).count() as u32;
            lv * freq
        })
        .sum()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(TEST_INPUT)), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(TEST_INPUT)), 31);
    }

    #[bench]
    fn parsing(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| drop(parse(&input)));
    }

    #[bench]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part1(black_box(&parsed)), 1320851));
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(part2(black_box(&parsed)), 26859182));
    }
}

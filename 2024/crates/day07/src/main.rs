#![feature(test)]
use std::{
    fmt::Write as _,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut okay = false;
        let (result, nums) = line.split_once(": ").unwrap();
        let result = result.parse().unwrap();
        let nums = nums
            .split(" ")
            .map(|it| it.parse().unwrap())
            .collect::<Vec<i64>>();
        for bits in 0..(1 << (nums.len() - 1)) {
            let mut last = nums[0];
            for i in 0..nums.len() - 1 {
                if (bits & (1 << i)) == 0 {
                    last = last + nums[i + 1];
                } else {
                    last = last * nums[i + 1];
                }
            }
            if last == result {
                okay = true;
            }
        }
        if okay {
            sum += result;
        }
    }
    sum
}

fn part2(input: &str) -> i64 {
    let mut sum = 0;
    let mut buf = String::new();
    for line in input.lines() {
        let mut okay = false;
        let (result, nums) = line.split_once(": ").unwrap();
        let result = result.parse().unwrap();
        let nums = nums
            .split(" ")
            .map(|it| it.parse().unwrap())
            .collect::<Vec<i64>>();
        for bits in 0..(3i64.pow(nums.len() as u32 - 1)) {
            let mut last = nums[0];
            for i in 0..nums.len() - 1 {
                if bits / 3i64.pow(i as u32) % 3 == 0 {
                    last = last + nums[i + 1];
                } else if bits / 3i64.pow(i as u32) % 3 == 1 {
                    last = last * nums[i + 1];
                } else {
                    assert!(bits / 3i64.pow(i as u32) % 3 == 2);
                    //last = last * 10i64.pow(nums[i + 1].ilog10()) + nums[i + 1];
                    buf.clear();
                    write!(&mut buf, "{}{}", last, nums[i + 1]).unwrap();
                    last = buf.parse().unwrap();
                }
            }
            if last == result {
                okay = true;
            }
        }
        if okay {
            sum += result;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 11387);
    }

    #[bench]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(black_box(&input)), 1620690235709));
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(black_box(&input)), 145397611075341));
    }
}

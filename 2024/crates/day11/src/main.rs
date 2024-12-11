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
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    solve(input, 25)
}

fn part2(input: &str) -> i64 {
    solve(input, 75)
}

fn solve(input: &str, count: usize) -> i64 {
    fn go(num: i64, counts_left: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
        if counts_left == 0 {
            return 1;
        }
        if let Some(result) = cache.get(&(num, counts_left)).copied() {
            return result;
        }
        let result = if num == 0 {
            go(1, counts_left - 1, cache)
        } else if num.ilog10() % 2 == 1 {
            let d = 10i64.pow(num.ilog10() / 2 + 1) as i64;
            go(num / d, counts_left - 1, cache) + go(num % d, counts_left - 1, cache)
        } else {
            go(num * 2024, counts_left - 1, cache)
        };
        cache.insert((num, counts_left), result);
        result
    }
    let nums: Vec<i64> = input.trim().split(" ").map(|it| it.parse().unwrap()).collect();
    let mut cache = HashMap::default();
    nums.iter()
        .copied()
        .map(|num| go(num, count as i64, &mut cache))
        .sum()
}

mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 55312);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 65601038650482);
    }

    #[bench]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(black_box(&input)), 194782));
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(black_box(&input)), 233007586663131));
    }
}

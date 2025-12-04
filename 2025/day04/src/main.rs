#![feature(test)]
use std::io::{self, Read};
use util::{Grid, offset, point};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let grid = Grid::parse(input, |c| c);
    let mut count = 0;
    for pt in grid.points() {
        if grid.get(pt).copied() != Some('@') {
            continue;
        }
        let inner_count = pt
            .neighbors()
            .into_iter()
            .filter(|&neighbor| grid.get(neighbor).copied() == Some('@'))
            .count();
        if inner_count < 4 {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> i64 {
    let mut grid = Grid::parse(input, |c| c);
    let mut count = 0;
    loop {
        let mut did_anything = false;
        for pt in grid.points() {
            if grid.get(pt).copied() != Some('@') {
                continue;
            }
            let inner_count = pt
                .neighbors()
                .into_iter()
                .filter(|&neighbor| grid.get(neighbor).copied() == Some('@'))
                .count();
            if inner_count < 4 {
                count += 1;
                grid.set(pt, '.');
                did_anything = true;
            }
        }
        if !did_anything {
            break count;
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 43);
    }

    #[bench]
    fn real_p1(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(test::black_box(&input)), 1397));
    }

    #[bench]
    fn real_p2(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(test::black_box(&input)), 8758));
    }
}

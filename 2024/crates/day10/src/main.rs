#![feature(test)]
use std::{
    collections::HashSet,
    io::{self, Read},
};
use util::{offset, point, Grid, Point};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let grid = Grid::parse(input, |c| c.to_digit(10).unwrap_or(u32::MAX));

    let mut total = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let mut reachable = HashSet::new();
            let pt = point(x as i32, y as i32);
            if grid[pt] != 0 {
                continue;
            }
            count(&grid, pt, &mut reachable);
            total += reachable.len() as i32;
        }
    }

    total
}

fn part2(input: &str) -> i32 {
    let grid = Grid::parse(input, |c| c.to_digit(10).unwrap_or(u32::MAX));

    let mut total = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let pt = point(x as i32, y as i32);
            if grid[pt] != 0 {
                continue;
            }
            total += count2(&grid, pt);
        }
    }

    total
}

fn count(grid: &Grid<u32>, pt: Point, acc: &mut HashSet<Point>) {
    let cur = grid[pt];
    if cur == 9 {
        acc.insert(pt);
        return;
    }
    let dirs = [offset(0, -1), offset(1, 0), offset(0, 1), offset(-1, 0)];
    for dir in dirs {
        if grid.get(pt + dir).copied() == Some(cur + 1) {
            count(grid, pt + dir, acc);
        }
    }
}

fn count2(grid: &Grid<u32>, pt: Point) -> i32 {
    let cur = grid[pt];
    if cur == 9 {
        return 1;
    }
    let mut total = 0;
    let dirs = [offset(0, -1), offset(1, 0), offset(0, 1), offset(-1, 0)];
    for dir in dirs {
        if grid.get(pt + dir).copied() == Some(cur + 1) {
            total += count2(grid, pt + dir);
        }
    }
    total
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9"
            ),
            2
        );
        assert_eq!(
            part1(
                "..90..9
...1.98
...2..7
6543456
765.987
876....
987...."
            ),
            4
        );
        assert_eq!(part1(TEST_INPUT), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 81);
    }

    #[bench]
    #[ignore = "todo"]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(black_box(&input)), 0));
    }

    #[bench]
    #[ignore = "todo"]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(black_box(&input)), 0));
    }
}

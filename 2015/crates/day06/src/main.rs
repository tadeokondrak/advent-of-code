#![feature(test)]
use std::io::{self, Read};
use util::{Grid, Point, point};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Clone, Copy)]
enum Cmd {
    On,
    Off,
    Toggle,
}

fn parse_cmd(line: &str) -> (Cmd, Point, Point) {
    let (cmd, rest) = if let Some(rest) = line.strip_prefix("turn on ") {
        (Cmd::On, rest)
    } else if let Some(rest) = line.strip_prefix("turn off ") {
        (Cmd::Off, rest)
    } else if let Some(rest) = line.strip_prefix("toggle ") {
        (Cmd::Toggle, rest)
    } else {
        unimplemented!()
    };
    let (lhs, rhs) = rest.split_once(" through ").unwrap();
    let lhs = lhs.parse::<Point>().unwrap();
    let rhs = rhs.parse::<Point>().unwrap();
    (cmd, lhs, rhs)
}

fn part1(input: &str) -> usize {
    let mut grid: Grid<bool> = Grid::new(1000, 1000);
    for line in input.lines() {
        let (cmd, lhs, rhs) = parse_cmd(line);
        for y in lhs.y..=rhs.y {
            for x in lhs.x..=rhs.x {
                let pt = point(x, y);
                let new_state = match (grid.get(pt).unwrap(), cmd) {
                    (_, Cmd::On) => true,
                    (_, Cmd::Off) => false,
                    (x, Cmd::Toggle) => !x,
                };
                grid.set(pt, new_state);
            }
        }
    }
    grid.data.iter().filter(|&&x| x).count()
}

fn part2(input: &str) -> i64 {
    let mut grid: Grid<i64> = Grid::new(1000, 1000);
    for line in input.lines() {
        let (cmd, lhs, rhs) = parse_cmd(line);
        for y in lhs.y..=rhs.y {
            for x in lhs.x..=rhs.x {
                let pt = point(x, y);
                let new_state = match (grid.get(pt).unwrap(), cmd) {
                    (x, Cmd::On) => x + 1,
                    (x, Cmd::Off) => (x - 1).max(0),
                    (x, Cmd::Toggle) => x + 2,
                };
                grid.set(pt, new_state);
            }
        }
    }
    grid.data.iter().copied().sum()
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    #[bench]
    fn real_p1(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let input = input.trim();
        b.iter(|| assert_eq!(part1(test::black_box(input)), 400410));
    }

    #[bench]
    fn real_p2(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let input = input.trim();
        b.iter(|| assert_eq!(part2(test::black_box(input)), 15343601));
    }
}

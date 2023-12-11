#![feature(test)]

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve(parse(&input), 2));
    eprintln!("p2: {}", solve(parse(&input), 1_000_000));
}

type Parsed = Vec<(u32, u32)>;

fn parse(input: &str) -> Parsed {
    let mut galaxies = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x as u32, y as u32));
            }
        }
    }
    galaxies
}

fn solve(mut galaxies: Parsed, n: u32) -> u64 {
    galaxies.sort_unstable_by_key(|&(gx, _gy)| gx);
    let mut x = 0;
    'outer: loop {
        let mut seen = false;
        for &(gx, _gy) in &galaxies {
            if gx == x {
                x += 1;
                continue 'outer;
            }
            if gx > x {
                seen = true;
            }
        }
        if !seen {
            break;
        }

        let pp = galaxies.partition_point(|&(gx, _gy)| gx <= x);

        for (gx, _gy) in &mut galaxies[pp..] {
            debug_assert!(*gx > x);
            *gx += n - 1;
        }

        x += n;
    }

    galaxies.sort_unstable_by_key(|&(_gx, gy)| gy);

    let mut y = 0;
    'outer: loop {
        let mut seen = false;
        for &(_gx, gy) in &galaxies {
            if gy == y {
                y += 1;
                continue 'outer;
            }
            if gy > y {
                seen = true;
            }
        }
        if !seen {
            break;
        }

        let pp = galaxies.partition_point(|&(_gx, gy)| gy <= y);

        for (_gx, gy) in &mut galaxies[pp..] {
            debug_assert!(*gy > y);
            *gy += n - 1;
        }

        y += n;
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let galaxy_a = galaxies[i];
            let galaxy_b = galaxies[j];
            sum += u64::from(galaxy_a.0.abs_diff(galaxy_b.0) + galaxy_a.1.abs_diff(galaxy_b.1));
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn part_1() {
        assert_eq!(
            solve(
                parse(
                    "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
                ),
                2
            ),
            374
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve(
                parse(
                    "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
                ),
                10
            ),
            1030
        );

        assert_eq!(
            solve(
                parse(
                    "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
                ),
                100
            ),
            8410
        );
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
        b.iter(|| assert_eq!(solve(black_box(parsed.clone()), 2), 9805264));
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(solve(black_box(parsed.clone()), 1_000_000), 779032247216));
    }
}

use std::{
    collections::HashSet,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve(&input, 2));
    eprintln!("p2: {}", solve(&input, 1_000_000));
}

fn solve(input: &str, n: u64) -> u64 {
    let mut galaxies = Vec::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x as u64, y as u64));
            }
        }
    }

    let mut x = 0;
    loop {
        if galaxies.iter().copied().all(|(gx, _gy)| gx < x) {
            break;
        }
        if galaxies.iter().copied().any(|(gx, _gy)| gx == x) {
            x += 1;
            continue;
        }
        for (gx, _gy) in galaxies.iter_mut() {
            assert_ne!(*gx, x);
            if *gx > x {
                *gx += n - 1;
            }
        }
        x += n;
    }

    let mut y = 0;
    loop {
        if galaxies.iter().copied().all(|(_gx, gy)| gy < y) {
            break;
        }
        if galaxies.iter().copied().any(|(_gx, gy)| gy == y) {
            y += 1;
            continue;
        }
        for (_gx, gy) in galaxies.iter_mut() {
            assert_ne!(*gy, y);
            if *gy > y {
                *gy += n - 1;
            }
        }
        y += n;
    }

    let mut seen = HashSet::new();
    galaxies
        .iter()
        .copied()
        .enumerate()
        .map(|(ai, galaxy_a)| {
            galaxies
                .iter()
                .copied()
                .enumerate()
                .filter(|&(_bi, galaxy)| galaxy != galaxy_a)
                .map(|(bi, galaxy_b)| {
                    if seen.insert((ai.min(bi), bi.max(ai))) {
                        galaxy_a.0.abs_diff(galaxy_b.0) + galaxy_a.1.abs_diff(galaxy_b.1)
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            solve(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                2
            ),
            374
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                10
            ),
            1030
        );

        assert_eq!(
            solve(
                "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
                100
            ),
            8410
        );
    }
}

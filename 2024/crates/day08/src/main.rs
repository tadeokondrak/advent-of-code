#![feature(test)]
use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};
use util::{point, Grid, Point};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let (grid, antennae) = parse(input);
    let mut antinodes = HashSet::new();
    for y in 0..grid.width as i32 {
        for x in 0..grid.height as i32 {
            let pt = point(x, y);
            for (&_freq, points) in &antennae {
                for &pt1 in points {
                    for &pt2 in points {
                        if pt1 == pt2 {
                            continue;
                        }
                        if !(2 * pt.distance_from(pt1) == pt.distance_from(pt2)
                            || pt.distance_from(pt1) == 2 * pt.distance_from(pt2))
                        {
                            continue;
                        }
                        let dir_to_pt1 = pt - pt1;
                        let dir_to_pt2 = pt - pt2;
                        if !(dir_to_pt1 * 2 == dir_to_pt2 || dir_to_pt1 == dir_to_pt2 * 2) {
                            continue;
                        }
                        antinodes.insert(pt);
                    }
                }
            }
        }
    }

    antinodes.len() as i32
}

fn part2(input: &str) -> i32 {
    let (grid, antennae) = parse(input);
    let mut count = 0;
    for y in 0..grid.width as i32 {
        for x in 0..grid.height as i32 {
            let pt = point(x, y);
            let mut has_antinode = false;
            'l: for (&_freq, points) in &antennae {
                for &pt1 in points {
                    for &pt2 in points {
                        if pt1 == pt2 {
                            continue;
                        }
                        let dir_to_pt1 = pt - pt1;
                        let dir_to_pt2 = pt - pt2;
                        let dir_to_pt1_multiplied =
                            dir_to_pt1 * std::cmp::max(dir_to_pt2.x.abs(), dir_to_pt2.y.abs());
                        let dir_to_pt2_multiplied =
                            dir_to_pt2 * std::cmp::max(dir_to_pt1.x.abs(), dir_to_pt1.y.abs());
                        let are_dirs_equal = dir_to_pt1_multiplied == dir_to_pt2_multiplied;
                        if !(pt == pt1 || pt == pt2) && !are_dirs_equal {
                            continue;
                        }
                        has_antinode = true;
                        break 'l;
                    }
                }
            }
            if has_antinode {
                count += 1;
            }
        }
    }
    count
}

fn parse(input: &str) -> (Grid<char>, HashMap<char, Vec<Point>>) {
    let grid = Grid::parse(input, |ch| ch);
    let mut antennae: HashMap<char, Vec<Point>> = HashMap::new();
    for y in 0..grid.width as i32 {
        for x in 0..grid.height as i32 {
            match grid[(x, y)] {
                '.' => {}
                ch => antennae.entry(ch).or_default().push(point(x, y)),
            }
        }
    }
    (grid, antennae)
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."
            ),
            2
        );
        assert_eq!(part1(TEST_INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 34);
    }

    #[bench]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(black_box(&input)), 271));
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(black_box(&input)), 994));
    }
}

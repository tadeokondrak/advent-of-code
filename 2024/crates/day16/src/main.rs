#![feature(test)]
use std::cmp::Reverse as R;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    io::{self, Read},
};
use util::{point, Grid, Offset, Point};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coord {
    pos: Point,
    dir: Offset,
}

fn part1(input: &str) -> i32 {
    let grid = Grid::parse(input, |c| c == '#');
    let src = find_point(input, 'S');
    let dst = find_point(input, 'E');
    [Offset::NORTH, Offset::EAST, Offset::SOUTH, Offset::WEST]
        .map(|dstdir| lowest_score(&grid, src, Offset::EAST, dst, dstdir))
        .into_iter()
        .min()
        .unwrap()
}

fn lowest_score(
    grid: &Grid<bool>,
    src: Point,
    src_dir: Offset,
    dst: Point,
    dst_dir: Offset,
) -> i32 {
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    visited.insert(Coord {
        pos: src,
        dir: src_dir,
    });
    distances.insert(
        Coord {
            pos: src,
            dir: src_dir,
        },
        0,
    );
    let mut queue = BinaryHeap::new();
    queue.push((
        R(0),
        Coord {
            pos: src,
            dir: src_dir,
        },
    ));
    while let Some((
        R(cur_cost),
        Coord {
            pos: cur_pos,
            dir: cur_dir,
        },
    )) = queue.pop()
    {
        let dirs = [Offset::NORTH, Offset::EAST, Offset::SOUTH, Offset::WEST];
        let cur_dir_idx = dirs.iter().copied().position(|it| it == cur_dir).unwrap();
        let dir_cw = dirs[(cur_dir_idx + 1) % 4];
        let dir_ccw = dirs[(cur_dir_idx + 3) % 4];
        if visited.insert(Coord {
            pos: cur_pos,
            dir: dir_cw,
        }) {
            queue.push((
                R(cur_cost + 1000),
                Coord {
                    pos: cur_pos,
                    dir: dir_cw,
                },
            ));
        }
        if visited.insert(Coord {
            pos: cur_pos,
            dir: dir_ccw,
        }) {
            queue.push((
                R(cur_cost + 1000),
                Coord {
                    pos: cur_pos,
                    dir: dir_ccw,
                },
            ));
        }
        if grid.get(cur_pos + cur_dir) == Some(&false)
            && visited.insert(Coord {
                pos: cur_pos + cur_dir,
                dir: cur_dir,
            })
        {
            queue.push((
                R(cur_cost + 1),
                Coord {
                    pos: cur_pos + cur_dir,
                    dir: cur_dir,
                },
            ));
        }
        distances.insert(
            Coord {
                pos: cur_pos,
                dir: cur_dir,
            },
            cur_cost,
        );
        visited.insert(Coord {
            pos: cur_pos,
            dir: cur_dir,
        });
    }
    distances
        .iter()
        .filter(|&(&Coord { pos, dir }, &_cost)| pos == dst && dir == dst_dir)
        .map(|(_, &cost)| cost)
        .min()
        .unwrap()
}

fn part2(input: &str) -> i32 {
    let grid = Grid::parse(input, |c| c == '#');
    let src = find_point(input, 'S');
    let dst = find_point(input, 'E');
    let mut results = HashMap::new();
    for y in 0..grid.width as i32 {
        for x in 0..grid.height as i32 {
            for middir in [Offset::NORTH, Offset::EAST, Offset::SOUTH, Offset::WEST] {
                for dstdir in [Offset::NORTH, Offset::EAST, Offset::SOUTH, Offset::WEST] {
                    let pt = point(x, y);
                    if grid.get(pt) == Some(&false) {
                        let first_part = lowest_score(&grid, src, Offset::EAST, pt, middir);
                        let second_part = lowest_score(&grid, pt, middir, dst, dstdir);
                        let entry = results
                            .entry(Coord {
                                pos: pt,
                                dir: middir,
                            })
                            .or_insert(first_part + second_part);
                        *entry = (*entry).min(first_part + second_part);
                    }
                }
            }
        }
    }
    let min_cost = results.values().copied().min().unwrap();
    results
        .iter()
        .filter(|&(_, &cost)| cost == min_cost)
        .map(|(&Coord { pos, dir: _ }, _)| pos)
        .collect::<HashSet<Point>>()
        .len() as i32
}

fn find_point(input: &str, mark: char) -> Point {
    let mut pos = None;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == mark {
                pos = Some(point(x as i32, y as i32));
            }
        }
    }
    pos.unwrap()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 7036);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 45);
    }

    #[bench]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(black_box(&input)), 95476));
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(black_box(&input)), 511));
    }
}

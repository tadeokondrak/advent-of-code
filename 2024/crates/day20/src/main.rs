#![feature(test)]
use std::{
    collections::{BTreeMap, HashSet},
    io::{self, Read},
};
use util::{find_point, Grid, Offset, Point};

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
    cheated: bool,
    cheat_time_left: i32,
}
impl Coord {}

fn part1(input: &str) -> i64 {
    let grid = Grid::parse(input, |c| c == '#');
    let src = find_point(input, 'S');
    let dst = find_point(input, 'E');

    let (legit_distances, _) = run_pathfinding(
        &grid,
        Coord {
            pos: src,
            cheated: true,
            cheat_time_left: 0,
        },
    );
    let (distances, cheat_locations) = run_pathfinding(
        &grid,
        Coord {
            pos: src,
            cheated: false,
            cheat_time_left: 0,
        },
    );
    let legit_time = get_distance(&legit_distances, dst, true);
    let mut total = 0;
    for &(start, end) in &cheat_locations {
        let first_leg_time = get_distance(&distances, start, false);
        let (distances, _) = run_pathfinding(
            &grid,
            Coord {
                pos: end,
                cheated: true,
                cheat_time_left: 0,
            },
        );

        let second_leg_time = get_distance(&distances, dst, true);
        let cheat_time = first_leg_time + second_leg_time + 2;

        eprintln!("{}", legit_time - cheat_time);
        if legit_time - cheat_time >= 100 {
            total += 1;
        }
    }

    total
}

fn get_distance(distances: &BTreeMap<Coord, i32>, dst: Point, allow_having_cheated: bool) -> i32 {
    distances
        .get(&Coord {
            pos: dst,
            cheated: allow_having_cheated,
            cheat_time_left: 0,
        })
        .copied()
        .or_else(|| {
            distances
                .get(&Coord {
                    pos: dst,
                    cheated: false,
                    cheat_time_left: 0,
                })
                .copied()
        })
        .unwrap()
}

fn run_pathfinding(
    grid: &Grid<bool>,
    src: Coord,
) -> (BTreeMap<Coord, i32>, HashSet<(Point, Point)>) {
    let mut cheat_locations = HashSet::new();
    let mut visited = HashSet::new();
    let mut distances = BTreeMap::new();
    visited.insert(src);
    distances.insert(src, 0);
    let mut stack = Vec::new();
    stack.push((0, src));
    while let Some((
        cur_cost,
        Coord {
            pos: cur_pos,
            cheated: cur_cheated,
            cheat_time_left: cur_cheat_time_left,
        },
    )) = stack.pop()
    {
        for dir in [Offset::NORTH, Offset::EAST, Offset::SOUTH, Offset::WEST] {
            let coord = Coord {
                pos: cur_pos + dir,
                cheated: cur_cheated,
                cheat_time_left: (cur_cheat_time_left - 1).max(0),
            };
            if grid.get(cur_pos + dir) == Some(&false) && visited.insert(coord) {
                stack.push((cur_cost + 1, coord));
            }
            if !cur_cheated
                && grid.get(cur_pos + dir * 2) == Some(&false)
                && visited.insert(Coord {
                    cheated: true,
                    ..coord
                })
            {
                cheat_locations.insert((cur_pos, cur_pos + dir * 2));
                stack.push((
                    cur_cost + 1,
                    Coord {
                        cheated: true,
                        ..coord
                    },
                ));
            }
        }
        distances.insert(
            Coord {
                pos: cur_pos,
                cheated: cur_cheated,
                cheat_time_left: cur_cheat_time_left,
            },
            cur_cost,
        );
        visited.insert(Coord {
            pos: cur_pos,
            cheated: cur_cheated,
            cheat_time_left: cur_cheat_time_left,
        });
    }
    (distances, cheat_locations)
}

fn part2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;

    const TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }

    /*
    #[bench]
    #[ignore = "reason"]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        //b.iter(|| assert_eq!(part1(black_box(&input)), 95476));
    }

    #[bench]
    #[ignore = "reason"]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        //b.iter(|| assert_eq!(part2(black_box(&input)), 511));
    }
    */
}

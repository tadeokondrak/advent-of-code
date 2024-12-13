#![feature(test)]
use std::{
    collections::{HashMap, HashSet},
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

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum Neighbor {
    OtherRegion(i32),
    Edge,
}

fn part1(input: &str) -> i32 {
    let grid = Grid::parse(input.trim(), |c| c);
    let mut next_id = 0;
    let mut memberships = HashMap::<Point, i32>::new();
    for y in 0..grid.width as i32 {
        for x in 0..grid.height as i32 {
            let pt = point(x, y);
            if memberships.contains_key(&pt) {
                continue;
            }
            let id = next_id;
            next_id += 1;
            memberships.insert(pt, id);
            // neighbors is unreliable here because regions haven't all been built
            // so we have to check again later
            flood(&mut memberships, &grid, pt, &mut HashSet::new());
        }
    }
    // HashMap<enclave id, containing id>
    let mut enclaves = HashMap::<i32, i32>::new();
    for region in 0..next_id {
        let (&pt, _) = memberships.iter().find(|(_pt, &it)| it == region).unwrap();
        let mut neighbors = HashSet::new();
        flood2(
            &mut memberships,
            &grid,
            pt,
            &mut HashSet::new(),
            &mut neighbors,
        );
        if neighbors.len() == 1 {
            match neighbors.iter().copied().next().unwrap() {
                Neighbor::OtherRegion(other) => _ = enclaves.insert(region, other),
                Neighbor::Edge => {}
            };
        }
    }

    let mut total = 0;
    for region in 0..next_id {
        let (&some_pt, _) = memberships.iter().find(|(_pt, &it)| it == region).unwrap();
        let (area, perimeter) = area_stats(&memberships, region);
        let mut enclave_perimeter_offset = 0;
        for (&enclave, &container) in &enclaves {
            if container != region {
                continue;
            }
            let (_area, perimeter) = area_stats(&memberships, enclave);
            enclave_perimeter_offset += perimeter;
        }
        dbg!(grid[some_pt], area, perimeter, enclave_perimeter_offset);
        total += (perimeter + enclave_perimeter_offset) * area;
        eprintln!(
            "{} * {} = {}",
            (area),
            (perimeter + enclave_perimeter_offset),
            (perimeter + enclave_perimeter_offset) * area
        );
    }
    total
}

fn area_stats(memberships: &HashMap<Point, i32>, region: i32) -> (i32, i32) {
    let points = memberships
        .iter()
        .filter(|(_pt, &it)| it == region)
        .map(|(pt, _)| pt);
    // this is an incorrect way to find the perimeter
    let min_x = points.clone().map(|pt| pt.x).min().unwrap();
    let max_x = points.clone().map(|pt| pt.x).max().unwrap();
    let min_y = points.clone().map(|pt| pt.y).min().unwrap();
    let max_y = points.clone().map(|pt| pt.y).max().unwrap();
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    let area = points.count() as i32;
    let perimeter = width * 2 + height * 2;
    (area, perimeter)
}

fn flood(
    memberships: &mut HashMap<Point, i32>,
    grid: &Grid<char>,
    pt: Point,
    neighbors: &mut HashSet<Neighbor>,
) {
    let us = grid[pt];
    let dirs = [offset(0, -1), offset(1, 0), offset(0, 1), offset(-1, 0)];
    for dir in dirs {
        if let Some(&neighbor) = memberships.get(&(pt + dir)) {
            if neighbor != memberships[&pt] {
                neighbors.insert(Neighbor::OtherRegion(neighbor));
            }
            continue;
        } else if grid.get(pt + dir).is_none() {
            neighbors.insert(Neighbor::Edge);
        }
        if grid.get(pt + dir) == Some(&us) {
            memberships.insert(pt + dir, memberships[&pt]);
            flood(memberships, grid, pt + dir, neighbors);
        }
    }
}

fn flood2(
    memberships: &HashMap<Point, i32>,
    grid: &Grid<char>,
    pt: Point,
    visited: &mut HashSet<Point>,
    neighbors: &mut HashSet<Neighbor>,
) {
    if visited.contains(&pt) {
        return;
    }
    let our_character = grid[pt];
    let our_region = memberships[&pt];
    let dirs = [offset(0, -1), offset(1, 0), offset(0, 1), offset(-1, 0)];
    for dir in dirs {
        match grid.get(pt + dir) {
            Some(&c) if c == our_character => {
                visited.insert(pt);
                flood2(memberships, grid, pt + dir, visited, neighbors);
            }
            Some(_) => {
                let &neighbor_region = memberships.get(&(pt + dir)).unwrap();
                assert_ne!(neighbor_region, our_region);
                neighbors.insert(Neighbor::OtherRegion(neighbor_region));
            }
            None => {
                neighbors.insert(Neighbor::Edge);
            }
        }
    }
}

fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        //        assert_eq!(
        //            part1(
        //                "AAAA
        //BBCD
        //BBCC
        //EEEC"
        //            ),
        //            140
        //        );
        //        assert_eq!(
        //            part1(
        //                "OOOOO
        //OXOXO
        //OOOOO
        //OXOXO
        //OOOOO"
        //            ),
        //            772
        //        );
        assert_eq!(part1(TEST_INPUT), 1930);
    }

    #[test]
    #[ignore = "todo"]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0);
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

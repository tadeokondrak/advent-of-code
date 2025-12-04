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
    println!("Part 2: {:?}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut grid = Grid::new(71, 71);
    input
        .lines()
        .take(1024)
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .for_each(|(x, y)| {
            grid[(x, y)] = true;
        });
    let src = point(0, 0);
    let dst = point(70, 70);

    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    visited.insert(src);
    distances.insert(src, 0);
    let mut queue = BinaryHeap::new();
    queue.push((R(0), src));
    while let Some((R(cur_cost), cur_pos)) = queue.pop() {
        let dirs = [Offset::NORTH, Offset::EAST, Offset::SOUTH, Offset::WEST];
        for dir in dirs {
            if grid.get(cur_pos + dir) == Some(&false) && visited.insert(cur_pos + dir) {
                queue.push((R(cur_cost + 1), cur_pos + dir));
            }
        }
        distances.insert(cur_pos, cur_cost);
        visited.insert(cur_pos);
    }
    distances
        .iter()
        .filter(|&(&pos, &_cost)| pos == dst)
        .map(|(_, &cost)| cost)
        .min()
        .unwrap()
}

fn part2(input: &str) -> Point {
    for count in 0..input.lines().count() {
        let mut grid = Grid::new(71, 71);
        let points = input
            .lines()
            .take(count)
            .map(|line| {
                let (x, y) = line.split_once(",").unwrap();
                point(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect::<Vec<Point>>();
        for &pt in &points {
            grid[pt] = true;
        }
        let src = point(0, 0);
        let dst = point(70, 70);

        let mut visited = HashSet::new();
        let mut distances = HashMap::new();
        visited.insert(src);
        distances.insert(src, 0);
        let mut queue = BinaryHeap::new();
        queue.push((R(0), src));
        while let Some((R(cur_cost), cur_pos)) = queue.pop() {
            let dirs = [Offset::NORTH, Offset::EAST, Offset::SOUTH, Offset::WEST];
            for dir in dirs {
                if grid.get(cur_pos + dir) == Some(&false) && visited.insert(cur_pos + dir) {
                    queue.push((R(cur_cost + 1), cur_pos + dir));
                }
            }
            distances.insert(cur_pos, cur_cost);
            visited.insert(cur_pos);
        }
        let solution = distances
            .iter()
            .filter(|&(&pos, &_cost)| pos == dst)
            .map(|(_, &cost)| cost)
            .min();
        if solution.is_none() {
            return points.last().copied().unwrap();
        }
    }
    todo!()
}
#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;

    const TEST_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 22);
    }

    #[test]
    fn test_part2() {}

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

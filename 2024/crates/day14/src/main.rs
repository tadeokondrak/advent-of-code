#![feature(test)]
use std::{
    collections::HashMap,
    io::{self, Read},
};
use util::{offset, point, Grid, Offset, Point};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input, (101, 103)));
    println!("Part 2: {}", part2(&input, (101, 103)));
}

fn part1(input: &str, (width, height): (i32, i32)) -> i32 {
    let (mut positions, velocities) = parse(input);
    for _ in 0..100 {
        for i in 0..positions.len() {
            positions[i] = positions[i] + velocities[i];
            positions[i] = point(
                (positions[i].x + width) % width,
                (positions[i].y + height) % height,
            );
        }
    }
    let mut buckets: HashMap<(bool, bool), i32> = HashMap::new();
    for pos in positions {
        if pos.x == width / 2 || pos.y == height / 2 {
            continue;
        }
        let horizontal = pos.x > width / 2;
        let vertical = pos.y > height / 2;
        *buckets.entry((horizontal, vertical)).or_default() += 1;
    }
    buckets.values().product()
}

fn part2(input: &str, (width, height): (i32, i32)) -> i32 {
    let (mut positions, velocities) = parse(input);
    let mut grid: Grid<char> = Grid::new(width as usize, height as usize);
    for iteration in 0.. {
        let mut max_consecutive = 0;
        grid.data.fill(' ');
        for i in 0..positions.len() {
            grid[positions[i]] = '#';
        }

        for y in 0..grid.height as i32 {
            let mut current_consecutive = 0;
            for x in 0..grid.width as i32 {
                let this = grid[point(x, y)];
                if this == '#' {
                    current_consecutive += 1;
                } else {
                    current_consecutive = 0;
                }
                max_consecutive = max_consecutive.max(current_consecutive);
            }
        }

        if max_consecutive > 30 {
            return iteration;
        }

        for i in 0..positions.len() {
            positions[i] = positions[i] + velocities[i];
            positions[i] = point(
                (positions[i].x + width) % width,
                (positions[i].y + height) % height,
            );
        }
    }
    unreachable!()
}

fn parse(input: &str) -> (Vec<Point>, Vec<Offset>) {
    input
        .lines()
        .map(|line| {
            let (pos, vel) = line.strip_prefix("p=").unwrap().split_once(" v=").unwrap();
            let pos = pos
                .split_once(",")
                .map(|(x, y)| point(x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            let vel = vel
                .split_once(",")
                .map(|(x, y)| offset(x.parse().unwrap(), y.parse().unwrap()))
                .unwrap();
            (pos, vel)
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT, (11, 7)), 12);
    }

    #[bench]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(black_box(&input), (101, 103)), 218433348));
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(black_box(&input), (101, 103)), 6512));
    }
}

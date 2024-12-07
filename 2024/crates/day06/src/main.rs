// this file is a mess of failed optimization attempts
// it was fun to try
#![feature(test)]
use rustc_hash::FxHashSet as HashSet;
use std::{
    cmp::{max, min},
    collections::BTreeSet,
    io::{self, Read},
};
use util::{offset, point, Offset, Point};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Ranges {
    xranges: Vec<(i32, i32, i32)>,
    yranges: Vec<(i32, i32, i32)>,
}

impl Ranges {
    // requires xranges to be sorted by first tuple member
    fn count(&self) -> i32 {
        let mut count = 0;
        for &(_, min_y, max_y) in &self.xranges {
            count += max_y - min_y + 1;
        }
        for &(y, min_x, max_x) in &self.yranges {
            count += max_x - min_x + 1;
            let start = self.xranges.partition_point(|&(x, _, _)| x < min_x);
            let len = self.xranges[start..].partition_point(|&(x, _, _)| x <= max_x);
            for &(_, min_y, max_y) in &self.xranges[start..start + len] {
                if min_y <= y && y <= max_y {
                    count -= 1;
                }
            }
        }
        count
    }
}

fn solve(grid: &Grid, mut pos: Point, mut dir: Offset) -> Ranges {
    let mut xranges = Vec::new();
    let mut yranges = Vec::new();
    let mut addrange = |from: Point, to: Point| {
        if from.x == to.x {
            xranges.push((from.x, from.y.min(to.y), from.y.max(to.y)));
        } else {
            yranges.push((from.y, from.x.min(to.x), from.x.max(to.x)));
        }
    };
    loop {
        let range_start = pos;
        match cast_ray(&grid, pos, dir) {
            Ok(next_pos) => {
                dir = rotate_right(dir);
                pos = next_pos;
                let range_end = pos;
                addrange(range_start, range_end);
            }
            Err(next_pos) => {
                pos = next_pos;
                let range_end = pos;
                addrange(range_start, range_end);
                break;
            }
        }
    }
    Ranges { xranges, yranges }
}

fn part1(input: &str) -> i32 {
    let grid = Grid::parse(input, |c| c == '#');
    let pos = find_starting_point(input);
    let dir = offset(0, -1);
    let mut ranges = solve(&grid, pos, dir);
    ranges.xranges.sort_unstable_by_key(|&(x, _, _)| x);
    ranges.count()
}

fn checkloop(grid: &Grid, mut pos: Point, mut dir: Offset) -> bool {
    let mut visited = HashSet::default();
    visited.insert((pos, dir));
    loop {
        match cast_ray(&grid, pos, dir) {
            Ok(next_pos) => {
                dir = rotate_right(dir);
                pos = next_pos;
                if !visited.insert((pos, dir)) {
                    return true;
                }
            }
            Err(_) => {
                return false;
            }
        }
    }
}

fn cast_ray(grid: &Grid, pos: Point, dir: Offset) -> Result<Point, Point> {
    let result = match dir {
        Offset { x: 0, y: -1 } => grid
            .data_x
            .range((pos.x, i32::MIN)..(pos.x, pos.y))
            .copied()
            .rev()
            .next()
            .map(|(x, y)| point(x, y + 1))
            .ok_or(point(pos.x, 0)),
        Offset { x: -1, y: 0 } => grid
            .data_y
            .range((pos.y, i32::MIN)..(pos.y, pos.x))
            .copied()
            .rev()
            .next()
            .map(|(y, x)| point(x + 1, y))
            .ok_or(point(0, pos.y)),
        Offset { x: 0, y: 1 } => grid
            .data_x
            .range((pos.x, pos.y + 1)..=(pos.x, i32::MAX))
            .copied()
            .next()
            .map(|(x, y)| point(x, y - 1))
            .ok_or(point(pos.x, grid.width - 1)),
        Offset { x: 1, y: 0 } => grid
            .data_y
            .range((pos.y, pos.x + 1)..=(pos.y, i32::MAX))
            .copied()
            .next()
            .map(|(y, x)| point(x - 1, y))
            .ok_or(point(grid.height - 1, pos.y)),
        _ => unreachable!(),
    };
    if let Some((x, y)) = grid.overlay {
        match dir {
            Offset { x: 0, y: -1 } if x == pos.x && y < pos.y => match result {
                Ok(result) => Ok(point(x, max(result.y, y + 1))),
                Err(_) => Ok(point(x, y + 1)),
            },
            Offset { x: -1, y: 0 } if y == pos.y && x < pos.x => match result {
                Ok(result) => Ok(point(max(result.x, x + 1), y)),
                Err(_) => Ok(point(x + 1, y)),
            },
            Offset { x: 0, y: 1 } if x == pos.x && pos.y < y => match result {
                Ok(result) => Ok(point(x, min(result.y, y - 1))),
                Err(_) => Ok(point(x, y - 1)),
            },
            Offset { x: 1, y: 0 } if y == pos.y && pos.x < x => match result {
                Ok(result) => Ok(point(min(result.x, x - 1), y)),
                Err(_) => Ok(point(x - 1, y)),
            },
            _ => result,
        }
    } else {
        result
    }
}

fn part2(input: &str) -> i32 {
    let mut grid = Grid::parse(input, |c| c == '#');
    let mut pos = find_starting_point(input);
    let mut dir = offset(0, -1);
    let origpos = pos;
    let origdir = dir;
    let mut points = Vec::new();
    loop {
        let mut oldpos = pos;
        let olddir = dir;
        match cast_ray(&grid, pos, dir) {
            Ok(next_pos) => {
                dir = rotate_right(dir);
                pos = next_pos;
                points.push((oldpos, olddir));
                while oldpos != pos {
                    oldpos += olddir;
                    points.push((oldpos, olddir));
                }
            }
            Err(next_pos) => {
                pos = next_pos;
                while oldpos != pos {
                    points.push((oldpos, olddir));
                    oldpos += olddir;
                }
                break;
            }
        }
    }
    let mut checked = HashSet::default();
    let mut count = 0;
    for (pos, dir) in points {
        if !grid.test(pos + dir) && checked.insert(pos + dir) {
            grid.set(pos + dir);
            if checkloop(&grid, origpos, origdir) {
                count += 1;
            }
            grid.clear(pos + dir);
        }
    }
    count
}

fn rotate_right(dir: Offset) -> Offset {
    match dir {
        Offset { x: 0, y: -1 } => offset(1, 0),
        Offset { x: 1, y: 0 } => offset(0, 1),
        Offset { x: 0, y: 1 } => offset(-1, 0),
        Offset { x: -1, y: 0 } => offset(0, -1),
        _ => unreachable!(),
    }
}

fn find_starting_point(input: &str) -> Point {
    let mut pos = None;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '^' {
                pos = Some(point(x as i32, y as i32));
            }
        }
    }
    pos.unwrap()
}

#[derive(Clone)]
struct Grid {
    data_x: BTreeSet<(i32, i32)>,
    data_y: BTreeSet<(i32, i32)>,
    overlay: Option<(i32, i32)>,
    width: i32,
    height: i32,
}

impl Grid {
    fn parse(s: &str, f: impl Fn(char) -> bool) -> Grid {
        let width = s.trim().lines().next().unwrap().len() as i32;
        let height = s.trim().lines().count() as i32;
        let mut data_x = BTreeSet::new();
        let mut data_y = BTreeSet::new();
        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let x = x as i32;
                let y = y as i32;
                if f(ch) {
                    data_x.insert((x, y));
                    data_y.insert((y, x));
                }
            }
        }
        Grid {
            data_x,
            data_y,
            overlay: None,
            width,
            height,
        }
    }

    fn test(&self, Point { x, y }: Point) -> bool {
        self.overlay == Some((x, y)) || self.data_x.contains(&(x, y))
    }

    fn set(&mut self, Point { x, y }: Point) {
        assert_eq!(self.overlay.replace((x, y)), None);
    }

    fn clear(&mut self, Point { x, y }: Point) {
        assert_eq!(self.overlay.take(), Some((x, y)));
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 6);
    }

    #[bench]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(black_box(&input)), 4982));
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(black_box(&input)), 1663));
    }
}

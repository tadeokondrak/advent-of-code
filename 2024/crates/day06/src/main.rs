#![feature(test)]
use std::io::{self, Read};
use util::{offset, point, Grid, Offset, Point};

struct Parsed {
    grid: Grid<bool>,
    pos: Point,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let Parsed { grid, pos } = parse(input);
    solve(&grid, pos)
        .unwrap()
        .data
        .into_iter()
        .filter(|&x| x)
        .count() as i32
}

fn part2(input: &str) -> i32 {
    let mut count = 0;
    let Parsed { mut grid, pos } = parse(input);
    let visited = solve(&grid, pos).unwrap();
    for y in 0..grid.width as i32 {
        for x in 0..grid.height as i32 {
            if !visited[(x, y)] || grid[(x, y)] {
                continue;
            }
            grid[(x, y)] = true;
            if solve(&grid, pos).is_none() {
                count += 1;
            }
            grid[(x, y)] = false;
        }
    }
    count
}

fn solve(grid: &Grid<bool>, mut pos: Point) -> Option<Grid<bool>> {
    let mut dir = offset(0, -1);
    let mut visited = Grid::new(grid.width, grid.height);
    let mut past_states: Grid<u8> = Grid::new(grid.width, grid.height);
    loop {
        let dirbit = 1 << dir_index(dir);
        if past_states[pos] & dirbit != 0 {
            return None;
        }
        past_states[pos] |= dirbit;
        visited[pos] = true;

        if !grid.is_in_bounds(pos + dir) {
            return Some(visited);
        }

        while grid[pos + dir] {
            dir = rotate_right(dir);
            if !grid.is_in_bounds(pos + dir) {
                return Some(visited);
            }
        }

        pos += dir;
    }
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

fn dir_index(dir: Offset) -> u8 {
    match dir {
        Offset { x: 0, y: -1 } => 0,
        Offset { x: 1, y: 0 } => 1,
        Offset { x: 0, y: 1 } => 2,
        Offset { x: -1, y: 0 } => 3,
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> Parsed {
    let mut pos = None;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '^' {
                pos = Some(point(x as i32, y as i32));
            }
        }
    }
    Parsed {
        grid: Grid::parse(input, |c| c == '#'),
        pos: pos.unwrap(),
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

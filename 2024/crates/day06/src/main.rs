#![feature(test)]
use std::{
    collections::HashSet,
    io::{self, Read},
};

struct Parsed {
    grid: Vec<bool>,
    width: i32,
    height: i32,
    pos: (i32, i32),
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    solve(input).unwrap().iter().copied().filter(|&x| x).count() as i32
}

fn part2(input: &str) -> i32 {
    let mut count = 0;
    let (_grid, width, height, _pos) = parse(input);
    let visited = solve(input).unwrap();
    for y in 0..width {
        for x in 0..height {
            if !visited[(y * (width) + x) as usize] {
                continue;
            }
            let mut input = input.to_owned();
            let i = (y * (width + 1) + x) as usize;
            if &input[i..i + 1] == "#" || &input[i..i + 1] == "^" {
                continue;
            }
            input.replace_range(i..=i, "#");
            if solve(&input).is_none() {
                count += 1;
            }
        }
    }
    count
}

fn idx(width: i32, pos: (i32, i32)) -> usize {
    (pos.1 * width + pos.0) as usize
}

fn solve(input: &str) -> Option<Vec<bool>> {
    let (grid, width, height, mut pos) = parse(input);
    let mut dir = (0, -1);
    let mut visited = vec![false; grid.len()];
    let mut past_states = HashSet::new();
    loop {
        if past_states.contains(&(pos, dir)) {
            return None;
        }
        past_states.insert((pos, dir));
        visited[idx(width, pos)] = true;

        let next_pos = (pos.0 + dir.0, pos.1 + dir.1);

        if next_pos.0 < 0 || next_pos.0 >= width {
            break;
        }
        if next_pos.1 < 0 || next_pos.1 >= height {
            break;
        }

        let next_pos_blocked = grid[idx(width, next_pos)];
        if next_pos_blocked {
            dir = rotate_right(dir);
            let new_next_pos = (pos.0 + dir.0, pos.1 + dir.1);

            if new_next_pos.0 < 0 || new_next_pos.0 >= width {
                break;
            }
            if new_next_pos.1 < 0 || new_next_pos.1 >= height {
                break;
            }
            pos = new_next_pos;
        } else {
            pos = next_pos;
        }
    }
    Some(visited)
}

fn rotate_right(dir: (i32, i32)) -> (i32, i32) {
    match dir {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> (Vec<bool>, i32, i32, (i32, i32)) {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let mut pos = None;
    let mut grid = vec![false; (width * height) as usize];
    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        for (x, ch) in line.chars().enumerate() {
            let x = x as i32;
            grid[(y * width + x) as usize] = ch == '#';
            if ch == '^' {
                pos = Some((x, y));
            }
        }
    }
    (grid, width, height, pos.unwrap())
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
        b.iter(|| assert_eq!(part2(black_box(&input)), 1601));
    }
}

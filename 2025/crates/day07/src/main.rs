#![feature(test)]
use std::io::{self, Read};
use util::{Grid, point};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let line_len = input.lines().next().unwrap().len();
    let grid = Grid::parse(&input[line_len * 2 + 2..], |c| c == '^');
    let mut beams = Vec::new();
    beams.push(grid.width as i32 / 2);
    let mut count = 0;
    for y in 0..grid.height as i32 {
        let mut new_beams = Vec::new();
        for &x in &beams {
            if grid.get(point(x, y)).copied() == Some(true) {
                count += 1;
                new_beams.push(x - 1);
                new_beams.push(x + 1);
            } else {
                new_beams.push(x);
            }
        }
        new_beams.dedup();
        beams = new_beams;
    }
    count
}

fn part2(input: &str) -> i64 {
    let line_len = input.lines().next().unwrap().len();
    let grid = Grid::parse(&input[line_len * 2 + 2..], |c| c == '^');
    let mut beams = Vec::new();
    beams.push((grid.width as i32 / 2, 1));
    for y in 0..grid.height as i32 {
        let mut new_beams = Vec::new();
        fn push_beam(new_beams: &mut Vec<(i32, i64)>, (x, count): (i32, i64)) {
            if !new_beams.is_empty() && new_beams.last().unwrap().0 == x {
                new_beams.last_mut().unwrap().1 += count;
            } else {
                new_beams.push((x, count));
            }
        }
        for &(x, count) in &beams {
            if grid.get(point(x, y)).copied() == Some(true) {
                push_beam(&mut new_beams, (x - 1, count));
                push_beam(&mut new_beams, (x + 1, count));
            } else {
                push_beam(&mut new_beams, (x, count));
            }
        }
        beams = new_beams;
    }
    beams.iter().map(|x| x.1).sum::<i64>()
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 3263827);
    }

    #[bench]
    #[ignore = "reason"]
    fn real_p1(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let input = input.trim();
        b.iter(|| assert_eq!(part1(test::black_box(&input)), 5977759036837));
    }

    #[bench]
    #[ignore = "reason"]
    fn real_p2(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let input = input.trim();
        b.iter(|| assert_eq!(part2(test::black_box(&input)), 9630000828442));
    }
}

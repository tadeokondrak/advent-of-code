#![feature(test)]
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn search(
    grid: &[char],
    width: i32,
    height: i32,
    dx: i32,
    dy: i32,
    x: i32,
    y: i32,
    word: &str,
) -> bool {
    if x < 0 || y < 0 || x >= width || y >= height {
        return false;
    }
    if word.is_empty() {
        return true;
    }
    let first_char = word.chars().next().unwrap();
    if grid[(y * width + x) as usize] != first_char {
        return false;
    }
    if word.len() == 1 {
        return true;
    }
    search(grid, width, height, dx, dy, x + dx, y + dy, &word[1..])
}

fn parse(input: &str) -> (Vec<char>, i32, i32) {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let mut grid = vec!['\0'; (width * height) as usize];
    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        for (x, ch) in line.chars().enumerate() {
            let x = x as i32;
            grid[(y * width + x) as usize] = ch;
        }
    }
    (grid, width, height)
}

fn part1(input: &str) -> i32 {
    let (grid, width, height) = parse(input);
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            count += search(&grid, width, height, -1, -1, x, y, "XMAS") as i32;
            count += search(&grid, width, height, -1, 0, x, y, "XMAS") as i32;
            count += search(&grid, width, height, -1, 1, x, y, "XMAS") as i32;
            count += search(&grid, width, height, 0, -1, x, y, "XMAS") as i32;
            count += search(&grid, width, height, 0, 1, x, y, "XMAS") as i32;
            count += search(&grid, width, height, 1, -1, x, y, "XMAS") as i32;
            count += search(&grid, width, height, 1, 0, x, y, "XMAS") as i32;
            count += search(&grid, width, height, 1, 1, x, y, "XMAS") as i32;
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let (grid, width, height) = parse(input);
    (0..height)
        .map(|y| {
            (0..width)
                .filter(|x| {
                    let left = search(&grid, width, height, 1, 1, x - 1, y - 1, "MAS")
                        || search(&grid, width, height, 1, 1, x - 1, y - 1, "SAM");
                    let right = search(&grid, width, height, -1, 1, x + 1, y - 1, "MAS")
                        || search(&grid, width, height, -1, 1, x + 1, y - 1, "SAM");
                    left && right
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 9);
    }

    #[bench]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(black_box(&input)), 2344));
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(black_box(&input)), 1815));
    }
}

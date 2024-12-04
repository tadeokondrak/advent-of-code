#![feature(test)]
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
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
        //dbg!(width, height, dx, dy, x, y, word);

        let first_char = word.chars().next().unwrap();
        if grid[(y * width + x) as usize] != first_char {
            return false;
        }
        if word.len() == 1 {
            return true;
        }
        search(grid, width, height, dx, dy, x + dx, y + dy, &word[1..])
    }

    fn show(
        grid: &[char],
        full_new_grid: &mut [char],
        width: i32,
        height: i32,
        dx: i32,
        dy: i32,
        mut x: i32,
        mut y: i32,
        word: &str,
    ) {
        let mut new_grid = grid.iter().copied().map(|x| '.').collect::<Vec<char>>();
        for _ in 0..word.len() {
            new_grid[(y * width + x) as usize] =
                grid[(y * width + x) as usize].to_ascii_uppercase();
            full_new_grid[(y * width + x) as usize] =
                grid[(y * width + x) as usize].to_ascii_uppercase();
            x += dx;
            y += dy;
        }
        for y in 0..height {
            for x in 0..width {
                //eprint!("{}", new_grid[(y * width + x) as usize]);
            }
            //eprintln!();
        }
        //eprintln!();
    }

    const WORD: &str = "XMAS";

    let mut count = 0;
    let mut full_new_grid = grid.iter().copied().map(|x| '.').collect::<Vec<char>>();
    for y in 0..height {
        for x in 0..width {
            let mut add = |grid: &[char], width, height, dx, dy, x, y, word| {
                //eprintln!("{:?}", (x, y, dx, dy));
                if search(&grid, width, height, dx, dy, x, y, word) {
                    show(&grid, &mut full_new_grid, width, height, dx, dy, x, y, word);
                    count += 1;
                }
            };
            add(&grid, width, height, -1, -1, x, y, WORD);
            add(&grid, width, height, -1, 0, x, y, WORD);
            add(&grid, width, height, -1, 1, x, y, WORD);
            add(&grid, width, height, 0, -1, x, y, WORD);
            add(&grid, width, height, 0, 1, x, y, WORD);
            add(&grid, width, height, 1, -1, x, y, WORD);
            add(&grid, width, height, 1, 0, x, y, WORD);
            add(&grid, width, height, 1, 1, x, y, WORD);
        }
    }

    //assert!(search(&grid, width, height, -1, -1, 4, 9, "XMAS"));

    for y in 0..height {
        for x in 0..width {
            //eprint!("{}", full_new_grid[(y * width + x) as usize]);
        }
        //eprintln!();
    }
    //eprintln!();

    count
}

fn part2(input: &str) -> i32 {
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
        //dbg!(width, height, dx, dy, x, y, word);

        let first_char = word.chars().next().unwrap();
        if grid[(y * width + x) as usize] != first_char {
            return false;
        }
        if word.len() == 1 {
            return true;
        }
        search(grid, width, height, dx, dy, x + dx, y + dy, &word[1..])
    }

    fn show(
        grid: &[char],
        full_new_grid: &mut [char],
        width: i32,
        height: i32,
        dx: i32,
        dy: i32,
        mut x: i32,
        mut y: i32,
        word: &str,
    ) {
        let mut new_grid = grid.iter().copied().map(|x| '.').collect::<Vec<char>>();
        for _ in 0..word.len() {
            new_grid[(y * width + x) as usize] =
                grid[(y * width + x) as usize].to_ascii_uppercase();
            full_new_grid[(y * width + x) as usize] =
                grid[(y * width + x) as usize].to_ascii_uppercase();
            x += dx;
            y += dy;
        }
        for y in 0..height {
            for x in 0..width {
                eprint!("{}", new_grid[(y * width + x) as usize]);
            }
            eprintln!();
        }
        eprintln!();
    }

    const WORD: &str = "MAS";

    let mut count = 0;
    let mut full_new_grid = grid.iter().copied().map(|x| '.').collect::<Vec<char>>();
    for y in 0..height {
        for x in 0..width {
            let mut check = |grid: &[char], width, height, dx, dy, x, y, word| -> bool {
                eprintln!("{:?}", (x, y, dx, dy));
                if search(&grid, width, height, dx, dy, x, y, word) {
                    //show(&grid, &mut full_new_grid, width, height, dx, dy, x, y, word);
                    true
                } else {
                    false
                }
            };
            let a = check(&grid, width, height, 1, 1, x - 1, y - 1, "MAS")
                || check(&grid, width, height, 1, 1, x - 1, y - 1, "SAM");
            let b = check(&grid, width, height, -1, 1, x + 1, y - 1, "MAS")
                || check(&grid, width, height, -1, 1, x + 1, y - 1, "SAM");
            if a && b {
                count += 1;
            }
        }
    }

    //assert!(search(&grid, width, height, -1, -1, 4, 9, "XMAS"));

    for y in 0..height {
        for x in 0..width {
            eprint!("{}", full_new_grid[(y * width + x) as usize]);
        }
        eprintln!();
    }
    eprintln!();

    count
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
}

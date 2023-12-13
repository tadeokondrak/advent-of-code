use std::collections::HashSet;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve_p1(&input));
    println!("p2: {}", solve_p2(&input));
}

struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
    flash_count: usize,
}

impl Grid {
    fn new(s: &str) -> Self {
        let mut data = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in s.lines() {
            width = line.len();
            height += 1;
            for c in line.chars() {
                data.push(c as u8 - b'0');
            }
        }
        Self {
            data,
            width,
            height,
            flash_count: 0,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<u8> {
        if !self.is_valid_point(x, y) {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        Some(self.data[y * self.width + x])
    }

    fn set(&mut self, x: i32, y: i32, n: u8) -> Option<bool> {
        if !self.is_valid_point(x, y) {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        let flash = n > 9;
        self.data[y * self.width + x] = n % 10;
        Some(flash)
    }

    fn is_valid_point(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.width as i32 {
            return false;
        }
        if y < 0 || y >= self.height as i32 {
            return false;
        }
        true
    }

    fn inc(&mut self, flashed: &mut HashSet<(i32, i32)>, x: i32, y: i32) {
        if !self.is_valid_point(x, y) {
            return;
        }
        if flashed.contains(&(x, y)) {
            return;
        }
        if let Some(true) = self.set(x, y, self.get(x, y).unwrap() + 1) {
            self.flash_count += 1;
            flashed.insert((x, y));
            self.inc(flashed, x - 1, y - 1);
            self.inc(flashed, x - 0, y - 1);
            self.inc(flashed, x + 1, y - 1);
            self.inc(flashed, x - 1, y + 0);
            self.inc(flashed, x + 1, y + 0);
            self.inc(flashed, x - 1, y + 1);
            self.inc(flashed, x - 0, y + 1);
            self.inc(flashed, x + 1, y + 1);
        }
    }

    fn step(&mut self) {
        let mut flashed = HashSet::new();
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                self.inc(&mut flashed, x, y);
            }
        }
    }

    fn is_all_zeros(&mut self) -> bool {
        self.data.iter().all(|&x| x == 0)
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                write!(f, "{}", self.get(x, y).unwrap())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve_p2(input: &str) -> usize {
    let mut grid = Grid::new(input);
    let mut day = 0;
    while !grid.is_all_zeros() {
        grid.step();
        day += 1;
    }
    day
}

fn solve_p1(input: &str) -> usize {
    let mut grid = Grid::new(input);
    for _ in 0..100 {
        grid.step();
    }
    grid.flash_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(INPUT), 1656);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_p2(INPUT), 195);
    }
}

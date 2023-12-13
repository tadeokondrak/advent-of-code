use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve(&input, Mode::Part1));
    eprintln!("p2: {}", solve(&input, Mode::Part2));
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Part1,
    Part2,
}

fn reflects_vertical(grid: &Grid<u8>, row: usize, mode: Mode) -> bool {
    let mut diff = false;
    for i in 0.. {
        if row - i == 0 {
            break;
        }

        let t = row - i - 1;
        let b = row + i;

        let t_range = match grid.data.get(t * grid.width..t * grid.width + grid.width) {
            Some(x) => x,
            None => {
                break;
            }
        };

        let b_range = match grid.data.get(b * grid.width..b * grid.width + grid.width) {
            Some(x) => x,
            None => {
                break;
            }
        };

        let check = grid.width
            - t_range
                .iter()
                .copied()
                .zip(b_range.iter().copied())
                .filter(|(a, b)| a == b)
                .count();
        match check {
            0 => continue,
            1 => {
                if diff {
                    return false;
                }
                diff = true;
            }
            _ => return false,
        }
    }

    diff == (mode == Mode::Part2)
}

fn solve(input: &str, mode: Mode) -> usize {
    let mut total = 0;
    for grid_input in input.split("\n\n") {
        let grid = Grid::new(grid_input.trim());
        let transposed = grid.transposed();
        let mut count = 0;
        for line in 1..grid.height {
            if reflects_vertical(&grid, line, mode) {
                total += 100 * line;
                count += 1;
            }
        }
        for line in 1..transposed.height {
            if reflects_vertical(&transposed, line, mode) {
                total += line;
                count += 1;
            }
        }
        assert_eq!(count, 1);
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            solve(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
                Mode::Part1
            ),
            405
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve(
                "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
                Mode::Part2
            ),
            400
        );
    }
}

struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> Grid<T> {
    fn get(&self, x: i32, y: i32) -> Option<T> {
        if !self.is_valid_point(x, y) {
            return None;
        }
        let (x, y) = (x as usize, y as usize);
        Some(self.data[y * self.width + x])
    }

    fn set(&mut self, x: i32, y: i32, v: T) {
        if !self.is_valid_point(x, y) {
            panic!();
        }
        let (x, y) = (x as usize, y as usize);
        self.data[y * self.width + x] = v;
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
}

impl Grid<u8> {
    fn new(s: &str) -> Self {
        let mut data = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in s.lines() {
            width = line.len();
            height += 1;
            for &c in line.as_bytes() {
                assert_ne!(c, b'\n');
                data.push(c);
            }
        }
        assert_eq!(data.len(), width * height);
        Self {
            data,
            width,
            height,
        }
    }

    fn transposed(&self) -> Self {
        let width = self.height;
        let height = self.width;
        let data = vec![0u8; width * height];
        let mut grid = Self {
            data,
            width,
            height,
        };
        for x in 0..self.width as i32 {
            for y in 0..self.height as i32 {
                grid.set(y, x, self.get(x, y).unwrap());
            }
        }
        grid
    }
}

impl std::fmt::Debug for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height as i32 {
            write!(f, "{y:2} ")?;
            for x in 0..self.width as i32 {
                write!(f, "{}", self.get(x, y).unwrap() as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

use fnv::FnvHasher;
use std::{
    hash::{Hash, Hasher},
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> usize {
    let mut grid = Grid::new(&input);

    let &Grid { width, height, .. } = &grid;
    tilt(&mut grid, 0..width, (1..height).rev(), 0, -1);

    let mut load = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.get(x, y) == b'O' {
                load += grid.height - y;
            }
        }
    }

    load
}

fn solve_p2(input: &str) -> usize {
    let mut grid = Grid::new(&input);

    let mut hashes = Vec::new();
    let mut times_left = 1000000000;
    while times_left > 0 {
        let &Grid { width, height, .. } = &grid;
        tilt(&mut grid, 0..width, (1..height).rev(), 0, -1);
        tilt(&mut grid, (1..width).rev(), 0..height, -1, 0);
        tilt(&mut grid, 0..width, 0..height - 1, 0, 1);
        tilt(&mut grid, 0..width - 1, 0..height, 1, 0);

        times_left -= 1;

        let mut hasher = FnvHasher::default();
        grid.data.hash(&mut hasher);
        hashes.push(hasher.finish());

        if times_left > 100
            && hashes.len() > 1
            && hashes[..hashes.len() - 1].contains(hashes.last().unwrap())
        {
            let last = hashes[..hashes.len() - 1]
                .iter()
                .copied()
                .enumerate()
                .filter(|(_, hash)| hash == hashes.last().unwrap())
                .map(|(i, _)| i)
                .last()
                .unwrap();
            let period = hashes.len() - last - 1;

            times_left = times_left % period;
        }
    }

    let mut load = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.get(x, y) == b'O' {
                load += grid.height - y;
            }
        }
    }

    load
}

fn tilt(
    grid: &mut Grid<u8>,
    x_range: impl Iterator<Item = usize> + Clone,
    y_range: impl Iterator<Item = usize> + Clone,
    dx: isize,
    dy: isize,
) {
    loop {
        let mut did_something = false;
        for x in x_range.clone() {
            for y in y_range.clone() {
                let other_x = (x as isize + dx) as usize;
                let other_y = (y as isize + dy) as usize;
                if grid.get(x, y) == b'O' && grid.get(other_x, other_y) == b'.' {
                    grid.set(x, y, b'.');
                    grid.set(other_x, other_y, b'O');
                    did_something = true;
                }
            }
        }
        if !did_something {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            solve_p1(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            ),
            136
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve_p2(
                "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
            ),
            64
        );
    }
}

struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> Grid<T> {
    fn get(&self, x: usize, y: usize) -> T {
        if !self.is_valid_point(x, y) {
            panic!("invalid point");
        }
        self.data[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, v: T) {
        if !self.is_valid_point(x, y) {
            panic!("invalid point");
        }
        let (x, y) = (x, y);
        self.data[y * self.width + x] = v;
    }

    fn is_valid_point(&self, x: usize, y: usize) -> bool {
        if x >= self.width {
            return false;
        }
        if y >= self.height {
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
}

impl std::fmt::Debug for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            write!(f, "{y:2} ")?;
            for x in 0..self.width {
                write!(f, "{}", self.get(x, y) as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

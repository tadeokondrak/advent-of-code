use std::{
    collections::HashSet,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    //eprintln!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> u32 {
    let mut pos = (0i64, 0i64);
    let mut set = HashSet::new();
    for line in input.lines() {
        let (dir, rest) = line.split_once(" ").unwrap();
        let (count, _rest) = rest.split_once(" ").unwrap();

        for _ in 0..count.parse::<i64>().unwrap() {
            set.insert(pos);

            match dir {
                "L" => pos.0 -= 1,
                "R" => pos.0 += 1,
                "U" => pos.1 -= 1,
                "D" => pos.1 += 1,
                _ => panic!(),
            }
        }
        set.insert(pos);
    }

    let x_offset = set.iter().copied().map(|(x, _y)| x).min().unwrap();
    let y_offset = set.iter().copied().map(|(_x, y)| y).min().unwrap();

    let set = set
        .iter()
        .copied()
        .map(|(x, y)| (x - x_offset, y - y_offset))
        .collect::<HashSet<_>>();

    let width = set.iter().copied().map(|(x, _y)| x).max().unwrap() + 3;
    let height = set.iter().copied().map(|(_x, y)| y).max().unwrap() + 3;
    let mut grid = Grid {
        data: vec![b'.'; width as usize * height as usize],
        width: width as usize,
        height: height as usize,
    };

    for (x, y) in set.iter().copied() {
        grid.set(x + 1, y + 1, b'#');
    }

    let fill_start = (width - 1, height - 1);

    let mut queue = Vec::new();
    queue.push(fill_start);
    while let Some((x, y)) = queue.pop() {
        if !grid.is_valid_point(x, y) {
            continue;
        }
        if grid.get(x, y) != b'.' {
            continue;
        }
        queue.push((x + 1, y));
        queue.push((x - 1, y));
        queue.push((x, y + 1));
        queue.push((x, y - 1));
        grid.set(x, y, b'!');
    }

    let mut count = 0;
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            if grid.get(x, y) != b'!' {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            solve_p1(
                "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
            ),
            62
        );
    }

    #[test]
    fn part_2() {
        //assert_eq!(solve_p2(""), 0);
    }
}
#[derive(Clone)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl Grid<u8> {
    fn get(&self, x: i64, y: i64) -> u8 {
        if !self.is_valid_point(x, y) {
            return b'.';
        }
        self.data[(y * self.width as i64 + x) as usize]
    }

    fn set(&mut self, x: i64, y: i64, v: u8) {
        if !self.is_valid_point(x, y) {
            return;
        }
        self.data[(y * self.width as i64 + x) as usize] = v;
    }

    fn is_valid_point(&self, x: i64, y: i64) -> bool {
        if x < 0 {
            return false;
        }
        if y < 0 {
            return false;
        }
        if x >= self.width as i64 {
            return false;
        }
        if y >= self.height as i64 {
            return false;
        }
        true
    }
}

impl std::fmt::Debug for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height as i64 {
            write!(f, "{y:3} ")?;
            for x in 0..self.width as i64 {
                write!(f, "{}", self.get(x, y) as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const ZERO: Point = Point { x: 0, y: 0 };
}

fn point(x: i32, y: i32) -> Point {
    Point { x, y }
}

impl std::ops::Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        *self = *self + rhs;
    }
}

fn find_move(leader: Point, follower: Point) -> Point {
    match leader - follower {
        Point {
            x: x @ (-2 | 2),
            y: y @ (0 | 1 | -1),
        } => point(x / 2, y),
        Point {
            x: x @ (0 | 1 | -1),
            y: y @ (-2 | 2),
        } => point(x, y / 2),
        Point {
            x: x @ (-2 | 2),
            y: y @ (-2 | 2),
        } => point(x / 2, y / 2),
        Point { x, y } if x.abs() == 1 && y.abs() == 1 => Point::ZERO,
        Point { x: 0, y: 0 } | Point { x: -1 | 1, y: 0 } | Point { x: 0, y: -1 | 1 } => Point::ZERO,
        other => panic!("unhandled case: {:?}", other),
    }
}

fn part1(input: &str) -> usize {
    let mut head = Point::ZERO;
    let mut tail = Point::ZERO;
    let mut visited = HashSet::new();
    visited.insert(tail);
    for line in input.lines() {
        let (dir, count) = line.split_once(" ").unwrap();
        let count = count.parse::<i32>().unwrap();
        for _ in 0..count {
            head += match dir {
                "L" => point(-1, 0),
                "R" => point(1, 0),
                "U" => point(0, 1),
                "D" => point(0, -1),
                _ => panic!(),
            };
            tail += find_move(head, tail);
            visited.insert(tail);
        }
    }
    visited.len()
}
fn part2(input: &str) -> usize {
    let mut head = Point::ZERO;
    let mut rope = [Point::ZERO; 9];
    let mut visited = HashSet::new();
    visited.insert(*rope.last().unwrap());
    for line in input.lines() {
        let (dir, count) = line.split_once(" ").unwrap();
        let count = count.parse::<i32>().unwrap();
        for _ in 0..count {
            head += match dir {
                "L" => point(-1, 0),
                "R" => point(1, 0),
                "U" => point(0, 1),
                "D" => point(0, -1),
                _ => panic!(),
            };
            for i in 0..9 {
                let leader = match i {
                    0 => head,
                    _ => rope[i - 1],
                };
                rope[i] += find_move(leader, rope[i]);
            }
            visited.insert(*rope.last().unwrap());
        }
    }
    visited.len()
}

#[cfg(test)]
mod test {
    use super::*;
    const SIMPLE_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    const COMPLEX_INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SIMPLE_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SIMPLE_INPUT), 1);
        assert_eq!(part2(COMPLEX_INPUT), 36);
    }
}
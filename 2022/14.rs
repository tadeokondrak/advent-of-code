use std::{
    collections::HashMap,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rock,
    Sand,
}

#[allow(dead_code)]
fn dump(sand: &HashMap<Point, Tile>) {
    let lowest_x = sand.keys().map(|pt| pt.x).min().unwrap();
    let highest_x = sand.keys().map(|pt| pt.x).max().unwrap();
    let highest_y = sand.keys().map(|pt| pt.y).max().unwrap();
    for y in 0..=highest_y {
        for x in lowest_x..=highest_x {
            let c = match sand.get(&point(x, y)) {
                Some(Tile::Rock) => '#',
                Some(Tile::Sand) => 'o',
                None => '.',
            };
            eprint!("{}", c);
        }
        eprintln!();
    }
}

fn part1(input: &str) -> i32 {
    let lines = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .filter(|line| !line.is_empty())
                .map(|point| sscanf::scanf!(point, "{i32},{i32}").unwrap())
                .map(|(x, y)| point(x, y))
                .collect()
        })
        .collect::<Vec<Vec<Point>>>();
    let mut grid = HashMap::<Point, Tile>::new();
    let highest_y = lines.iter().flatten().map(|pt| pt.y).max().unwrap();
    for line in lines {
        for points in line.windows(2) {
            let point_a = points[0];
            let point_b = points[1];
            let dir = point_b - point_a;
            if dir.x != 0 && dir.y == 0 {
                if dir.x > 0 {
                    for x in point_a.x..=point_b.x {
                        grid.insert(point(x, point_a.y), Tile::Rock);
                    }
                } else if dir.x < 0 {
                    for x in point_b.x..=point_a.x {
                        grid.insert(point(x, point_a.y), Tile::Rock);
                    }
                }
            } else if dir.y != 0 && dir.x == 0 {
                if dir.y > 0 {
                    for y in point_a.y..=point_b.y {
                        grid.insert(point(point_a.x, y), Tile::Rock);
                    }
                } else if dir.y < 0 {
                    for y in point_b.y..=point_a.y {
                        grid.insert(point(point_a.x, y), Tile::Rock);
                    }
                }
            } else {
                panic!("unknown direction: {:?}", dir);
            }
        }
    }
    'outer: loop {
        let mut cur_pos = point(500, 0);
        'inner: loop {
            if cur_pos.y >= highest_y {
                break 'outer;
            }

            let below_point = cur_pos + point(0, 1);
            let below = grid.get(&below_point);
            if below.is_none() {
                cur_pos = below_point;
                continue 'inner;
            }

            let below_left_point = cur_pos + point(-1, 1);
            let below_left = grid.get(&below_left_point);
            if below_left.is_none() {
                cur_pos = below_left_point;
                continue 'inner;
            }

            let below_right_point = cur_pos + point(1, 1);
            let below_right = grid.get(&below_right_point);
            if below_right.is_none() {
                cur_pos = below_right_point;
                continue 'inner;
            }

            grid.insert(cur_pos, Tile::Sand);
            break 'inner;
        }
    }
    grid.values()
        .copied()
        .filter(|&t| t == Tile::Sand)
        .count()
        .try_into()
        .unwrap()
}

fn part2(input: &str) -> i32 {
    let lines = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .filter(|line| !line.is_empty())
                .map(|point| sscanf::scanf!(point, "{i32},{i32}").unwrap())
                .map(|(x, y)| point(x, y))
                .collect()
        })
        .collect::<Vec<Vec<Point>>>();
    let mut grid = HashMap::<Point, Tile>::new();
    let highest_y = lines.iter().flatten().map(|pt| pt.y).max().unwrap();
    let floor = highest_y + 2;
    for line in lines {
        for points in line.windows(2) {
            let point_a = points[0];
            let point_b = points[1];
            let dir = point_b - point_a;
            if dir.x != 0 && dir.y == 0 {
                if dir.x > 0 {
                    for x in point_a.x..=point_b.x {
                        grid.insert(point(x, point_a.y), Tile::Rock);
                    }
                } else if dir.x < 0 {
                    for x in point_b.x..=point_a.x {
                        grid.insert(point(x, point_a.y), Tile::Rock);
                    }
                }
            } else if dir.y != 0 && dir.x == 0 {
                if dir.y > 0 {
                    for y in point_a.y..=point_b.y {
                        grid.insert(point(point_a.x, y), Tile::Rock);
                    }
                } else if dir.y < 0 {
                    for y in point_b.y..=point_a.y {
                        grid.insert(point(point_a.x, y), Tile::Rock);
                    }
                }
            } else {
                panic!("unknown direction: {:?}", dir);
            }
        }
    }
    'outer: loop {
        let mut cur_pos = point(500, 0);
        'inner: loop {
            let below_point = cur_pos + point(0, 1);
            let below = grid.get(&below_point);
            if below.is_none() && below_point.y < floor {
                cur_pos = below_point;
                continue 'inner;
            }

            let below_left_point = cur_pos + point(-1, 1);
            let below_left = grid.get(&below_left_point);
            if below_left.is_none() && below_left_point.y < floor {
                cur_pos = below_left_point;
                continue 'inner;
            }

            let below_right_point = cur_pos + point(1, 1);
            let below_right = grid.get(&below_right_point);
            if below_right.is_none() && below_right_point.y < floor {
                cur_pos = below_right_point;
                continue 'inner;
            }

            grid.insert(cur_pos, Tile::Sand);
            if cur_pos == point(500, 0) {
                break 'outer;
            }
            break 'inner;
        }
    }
    grid.values()
        .copied()
        .filter(|&t| t == Tile::Sand)
        .count()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 93);
    }
}

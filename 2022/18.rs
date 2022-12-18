use std::{
    collections::HashSet,
    fmt,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3 {
    const X: Point3 = Point3 { x: 1, y: 0, z: 0 };
    const Y: Point3 = Point3 { x: 0, y: 1, z: 0 };
    const Z: Point3 = Point3 { x: 0, y: 0, z: 1 };
    const NEG_X: Point3 = Point3 { x: -1, y: 0, z: 0 };
    const NEG_Y: Point3 = Point3 { x: 0, y: -1, z: 0 };
    const NEG_Z: Point3 = Point3 { x: 0, y: 0, z: -1 };
}

fn point3(x: i32, y: i32, z: i32) -> Point3 {
    Point3 { x, y, z }
}

impl fmt::Debug for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Point3")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .finish()
    }
}

impl std::ops::Add<Point3> for Point3 {
    type Output = Self;

    fn add(self, rhs: Point3) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

fn part1(input: &str) -> i32 {
    let lava: HashSet<Point3> = input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|line| {
            let mut fields = line.split(",");
            Point3 {
                x: fields.next().unwrap().parse().unwrap(),
                y: fields.next().unwrap().parse().unwrap(),
                z: fields.next().unwrap().parse().unwrap(),
            }
        })
        .collect();
    let mut surface_area = 0;
    let directions = [
        Point3::X,
        Point3::NEG_X,
        Point3::Y,
        Point3::NEG_Y,
        Point3::Z,
        Point3::NEG_Z,
    ];
    for cube in lava.iter().copied() {
        for direction in directions {
            if !lava.contains(&(cube + direction)) {
                surface_area += 1;
            }
        }
    }
    surface_area
}

fn part2(input: &str) -> i32 {
    let lava: HashSet<Point3> = input
        .lines()
        .filter(|&line| !line.is_empty())
        .map(|line| {
            let mut fields = line.split(",");
            Point3 {
                x: fields.next().unwrap().parse().unwrap(),
                y: fields.next().unwrap().parse().unwrap(),
                z: fields.next().unwrap().parse().unwrap(),
            }
        })
        .collect();
    let directions = [
        Point3::X,
        Point3::NEG_X,
        Point3::Y,
        Point3::NEG_Y,
        Point3::Z,
        Point3::NEG_Z,
    ];
    let mut air: HashSet<Point3> = HashSet::new();
    let mut reachable_lava: HashSet<Point3> = HashSet::new();
    let mut stack = Vec::new();
    let start = point3(0, 0, 0);
    assert!(!lava.contains(&start));
    stack.push(start);
    while let Some(cube) = stack.pop() {
        if !lava.contains(&cube) {
            air.insert(cube);
            stack.extend(
                directions
                    .iter()
                    .copied()
                    .map(|pt| cube + pt)
                    .filter(|pt| [pt.x, pt.y, pt.z].into_iter().all(|x| x >= -5 && x <= 25))
                    .filter(|pt| !air.contains(&pt)),
            );
            reachable_lava.extend(
                directions
                    .iter()
                    .copied()
                    .map(|pt| cube + pt)
                    .filter(|pt| lava.contains(&pt)),
            );
        }
    }
    let mut outer_surface_area = 0;
    for cube in reachable_lava.iter().copied() {
        for direction in directions {
            if air.contains(&(cube + direction)) {
                outer_surface_area += 1;
            }
        }
    }
    outer_surface_area
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 64);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 58);
    }
}

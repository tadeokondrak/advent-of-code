mod util;
use util::{point, Point};

use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input, 2000000));
    println!("Part 2: {}", part2(&input, 4000000));
}

fn parse(input: &str) -> (Vec<(Point, i32)>, HashSet<Point>) {
    let mut sensors = Vec::new();
    let mut beacons = HashSet::new();
    for line in input.lines().filter(|line| !line.is_empty()) {
        let (sx, sy, bx, by) = sscanf::sscanf!(
            line.trim(),
            "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}"
        )
        .unwrap();
        let sensor = point(sx, sy);
        let beacon = point(bx, by);
        let radius = sensor.distance_from(beacon);
        sensors.push((sensor, radius));
        beacons.insert(beacon);
    }
    (sensors, beacons)
}

fn part1(input: &str, row: i32) -> i32 {
    let (sensors, beacons) = parse(input);
    let min_x = sensors
        .iter()
        .map(|(sensor, radius)| sensor.x - radius)
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|(sensor, radius)| sensor.x + radius)
        .max()
        .unwrap();
    (min_x..=max_x)
        .filter(|&x| {
            sensors.iter().copied().any(|(sensor, radius)| {
                let pt = point(x, row);
                sensor.distance_from(pt) <= radius && !beacons.contains(&pt)
            })
        })
        .count() as i32
}

fn part2(input: &str, max: i32) -> i64 {
    let (sensors, _beacons) = parse(input);
    for (sensor, radius) in sensors.iter().copied() {
        let new_radius = radius + 1;
        for i in 0..=new_radius {
            let x = new_radius - i;
            let y = i;
            let dirs = [
                sensor + point(x, y),
                sensor + point(-x, y),
                sensor + point(x, -y),
                sensor + point(-x, -y),
            ];
            for cand in dirs
                .iter()
                .copied()
                .filter(|&pt| pt.x >= 0 && pt.y >= 0 && pt.x <= max && pt.y <= max)
                .filter(|&pt| {
                    sensors
                        .iter()
                        .copied()
                        .all(|(beacon, radius)| beacon.distance_from(pt) > radius)
                })
            {
                return (cand.x as i64) * 4000000 + (cand.y as i64);
            }
        }
    }
    panic!();
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT, 10), 26);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT, 20), 56000011);
    }
}

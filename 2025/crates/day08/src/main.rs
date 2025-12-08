#![feature(test)]
use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input, 1000));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str, mut count: i64) -> i64 {
    let points = input
        .lines()
        .map(|line| {
            let mut it = line.split(',');
            let x = it.next().unwrap().parse::<i64>().unwrap();
            let y = it.next().unwrap().parse::<i64>().unwrap();
            let z = it.next().unwrap().parse::<i64>().unwrap();
            [x, y, z]
        })
        .collect::<Vec<[i64; 3]>>();
    let mut distances: Vec<Vec<i64>> = Vec::new();
    for _ in 0..points.len() {
        distances.push(vec![0; points.len()]);
    }
    let mut heap = BinaryHeap::new();
    for i in 0..points.len() {
        for j in 0..points.len() {
            if i == j {
                continue;
            }
            let dist = (points[i][0] - points[j][0]) * (points[i][0] - points[j][0])
                + (points[i][1] - points[j][1]) * (points[i][1] - points[j][1])
                + (points[i][2] - points[j][2]) * (points[i][2] - points[j][2]);
            distances[i][j] = dist;
            if i < j {
                heap.push(Reverse((dist, i, j)));
            }
        }
    }
    let mut nets = Vec::with_capacity(points.len());
    for i in 0..points.len() {
        nets.push(i);
    }
    while count > 0 {
        let Reverse((_dist, i, j)) = heap.pop().unwrap();
        let minnet = nets[i].min(nets[j]);
        let maxnet = nets[i].max(nets[j]);
        for it in nets.iter_mut() {
            if *it == maxnet {
                *it = minnet;
            }
        }
        count -= 1;
    }
    let mut net_sizes = vec![0; nets.len() as usize];
    for &net in &nets {
        net_sizes[net as usize] += 1;
    }
    net_sizes.sort_by(|a, b| a.cmp(b).reverse());
    net_sizes.iter().take(3).product()
}

fn part2(input: &str) -> i64 {
    let points = input
        .lines()
        .map(|line| {
            let mut it = line.split(',');
            let x = it.next().unwrap().parse::<i64>().unwrap();
            let y = it.next().unwrap().parse::<i64>().unwrap();
            let z = it.next().unwrap().parse::<i64>().unwrap();
            [x, y, z]
        })
        .collect::<Vec<[i64; 3]>>();
    let mut distances: Vec<Vec<i64>> = Vec::new();
    for _ in 0..points.len() {
        distances.push(vec![0; points.len()]);
    }
    let mut heap = BinaryHeap::new();
    for i in 0..points.len() {
        for j in 0..points.len() {
            if i == j {
                continue;
            }
            let dist = (points[i][0] - points[j][0]) * (points[i][0] - points[j][0])
                + (points[i][1] - points[j][1]) * (points[i][1] - points[j][1])
                + (points[i][2] - points[j][2]) * (points[i][2] - points[j][2]);
            distances[i][j] = dist;

            if i < j {
                heap.push(Reverse((dist, i, j)));
            }
        }
    }
    let mut nets = Vec::with_capacity(points.len());
    for i in 0..points.len() {
        nets.push(i);
    }
    let mut result = None;
    while !nets.is_sorted() || nets.first() != nets.last() {
        let Reverse((_dist, i, j)) = heap.pop().unwrap();
        let minnet = nets[i].min(nets[j]);
        let maxnet = nets[i].max(nets[j]);
        for it in nets.iter_mut() {
            if *it == maxnet {
                *it = minnet;
            }
        }
        result = Some(points[i][0] * points[j][0]);
    }
    result.unwrap()
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT, 10), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 25272);
    }

    #[bench]
    fn real_p1(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let input = input.trim();
        b.iter(|| assert_eq!(part1(test::black_box(&input), 1000), 129564));
    }

    #[bench]
    fn real_p2(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let input = input.trim();
        b.iter(|| assert_eq!(part2(test::black_box(&input)), 42047840));
    }
}

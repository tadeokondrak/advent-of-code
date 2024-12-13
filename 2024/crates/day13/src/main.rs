#![feature(test)]
use std::io::{self, Read};
use util::{offset64 as offset, point64 as point, Offset64 as Offset, Point64 as Point};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    run(input, 0)
}

fn run(input: &str, coord_offset: i64) -> i64 {
    let mut total = 0;
    for machine_text in input.split("\n\n") {
        let mut lines = machine_text.lines();
        let a = lines
            .next()
            .unwrap()
            .strip_prefix("Button A: X+")
            .unwrap()
            .split_once(", Y+")
            .map(|(x, y)| offset(x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();
        let b = lines
            .next()
            .unwrap()
            .strip_prefix("Button B: X+")
            .unwrap()
            .split_once(", Y+")
            .map(|(x, y)| offset(x.parse().unwrap(), y.parse().unwrap()))
            .unwrap();
        let prize = lines
            .next()
            .unwrap()
            .strip_prefix("Prize: X=")
            .unwrap()
            .split_once(", Y=")
            .map(|(x, y)| {
                point(x.parse().unwrap(), y.parse().unwrap()) + offset(coord_offset, coord_offset)
            })
            .unwrap();
        if let Some(solution) = solve(a, b, prize) {
            total += solution;
        }
    }
    total
}

fn part2(input: &str) -> i64 {
    run(input, 10000000000000)
}

fn solve(a: Offset, b: Offset, prize: Point) -> Option<i64> {
    fn det(r: impl Into<[i64; 2]>, s: impl Into<[i64; 2]>) -> i64 {
        let (r, s) = (r.into(), s.into());
        r[0] * s[1] - r[1] * s[0]
    }
    let origin = point(0, 0);
    let x = det(prize, b) / det(a, b);
    let y = det(a, prize) / det(a, b);
    if origin + a * x + b * y == prize {
        Some(x * 3 + y)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part1() {
        assert_eq!(
            solve(offset(94, 34), offset(22, 67), point(8400, 5400)),
            Some(280)
        );
        assert_eq!(part1(TEST_INPUT), 480);
    }

    #[bench]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(black_box(&input)), 39748));
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(black_box(&input)), 74478585072604));
    }
}

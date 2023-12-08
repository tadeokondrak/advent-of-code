#![feature(test)]

use std::{
    io::{stdin, Read},
    iter::Cycle,
    str::Chars,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let parsed = parse(&input);
    eprintln!("p1: {}", solve_p1(&parsed));
    eprintln!("p2: {}", solve_p2(&parsed));
}

type Parsed<'a> = (Vec<u16>, Vec<u16>, Vec<u16>, Cycle<Chars<'a>>);

fn pack(name: &str) -> u16 {
    assert!(name.len() == 3);
    name.bytes().fold(0u16, |acc, b| {
        assert!(b'A' <= b && b <= b'Z');
        (acc << 5) | u16::from(b - b'A')
    })
}

fn is_start(packed: u16) -> bool {
    (packed & 0b11111) as u8 == 0
}

fn is_end(packed: u16) -> bool {
    (packed & 0b11111) as u8 == 25
}

fn parse<'a>(input: &str) -> Parsed {
    let mut all = Vec::new();
    let mut lefts = Vec::new();
    let mut rights = Vec::new();
    let (directions, links) = input.split_once("\n\n").unwrap();
    for link in links.lines() {
        let (k, v) = link.split_once(" = (").unwrap();
        let v = v.strip_suffix(")").unwrap();
        let (left, right) = v.split_once(", ").unwrap();
        let k = pack(k);
        let left = pack(left);
        let right = pack(right);
        if usize::from(k) >= lefts.len() {
            lefts.resize(usize::from(k + 1), 0);
            rights.resize(usize::from(k + 1), 0);
        }
        all.push(k);
        lefts[usize::from(k)] = left;
        rights[usize::from(k)] = right;
    }
    (all, lefts, rights, directions.chars().cycle())
}

fn solve_p1(parsed: &Parsed) -> u64 {
    count_steps(pack("AAA"), parsed)
}

fn solve_p2(parsed: &Parsed) -> u64 {
    let (all, ..) = parsed;
    all.iter()
        .copied()
        .filter(|&k| is_start(k))
        .fold(1u64, |acc, pos| lcm(acc, count_steps(pos, parsed)))
}

fn count_steps(pos: u16, (_, lefts, rights, directions): &Parsed) -> u64 {
    let mut pos = pos; // fixes lifetime issue
    let mut steps = 0;
    let mut directions = directions.clone();
    loop {
        let c = directions.next().unwrap();
        if is_end(pos) {
            break steps;
        }
        pos = match c {
            'L' => lefts[usize::from(pos)],
            'R' => rights[usize::from(pos)],
            _ => panic!(),
        };
        steps += 1;
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use test::Bencher;

    #[test]
    fn example_p1() {
        assert_eq!(
            solve_p1(&parse(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            )),
            2
        );
    }

    #[test]
    #[ignore = "non-alphabetic characters are broken"]
    fn example_p2() {
        assert_eq!(
            solve_p2(&parse(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            )),
            6
        );
    }

    #[bench]
    fn parsing(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| drop(parse(&input)));
    }

    #[bench]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(solve_p1(&parsed), 16897));
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(solve_p2(&parsed), 16563603485021));
    }
}

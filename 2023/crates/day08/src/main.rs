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

type Parsed<'a> = (
    Vec<u16>,
    &'static [u16; pack("ZZZ") as usize + 1],
    &'static [u16; pack("ZZZ") as usize + 1],
    Cycle<Chars<'a>>,
);

const fn pack(name: &str) -> u16 {
    assert!(name.len() == 3);
    let mut acc = 0;
    let bytes = name.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        assert!(b'A' <= b && b <= b'Z');
        acc = (acc << 5) | (b - b'A') as u16;
        i += 1;
    }
    acc
}

fn is_start(packed: u16) -> bool {
    (packed & 0b11111) as u8 == 0
}

fn is_end(packed: u16) -> bool {
    (packed & 0b11111) as u8 == 25
}

fn parse<'a>(input: &str) -> Parsed {
    let (directions, links) = input.split_once("\n\n").unwrap();
    let mut all = Vec::new();
    let mut lefts = [0; pack("ZZZ") as usize + 1];
    let mut rights = [0; pack("ZZZ") as usize + 1];
    for link in links.lines() {
        let (k, v) = link.split_once(" = (").unwrap();
        let v = v.strip_suffix(")").unwrap();
        let (left, right) = v.split_once(", ").unwrap();
        let k = pack(k);
        let left = pack(left);
        let right = pack(right);
        all.push(k);
        lefts[usize::from(k)] = left;
        rights[usize::from(k)] = right;
    }
    (
        all,
        Box::leak(Box::new(lefts)),
        Box::leak(Box::new(rights)),
        directions.chars().cycle(),
    )
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

fn count_steps(mut pos: u16, (_, lefts, rights, directions): &Parsed) -> u64 {
    let lefts = *lefts;
    let rights = *rights;
    let mut steps = 0;
    let mut directions = directions.clone();
    while !is_end(pos) {
        pos = match directions.next().unwrap() {
            'L' => lefts[usize::from(pos)],
            'R' => rights[usize::from(pos)],
            _ => panic!(),
        };
        steps += 1;
    }
    steps
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
    use std::hint::black_box;
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
        b.iter(|| assert_eq!(solve_p1(black_box(&parsed)), 16897));
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let parsed = parse(&input);
        b.iter(|| assert_eq!(solve_p2(black_box(&parsed)), 16563603485021));
    }
}

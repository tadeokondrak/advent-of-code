#![feature(test)]

use fnv::FnvHashMap as HashMap;
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

type Parsed<'a> = (HashMap<&'a str, (&'a str, &'a str)>, Cycle<Chars<'a>>);

fn parse<'a>(input: &str) -> Parsed {
    let mut edges = HashMap::default();
    let (directions, links) = input.split_once("\n\n").unwrap();
    for link in links.lines() {
        let (k, v) = link.split_once(" = (").unwrap();
        let v = v.strip_suffix(")").unwrap();
        let (left, right) = v.split_once(", ").unwrap();
        edges.insert(k, (left, right));
    }
    (edges, directions.chars().cycle())
}

fn solve_p1((edges, directions): &Parsed) -> u64 {
    count_steps("AAA", &edges, directions.clone())
}

fn solve_p2((edges, directions): &Parsed) -> u64 {
    edges
        .keys()
        .filter(|k| k.ends_with("A"))
        .fold(1u64, |acc, pos| {
            lcm(acc, count_steps(pos, &edges, directions.clone()))
        })
}

fn count_steps(
    pos: &str,
    edges: &HashMap<&str, (&str, &str)>,
    mut directions: impl Iterator<Item = char>,
) -> u64 {
    let mut pos = pos; // fixes lifetime issue
    let mut steps = 0;
    loop {
        let c = directions.next().unwrap();
        if pos.ends_with("Z") {
            break steps;
        }
        pos = match c {
            'L' => &edges[pos].0,
            'R' => &edges[pos].1,
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

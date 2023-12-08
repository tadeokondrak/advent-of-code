use std::{
    collections::HashMap,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

fn parse<'a>(
    input: &'a str,
) -> (
    HashMap<String, (String, String)>,
    impl Iterator<Item = char> + Clone + 'a,
) {
    let mut edges = HashMap::new();
    let (directions, links) = input.split_once("\n\n").unwrap();
    for link in links.lines() {
        let (k, v) = link.split_once(" = (").unwrap();
        let v = v.strip_suffix(")").unwrap();
        let (left, right) = v.split_once(", ").unwrap();
        edges.insert(k.to_string(), (left.to_string(), right.to_string()));
    }
    (edges, directions.chars().cycle())
}

fn solve_p1(input: &str) -> u64 {
    let (edges, directions) = parse(input);
    count_steps("AAA", &edges, directions)
}

fn solve_p2(input: &str) -> u64 {
    let (edges, directions) = parse(input);
    edges
        .keys()
        .filter(|k| k.ends_with("A"))
        .fold(1u64, |acc, pos| {
            lcm(acc, count_steps(pos, &edges, directions.clone()))
        })
}

fn count_steps(
    pos: &str,
    edges: &HashMap<String, (String, String)>,
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
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            solve_p1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            2
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve_p2(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            6
        );
    }
}

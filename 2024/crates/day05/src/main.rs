#![feature(test)]
use std::{
    collections::BTreeSet,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let (updates, edges) = parse(input);
    updates
        .iter()
        .filter(|line| is_ordered(&edges, line))
        .map(|line| line[line.len() / 2])
        .sum()
}

fn part2(input: &str) -> i32 {
    let (updates, edges) = parse(input);
    updates
        .iter()
        .filter(|line| !is_ordered(&edges, &line))
        .map(|line| reorder(&edges, &line))
        .sum()
}

fn is_ordered(edges: &BTreeSet<(i32, i32)>, line: &[i32]) -> bool {
    for i in 0..line.len() - 1 {
        let from = line[i];
        let to = line[i + 1];
        if !edges.contains(&(from, to)) {
            return false;
        }
    }
    true
}

// returns the middle element of the reordered line
fn reorder(edges: &BTreeSet<(i32, i32)>, line: &[i32]) -> i32 {
    let edges: BTreeSet<(i32, i32)> = edges
        .iter()
        .copied()
        .filter(|(from, to)| line.contains(&from) && line.contains(&to))
        .collect();
    fn go(edges: &BTreeSet<(i32, i32)>, path: &mut Vec<i32>, fulllen: usize) -> Option<i32> {
        if path.len() == fulllen {
            return Some(path[path.len() / 2]);
        }
        let prev = path.last().copied().unwrap();
        for &(_, to) in edges.range((prev, 0)..=(prev, i32::MAX)) {
            if path.contains(&to) {
                continue;
            }
            path.push(to);
            if let Some(result) = go(edges, path, fulllen) {
                return Some(result);
            }
            path.pop();
        }
        None
    }
    let mut path = Vec::new();
    for &start_vertex in line {
        path.push(start_vertex);
        if let Some(result) = go(&edges, &mut path, line.len()) {
            return result;
        }
        path.pop();
    }
    panic!("ordering not found")
}

fn parse(input: &str) -> (Vec<Vec<i32>>, BTreeSet<(i32, i32)>) {
    let (edges, updates) = input.split_once("\n\n").unwrap();
    let edges: BTreeSet<(i32, i32)> = edges
        .lines()
        .map(|line| {
            let (l, r) = line.split_once("|").unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect();
    let updates = updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|it| it.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect();
    (updates, edges)
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13`
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 123);
    }

    #[bench]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(black_box(&input)), 6949));
    }

    #[bench]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(black_box(&input)), 4145));
    }
}

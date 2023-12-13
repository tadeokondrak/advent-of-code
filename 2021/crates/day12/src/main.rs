use std::borrow::Borrow;
use std::collections::{BTreeMap, BTreeSet};
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve_p1(&input));
    println!("p2: {}", solve_p2(&input));
}

// some optimization because it was pretty slow originally
struct OverlayBTreeMap<'a, K, V> {
    parent: Option<&'a OverlayBTreeMap<'a, K, V>>,
    map: BTreeMap<K, V>,
}

impl<'a, K, V> OverlayBTreeMap<'a, K, V> {
    fn new(parent: Option<&'a OverlayBTreeMap<'a, K, V>>) -> Self {
        Self {
            parent,
            map: BTreeMap::new(),
        }
    }

    fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord,
    {
        match self.map.get(key) {
            Some(val) => Some(val),
            None => self.parent.as_ref().map_or(None, |parent| parent.get(key)),
        }
    }
}

fn num_paths_from(
    caves: &BTreeMap<String, BTreeSet<String>>,
    visited: &OverlayBTreeMap<String, usize>,
    start: &str,
    visit_small_twice: bool,
) -> usize {
    let mut sum = 0;
    let mut visited = OverlayBTreeMap::new(Some(visited));
    if start.chars().all(|x| x.is_ascii_lowercase()) {
        *visited.map.entry(start.to_owned()).or_default() += 1;
    }
    for choice in &caves[start] {
        sum += match &**choice {
            "start" => 0,
            "end" => 1,
            _ => match visited.get(choice) {
                Some(&n) if n >= 2 => 0,
                Some(&n) if n >= 1 && !visit_small_twice => 0,
                Some(_) => num_paths_from(caves, &visited, choice, false),
                None => num_paths_from(caves, &visited, choice, visit_small_twice),
            },
        }
    }
    sum
}

fn solve_p2(input: &str) -> usize {
    let mut caves = BTreeMap::<String, BTreeSet<String>>::new();
    for line in input.lines() {
        let (from, to) = line.split_once('-').unwrap();
        caves
            .entry(from.to_owned())
            .or_default()
            .insert(to.to_owned());
        caves
            .entry(to.to_owned())
            .or_default()
            .insert(from.to_owned());
    }
    num_paths_from(&caves, &OverlayBTreeMap::new(None), "start", true)
}

fn solve_p1(input: &str) -> usize {
    let mut caves = BTreeMap::<String, BTreeSet<String>>::new();
    for line in input.lines() {
        let (from, to) = line.split_once('-').unwrap();
        caves
            .entry(from.to_owned())
            .or_default()
            .insert(to.to_owned());
        caves
            .entry(to.to_owned())
            .or_default()
            .insert(from.to_owned());
    }
    num_paths_from(&caves, &OverlayBTreeMap::new(None), "start", false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_A: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const INPUT_B: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(INPUT_A), 19);
        assert_eq!(solve_p1(INPUT_B), 10);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_p2(INPUT_A), 103);
        assert_eq!(solve_p2(INPUT_B), 36);
    }
}

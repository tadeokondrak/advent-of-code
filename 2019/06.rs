use std::collections::{HashMap, HashSet};
use std::convert::identity;
use std::io::{stdin, BufRead};

fn find<'a>(map: &'a HashMap<String, HashSet<String>>, search: &'a str) -> Option<&'a str> {
    map.iter()
        .filter_map(|(obj, orbits)| orbits.iter().find(|&s| s == search).map(|_| obj as &str))
        .next()
}

fn check_orbits(map: &HashMap<String, HashSet<String>>, object: &str) -> usize {
    map[object]
        .iter()
        .map(|orbit| check_orbits(map, orbit) + 1)
        .sum()
}

fn check_distance<'a>(
    map: &'a HashMap<String, HashSet<String>>,
    seen: &mut HashSet<&'a str>,
    a: &'a str,
    b: &str,
) -> Option<usize> {
    map[a]
        .iter()
        .map(|x| x as &str)
        .chain(find(&map, &a))
        .map(|obj| {
            if obj == b {
                Some(1)
            } else if !seen.contains(obj) {
                seen.insert(a);
                check_distance(map, seen, obj, &b).map(|x| x + 1)
            } else {
                None
            }
        })
        .filter_map(identity)
        .min()
}

fn main() {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut split = line.split(')');
            (
                split.next().unwrap().to_string(),
                split.next().unwrap().to_string(),
            )
        })
        .for_each(|(a, b)| {
            map.entry(a).or_insert_with(HashSet::new).insert(b.clone());
            map.entry(b).or_insert_with(HashSet::new);
        });

    let p1: usize = map.keys().map(|obj| check_orbits(&map, obj)).sum();

    let p2 = check_distance(
        &map,
        &mut HashSet::new(),
        find(&map, "YOU").unwrap(),
        find(&map, "SAN").unwrap(),
    )
    .unwrap();

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn contains(
    rules: &HashMap<String, Vec<(usize, String)>>,
    container: &str,
    containee: &str,
) -> bool {
    rules[container]
        .iter()
        .any(|(_, elem)| elem == containee || contains(rules, elem, containee))
}

fn num_children(rules: &HashMap<String, Vec<(usize, String)>>, container: &str) -> usize {
    rules[container]
        .iter()
        .map(|(count, elem)| count + count * num_children(rules, elem))
        .sum()
}

fn main() {
    let rules = stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut it = line.split(" bags contain ");
            let bag = it.next().unwrap();
            let rule = it
                .next()
                .unwrap()
                .strip_suffix(".")
                .unwrap()
                .split(", ")
                .map(|s| s.strip_suffix(" bag").unwrap_or(s))
                .map(|s| s.strip_suffix(" bags").unwrap_or(s))
                .filter(|&x| x != "no other")
                .map(|s| {
                    let mut it = s.splitn(2, ' ');
                    (
                        it.next().unwrap().parse().unwrap(),
                        it.next().unwrap().to_owned(),
                    )
                })
                .collect();
            (bag.to_owned(), rule)
        })
        .collect::<HashMap<String, Vec<(usize, String)>>>();
    let p1 = rules
        .keys()
        .filter(|color| contains(&rules, color, "shiny gold"))
        .count();
    let p2 = num_children(&rules, "shiny gold");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

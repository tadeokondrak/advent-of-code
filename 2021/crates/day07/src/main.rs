use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve_p1(&input));
    println!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> i32 {
    let crabs = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let biggest_crab = crabs.iter().copied().max().unwrap();
    let mut list = Vec::new();
    for i in 0..biggest_crab {
        let mut fuel_needed = 0;
        for &crab in &crabs {
            fuel_needed += (crab - i).abs();
        }
        list.push((fuel_needed, i));
    }
    list.sort();
    list[0].0
}

fn solve_p2(input: &str) -> i32 {
    let crabs = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let biggest_crab = crabs.iter().copied().max().unwrap();
    let mut list = Vec::new();
    for i in 0..biggest_crab {
        let mut fuel_needed = 0;
        for &crab in &crabs {
            let dist = (crab - i).abs();
            let cost = dist * (dist + 1) / 2;
            fuel_needed += cost;
        }
        list.push((fuel_needed, i));
    }
    list.sort();
    list[0].0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(INPUT), 37);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_p2(INPUT), 168);
    }
}

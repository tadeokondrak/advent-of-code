use std::{
    collections::HashSet,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn priority(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - b'a' + 1,
        'A'..='Z' => c as u8 - b'A' + 27,
        _ => panic!(),
    }
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.trim();
        let (left, right) = line.split_at(line.len() / 2);
        let left_chars = left.chars().collect::<HashSet<char>>();
        let right_chars = right.chars().collect::<HashSet<char>>();
        let both_chars = left_chars.intersection(&right_chars);
        sum += both_chars.copied().map(|c| priority(c)).sum::<u8>() as i32;
    }
    sum
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let groups = input
        .lines()
        .map(|line| line.trim().to_owned())
        .collect::<Vec<String>>();
    for group in groups.chunks_exact(3) {
        let a = group[0].chars().collect::<HashSet<char>>();
        let b = group[1].chars().collect::<HashSet<char>>();
        let c = group[2].chars().collect::<HashSet<char>>();
        let a_b = a.intersection(&b).copied().collect::<HashSet<char>>();
        let a_b_c = a_b.intersection(&c).copied().collect::<HashSet<char>>();
        sum += priority(a_b_c.iter().copied().next().unwrap()) as i32;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 70);
    }
}

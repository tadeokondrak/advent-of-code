use std::{
    collections::BinaryHeap,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap()
}

fn part2(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<i32>().unwrap()).sum())
        .collect::<BinaryHeap<i32>>()
        .iter()
        .take(3)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 45000);
    }
}

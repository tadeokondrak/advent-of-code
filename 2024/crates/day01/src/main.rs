use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut left = input
        .lines()
        .map(|line| line.split("   ").next().unwrap().parse().unwrap())
        .collect::<Vec<i32>>();
    let mut right = input
        .lines()
        .map(|line| line.split("   ").skip(1).next().unwrap().parse().unwrap())
        .collect::<Vec<i32>>();
    left.sort();
    right.sort();
    let mut res = 0;
    for i in 0..left.len() {
        res += (left[i] - right[i]).abs();
    }
    res
}

fn part2(input: &str) -> i32 {
    let mut left = input
        .lines()
        .map(|line| line.split("   ").next().unwrap().parse().unwrap())
        .collect::<Vec<i32>>();
    let mut right = input
        .lines()
        .map(|line| line.split("   ").skip(1).next().unwrap().parse().unwrap())
        .collect::<Vec<i32>>();
    let mut res = 0;
    for i in 0..left.len() {
        res += left[i] * right.iter().filter(|&&rightx| rightx == left[i]).count() as i32;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 31);
    }
}

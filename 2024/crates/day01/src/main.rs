use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let l = input
        .lines()
        .map(|line| line.split("   ").next().unwrap().parse().unwrap())
        .collect();
    let r = input
        .lines()
        .map(|line| line.split("   ").skip(1).next().unwrap().parse().unwrap())
        .collect();
    (l, r)
}

fn part1(input: &str) -> u32 {
    let (mut l, mut r) = parse(input);
    l.sort();
    r.sort();
    l.into_iter()
        .zip(r.into_iter())
        .map(|(lv, rv)| lv.abs_diff(rv))
        .sum()
}

fn part2(input: &str) -> u32 {
    let (l, r) = parse(input);
    l.into_iter()
        .map(|lv| {
            let freq = r.iter().copied().filter(|&rv| rv == lv).count() as u32;
            lv * freq
        })
        .sum()
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

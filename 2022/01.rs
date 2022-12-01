use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut elves = vec![0];
    for line in input.lines() {
        if line.trim().is_empty() {
            elves.push(0);
            continue;
        }
        *elves.last_mut().unwrap() += line.parse::<i32>().unwrap()
    }
    *elves.iter().max().unwrap()
}

fn part2(input: &str) -> i32 {
    let mut elves = vec![0];
    for line in input.lines() {
        if line.trim().is_empty() {
            elves.push(0);
            continue;
        }
        *elves.last_mut().unwrap() += line.parse::<i32>().unwrap()
    }
    elves.sort_by(|a, b| b.cmp(a));
    elves[0] + elves[1] + elves[2]
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

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    0
}

fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}

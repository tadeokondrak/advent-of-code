use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve(&input, find_digit_simple));
    println!("p2: {}", solve(&input, find_digit));
}

fn find_digit_simple(s: &str) -> Option<u32> {
    s.chars().next().unwrap().to_digit(10)
}

fn find_digit_word(s: &str) -> Option<u32> {
    [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .iter()
    .enumerate()
    .find_map(|(i, word)| s.starts_with(word).then_some(i as u32 + 1))
}

fn find_digit(s: &str) -> Option<u32> {
    find_digit_simple(s).or_else(|| find_digit_word(s))
}

fn solve(input: &str, find_digit: impl Fn(&str) -> Option<u32>) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut forward = (0..line.len()).filter_map(|i| find_digit(&line[i..]));
            let mut backward = (0..line.len()).rev().filter_map(|i| find_digit(&line[i..]));
            10 * forward.next().unwrap() + backward.next().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            solve(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
                find_digit_simple
            ),
            142
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
                find_digit
            ),
            281
        );
    }
}

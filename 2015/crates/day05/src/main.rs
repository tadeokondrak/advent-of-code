use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> usize {
    input.lines().filter(|word| is_nice_p1(word)).count()
}

fn solve_p2(input: &str) -> usize {
    input.lines().filter(|word| is_nice_p2(word)).count()
}

fn is_nice_p1(word: &str) -> bool {
    let contains_enough_vowels = word
        .chars()
        .filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(c))
        .count()
        >= 3;
    let contains_double = word.as_bytes().windows(2).any(|w| w[0] == w[1]);
    let contains_banned = ["ab", "cd", "pq", "xy"]
        .iter()
        .any(|banned| word.contains(banned));
    contains_enough_vowels && contains_double && !contains_banned
}

fn is_nice_p2(word: &str) -> bool {
    let contains_qualifying_double = (0..(word.len() - 1)).any(|i| {
        let pair = &word[i..i + 2];
        word[..i].find(pair).is_some() || word[i + 2..].find(pair).is_some()
    });
    let contains_sandwich = word.as_bytes().windows(3).any(|w| w[0] == w[2]);
    contains_qualifying_double && contains_sandwich
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert!(is_nice_p1("ugknbfddgicrmopn"));
        assert!(is_nice_p1("aaa"));
        assert!(!is_nice_p1("jchzalrnumimnmhp"));
        assert!(!is_nice_p1("haegwjzuvuyypxyu"));
        assert!(!is_nice_p1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn part_2() {
        assert!(is_nice_p2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_p2("xxyxx"));
        assert!(!is_nice_p2("uurcxstgmygtbstg"));
        assert!(!is_nice_p2("ieodomkazucvgmuy"));
    }
}

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve_p1(&input));
    println!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> u32 {
    let mut stack = Vec::new();
    let mut total = 0;
    for line in input.lines() {
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                }
                ')' | ']' | '}' | '>' => {
                    if stack.pop().unwrap() != flip(c) {
                        total += score(c);
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    total
}

fn solve_p2(input: &str) -> u64 {
    let mut scores = Vec::new();
    for line in input.lines() {
        let mut stack = Vec::new();
        let mut is_invalid = false;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' | ']' | '}' | '>' => {
                    if stack.pop().unwrap() != flip(c) {
                        is_invalid = true;
                    }
                }
                _ => unreachable!(),
            }
        }
        if !is_invalid {
            let mut total = 0u64;
            for c in stack.iter().rev() {
                total = match c {
                    '(' => total * 5 + 1,
                    '[' => total * 5 + 2,
                    '{' => total * 5 + 3,
                    '<' => total * 5 + 4,
                    _ => unreachable!(),
                };
            }
            scores.push(total);
        }
    }
    scores.sort_unstable();
    scores[scores.len() / 2]
}
fn flip(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!(),
    }
}

fn score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(INPUT), 26397);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_p2(INPUT), 288957);
    }
}

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

#[derive(Debug, Copy, Clone)]
struct Symbol {
    x: i32,
    y: i32,
    c: char,
}

#[derive(Debug, Copy, Clone)]
struct Number {
    x: (i32, i32),
    y: i32,
    value: i32,
}

fn parse(input: &str) -> (Vec<Symbol>, Vec<Number>) {
    let mut syms = Vec::new();
    let mut nums = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        let mut x = 0;
        let mut run = String::new();
        for c in line.trim().chars() {
            match c {
                '0'..='9' => run.push(c),
                _ => {
                    if !run.is_empty() {
                        nums.push(Number {
                            x: (x - run.len() as i32, x - 1),
                            y,
                            value: run.parse().unwrap(),
                        })
                    }
                    if c != '.' {
                        syms.push(Symbol { x, y, c });
                    }
                    run.clear();
                }
            }
            x += 1;
        }
        if !run.is_empty() {
            nums.push(Number {
                x: (x - run.len() as i32, x - 1),
                y,
                value: run.parse().unwrap(),
            })
        }
    }
    (syms, nums)
}

impl Number {
    fn is_adjacent_to(self, sym: Symbol) -> bool {
        (self.x.0 - 1..=self.x.1 + 1).contains(&sym.x) && (self.y - 1..=self.y + 1).contains(&sym.y)
    }
}

fn solve_p1(input: &str) -> i32 {
    let (syms, nums) = parse(input);
    nums.iter()
        .filter(|num| syms.iter().copied().any(|sym| num.is_adjacent_to(sym)))
        .map(|num| num.value)
        .sum()
}

fn solve_p2(input: &str) -> i32 {
    let (syms, nums) = parse(input);
    syms.iter()
        .copied()
        .filter(|sym| sym.c == '*')
        .filter_map(|sym| {
            let adjacent_nums = nums
                .iter()
                .copied()
                .filter(|num| num.is_adjacent_to(sym))
                .collect::<Vec<_>>();
            (adjacent_nums.len() == 2).then(|| adjacent_nums[0].value * adjacent_nums[1].value)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            solve_p1(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
            ),
            4361
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve_p2(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
            ),
            467835
        );
    }
}

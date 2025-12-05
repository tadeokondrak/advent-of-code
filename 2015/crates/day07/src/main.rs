#![feature(test)]
use std::{
    collections::HashMap,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u16 {
    solve(input, "a", &[])
}

fn part2(input: &str) -> u16 {
    let p1 = solve(input, "a", &[]);
    solve(input, "a", &[("b", p1)])
}

#[derive(Clone, Copy, Debug)]
enum Unop {
    Id,
    Not,
}

#[derive(Clone, Copy, Debug)]
enum Binop {
    And,
    Or,
    Lshift,
    Rshift,
}

#[derive(Clone, Copy, Debug)]
enum Expr<'a> {
    Const(u16),
    Unary(Unop, &'a str),
    Binary(Binop, &'a str, &'a str),
}

fn solve(input: &str, name: &str, overrides: &[(&str, u16)]) -> u16 {
    fn eval<'a>(exprs: &mut HashMap<&'a str, Expr<'a>>, name: &'a str) -> u16 {
        if let Ok(x) = name.parse::<u16>() {
            return x;
        }
        let result = match exprs[name] {
            Expr::Const(x) => x,
            Expr::Unary(op, operand) => {
                let operand = eval(exprs, operand);
                match op {
                    Unop::Id => operand,
                    Unop::Not => !operand,
                }
            }
            Expr::Binary(op, lhs, rhs) => {
                let lhs = eval(exprs, lhs);
                let rhs = eval(exprs, rhs);
                match op {
                    Binop::And => lhs & rhs,
                    Binop::Or => lhs | rhs,
                    Binop::Lshift => lhs << rhs,
                    Binop::Rshift => lhs >> rhs,
                }
            }
        };
        exprs.insert(name, Expr::Const(result));
        result
    }
    let mut exprs = input
        .lines()
        .map(|line| parse_line(line))
        .collect::<HashMap<_, _>>();
    for &(name, val) in overrides {
        exprs.insert(name, Expr::Const(val));
    }
    eval(&mut exprs, name)
}

fn parse_line(line: &str) -> (&str, Expr<'_>) {
    let (expr, name) = line.split_once(" -> ").unwrap();
    let parsed = if let Some(operand) = expr.strip_prefix("NOT ") {
        Expr::Unary(Unop::Not, operand)
    } else if let Some((lhs, rhs)) = expr.split_once(" AND ") {
        Expr::Binary(Binop::And, lhs, rhs)
    } else if let Some((lhs, rhs)) = expr.split_once(" OR ") {
        Expr::Binary(Binop::Or, lhs, rhs)
    } else if let Some((lhs, rhs)) = expr.split_once(" LSHIFT ") {
        Expr::Binary(Binop::Lshift, lhs, rhs)
    } else if let Some((lhs, rhs)) = expr.split_once(" RSHIFT ") {
        Expr::Binary(Binop::Rshift, lhs, rhs)
    } else {
        Expr::Unary(Unop::Id, expr)
    };
    (name, parsed)
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    const TEST_INPUT: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn test_part1() {
        assert_eq!(solve(TEST_INPUT, "d", &[]), 72);
        assert_eq!(solve(TEST_INPUT, "e", &[]), 507);
        assert_eq!(solve(TEST_INPUT, "f", &[]), 492);
        assert_eq!(solve(TEST_INPUT, "g", &[]), 114);
        assert_eq!(solve(TEST_INPUT, "h", &[]), 65412);
        assert_eq!(solve(TEST_INPUT, "i", &[]), 65079);
        assert_eq!(solve(TEST_INPUT, "x", &[]), 123);
        assert_eq!(solve(TEST_INPUT, "y", &[]), 456);
    }

    #[bench]
    fn real_p1(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(test::black_box(&input)), 46065));
    }

    #[bench]
    fn real_p2(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(test::black_box(&input)), 14134));
    }
}

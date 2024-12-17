#![feature(test, box_patterns)]
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    io::{self, Read},
};
use util::{point, Grid, Offset, Point};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let (regs, prog) = input.split_once("\n\nProgram: ").unwrap();
    let regs = regs
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let &[a, b, c, ..] = regs.as_slice() else {
        panic!();
    };
    run_program(prog, a, b, c)
}

fn run_program(prog: &str, mut a: i64, mut b: i64, mut c: i64) -> String {
    let prog = prog
        .split(",")
        .map(|it| it.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut ip = 0;
    let mut output = Vec::new();
    while ip < prog.len() {
        let opc = prog[ip];
        let lit = prog[ip + 1];
        let combo = match lit {
            0 | 1 | 2 | 3 => lit,
            4 => a,
            5 => b,
            6 => c,
            other => unreachable!("{other}"),
        };

        match opc {
            0 => {
                // eprintln!("ADV: a = a / (1 << combo)");
                // eprintln!("> ADV: a = {a} / (1 << {combo})");
                a = a / (1 << combo);
            }
            1 => {
                // eprintln!("BXL: b = b ^ lit");
                // eprintln!("> BXL: b = {b} ^ {lit}");
                b = b ^ lit;
            }
            2 => {
                // eprintln!("BST: b = combo % 8");
                // eprintln!("> BST: b = {combo} % 8");
                b = combo % 8;
            }
            3 => {
                // eprintln!("JNZ: a lit");
                // eprintln!("> JNZ: {a} {lit}");
                if a != 0 {
                    ip = lit as usize;
                    continue;
                }
            }
            4 => {
                // eprintln!("BXC: b = b ^ c");
                // eprintln!("> BXC: b = {b} ^ {c}");
                b = b ^ c;
            }
            5 => {
                // eprintln!("OUT: output.push(combo % 8)");
                // eprintln!("> OUT: output.push({combo} % 8)");
                output.push(combo % 8);
            }
            6 => {
                // eprintln!("BDV: b = a / (1 << combo)");
                // eprintln!("> BDV: b = {a} / (1 << {combo})");
                b = a / (1 << combo);
            }
            7 => {
                // eprintln!("CDV: c = a / (1 << combo)");
                // eprintln!("> CDV: c = {a} / (1 << {combo})");
                c = a / (1 << combo);
            }
            _ => todo!(),
        }
        ip += 2;
    }
    output
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[derive(Clone)]
enum Expr {
    Input,
    Const(i64),
    Div(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Shl(Box<Expr>, Box<Expr>),
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input => write!(f, "Input"),
            Self::Const(arg0) => write!(f, "{arg0}"),
            Self::Div(arg0, arg1) => write!(f, "({arg0:?} / {arg1:?})"),
            Self::Mod(arg0, arg1) => write!(f, "({arg0:?} % {arg1:?})"),
            Self::Xor(arg0, arg1) => write!(f, "({arg0:?} ^ {arg1:?})"),
            Self::Shl(arg0, arg1) => write!(f, "({arg0:?} << {arg1:?})"),
        }
    }
}

impl Expr {
    fn eval(&self, input: i64) -> i64 {
        match self {
            Expr::Input => input,
            Expr::Const(val) => *val,
            Expr::Div(box lhs, box rhs) => lhs.eval(input) / rhs.eval(input),
            Expr::Mod(box lhs, box rhs) => lhs.eval(input) % rhs.eval(input),
            Expr::Xor(box lhs, box rhs) => lhs.eval(input) ^ rhs.eval(input),
            Expr::Shl(box lhs, box rhs) => lhs.eval(input) << rhs.eval(input),
        }
    }

    fn simplify(&mut self) {
        match self {
            Expr::Input => {}
            Expr::Const(_) => {}
            Expr::Div(lhs, rhs)
            | Expr::Mod(lhs, rhs)
            | Expr::Xor(lhs, rhs)
            | Expr::Shl(lhs, rhs) => {
                lhs.simplify();
                rhs.simplify();
            }
        }

        match self {
            Expr::Div(box Expr::Const(lhs), box Expr::Const(rhs)) => {
                *self = Expr::Const(*lhs / *rhs)
            }
            Expr::Div(box Expr::Div(lhs, box Expr::Const(rhs0)), box Expr::Const(rhs1)) => {
                *self = Expr::Div(lhs.clone(), Box::new(Expr::Const(*rhs0 * *rhs1)))
            }
            Expr::Mod(box Expr::Const(lhs), box Expr::Const(rhs)) => {
                *self = Expr::Const(*lhs % *rhs)
            }
            Expr::Xor(box Expr::Xor(lhs, box Expr::Const(rhs0)), box Expr::Const(rhs1)) => {
                *self = Expr::Xor(lhs.clone(), Box::new(Expr::Const(*rhs0 ^ *rhs1)))
            }
            Expr::Shl(box Expr::Const(lhs), box Expr::Const(rhs)) => {
                *self = Expr::Const(*lhs << *rhs)
            }
            _ => {}
        }
    }
}

fn part2(input: &str) -> String {
    let (regs, prog) = input.split_once("\n\nProgram: ").unwrap();
    let regs = regs
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let &[mut a, mut b, mut c, ..] = regs.as_slice() else {
        panic!();
    };

    let start = dbg!(8i64.pow(prog.len() as u32 / 2));
    let end = dbg!(8i64.pow(prog.len() as u32 / 2 + 1));
    assert_eq!(run_program(prog, start - 1, b, c).len(), prog.len() - 2);
    assert_eq!(run_program(prog, start, b, c).len(), prog.len());
    assert_eq!(run_program(prog, end - 1, b, c).len(), prog.len());
    assert_eq!(run_program(prog, end, b, c).len(), prog.len() + 2);

    let mut output = Vec::new();

    let parsedprog = prog
        .split(",")
        .map(|it| it.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    {
        let mut a_orig = a;
        let mut a = Expr::Input;
        let mut b = Expr::Const(0);
        let mut c = Expr::Const(0);
        let mut ip = 0;
        while ip < parsedprog.len() {
            let opc = parsedprog[ip];
            let lit = parsedprog[ip + 1];
            let combo = match lit {
                0 | 1 | 2 | 3 => Expr::Const(lit),
                4 => a.clone(),
                5 => b.clone(),
                6 => c.clone(),
                other => unreachable!("{other}"),
            };
            //dbg!(&a, &b, &c);

            match opc {
                0 => {
                    a = Expr::Div(
                        Box::new(a.clone()),
                        Box::new(Expr::Shl(Box::new(Expr::Const(1)), Box::new(combo.clone()))),
                    );
                }
                1 => {
                    b = Expr::Xor(Box::new(b.clone()), Box::new(Expr::Const(lit)));
                }
                2 => {
                    b = Expr::Mod(Box::new(combo.clone()), Box::new(Expr::Const(8)));
                }
                3 => {
                    assert_eq!(lit, 0);
                    if output.len() != parsedprog.len() {
                        ip = 0;
                        continue;
                    }
                }
                4 => {
                    b = Expr::Xor(Box::new(b.clone()), Box::new(c.clone()));
                }
                5 => {
                    output.push(Expr::Mod(Box::new(combo.clone()), Box::new(Expr::Const(8))));
                }
                6 => {
                    b = Expr::Div(
                        Box::new(a.clone()),
                        Box::new(Expr::Shl(Box::new(Expr::Const(1)), Box::new(combo))),
                    );
                }
                7 => {
                    c = Expr::Div(
                        Box::new(a.clone()),
                        Box::new(Expr::Shl(Box::new(Expr::Const(1)), Box::new(combo))),
                    );
                }
                _ => todo!(),
            }
            ip += 2;
        }
    }
    for expr in output.iter_mut() {
        for i in 0..1000 {
            expr.simplify();
        }
        eprintln!("{expr:?}");
    }
    let mut i = start;
    'l: loop {
        let nums = [
            1,
            8,
            64,
            512,
            4096,
            32768,
            262144,
            2097152,
            16777216,
            134217728,
            1073741824,
            8589934592,
            68719476736,
            549755813888,
            4398046511104,
            35184372088832,
        ];
        for j in 0..nums.len() {
            if output[output.len() - 1 - j].eval(i) != parsedprog[parsedprog.len() - 1 - j] {
                i += nums[nums.len() - 1 - j];
                continue 'l;
            }
        }
        //if output[output.len() - 2].eval(i) != 3 {
        //    i += 2097152;
        //    continue;
        //}
        eprintln!("{i} ---");
        //dbg!();
        for expr in output.iter_mut() {
            eprint!("{},", expr.eval(i));
        }
        eprintln!();
        if dbg!(run_program(prog, i, 0, 0)) == prog {
            return i.to_string();
        }

        //dbg!((i) % 8);
        //dbg!((i / 8) % 8);
        //dbg!((i / 64) % 8);
        //dbg!((i / 512) % 8);
        //dbg!((i / 4096) % 8);
        //dbg!((i / 32768) % 8);
        //dbg!((i / 262144) % 8);
        //dbg!((i / 2097152) % 8);
        //dbg!((i / 16777216) % 8);
        i += 8;
    }
    "".to_owned()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const TEST_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
            ),
            ""
        );
    }

    #[bench]
    #[ignore = "reason"]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        //b.iter(|| assert_eq!(part1(black_box(&input)), 95476));
    }

    #[bench]
    #[ignore = "reason"]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        //b.iter(|| assert_eq!(part2(black_box(&input)), 511));
    }
}

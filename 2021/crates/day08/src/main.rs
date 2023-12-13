use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve_p1(&input));
    println!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> usize {
    let input = input
        .trim()
        .split("\n")
        .map(|line| {
            let (wires, outnums) = line.split_once(" | ").unwrap();
            let wires = wires.split(" ").collect::<Vec<_>>();
            let outnums = outnums.split(" ").collect::<Vec<_>>();
            (wires, outnums)
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for (_, outnums) in input {
        let easycount = outnums
            .iter()
            .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
            .count();
        sum += easycount;
    }
    sum
}

fn solve_p2(input: &str) -> usize {
    let input = input
        .trim()
        .split("\n")
        .map(|line| {
            let (wires, outnums) = line.split_once(" | ").unwrap();
            let wires = wires.split(" ").collect::<Vec<_>>();
            let outnums = outnums.split(" ").collect::<Vec<_>>();
            (wires, outnums)
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for (wires, outnums) in input {
        let mut rules = Vec::new();
        for wire in &wires {
            fn rule(wire: &str, possible: &[char] /* must be sorted */) -> Expr {
                let mut rule = Vec::new();
                use permutohedron::LexicalPermutation;
                let mut possible = possible.to_vec();
                loop {
                    rule.push(Expr::And(
                        wire.chars()
                            .zip(possible.iter().copied())
                            .map(|(i, o)| Expr::Maps(i, o))
                            .collect(),
                    ));
                    if !possible.next_permutation() {
                        break;
                    }
                }
                Expr::Or(rule)
            }
            if wire.len() == 2 {
                rules.push(rule(wire, &['c', 'f']));
            }
            if wire.len() == 3 {
                rules.push(rule(wire, &['a', 'c', 'f']));
            }
            if wire.len() == 4 {
                rules.push(rule(wire, &['b', 'c', 'd', 'f']));
            }
            if wire.len() == 5 {
                rules.push(Expr::Or(vec![
                    rule(wire, &['a', 'c', 'd', 'e', 'g']),
                    rule(wire, &['a', 'c', 'd', 'f', 'g']),
                    rule(wire, &['a', 'b', 'd', 'f', 'g']),
                ]));
            }
            if wire.len() == 6 {
                rules.push(Expr::Or(vec![
                    rule(wire, &['a', 'b', 'c', 'e', 'f', 'g']),
                    rule(wire, &['a', 'b', 'd', 'e', 'f', 'g']),
                    rule(wire, &['a', 'b', 'c', 'd', 'f', 'g']),
                ]));
            }
        }
        let bigrule = Expr::And(rules);
        let sol = find_solution(&bigrule);
        let mut num = 0usize;
        for outnum in &outnums {
            num = num * 10 + sol.get_num(outnum) as usize;
        }
        sum += num;
    }
    sum
}

#[derive(Clone)]
enum Expr {
    Maps(/*input*/ char, /*output*/ char),
    And(Vec<Expr>),
    Or(Vec<Expr>),
}

impl Expr {
    fn eval(&self, test: &impl Fn(char) -> char) -> bool {
        match self {
            &Expr::Maps(i, o) => test(i) == o,
            Expr::And(v) => v.iter().all(|e| e.eval(test)),
            Expr::Or(v) => v.iter().any(|e| e.eval(test)),
        }
    }
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Maps(l, r) => write!(f, "{} -> {}", l, r)?,
            Expr::And(v) => {
                write!(f, "(")?;
                for (i, e) in v.iter().enumerate() {
                    write!(f, "{:?}", e)?;
                    if i != v.len() - 1 {
                        write!(f, " && ")?;
                    }
                }
                write!(f, ")")?;
            }
            Expr::Or(v) => {
                write!(f, "(")?;
                for (i, e) in v.iter().enumerate() {
                    write!(f, "{:?}", e)?;
                    if i != v.len() - 1 {
                        write!(f, " || ")?;
                    }
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Solution {
    data: Vec<char>,
}

impl Solution {
    fn initial() -> Self {
        Solution {
            data: vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'],
        }
    }
    fn next_perm(&mut self) -> bool {
        use permutohedron::LexicalPermutation;
        self.data.next_permutation()
    }
    fn map(&self, x: char) -> char {
        self.data[(x as u8 - b'a') as usize]
    }
    fn map_str(&self, s: &str) -> String {
        let mut num_str = s.chars().map(|c| self.map(c)).collect::<String>();
        // SAFETY: not
        unsafe { num_str.as_bytes_mut().sort_unstable() };
        num_str
    }

    fn get_num(&self, s: &str) -> u8 {
        let num_str = self.map_str(s);
        match &*num_str {
            "cf" => 1,
            "acdeg" => 2,
            "acdfg" => 3,
            "bcdf" => 4,
            "abdfg" => 5,
            "abdefg" => 6,
            "acf" => 7,
            "abcdefg" => 8,
            "abcdfg" => 9,
            "abcefg" => 0,
            other => unreachable!("{}", other),
        }
    }
}

fn find_solution(rule: &Expr) -> Solution {
    let mut sol = Solution::initial();
    loop {
        if rule.eval(&|c| sol.map(c)) {
            return sol;
        }
        if !sol.next_perm() {
            panic!("no solution!")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_A: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    const INPUT_B: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(INPUT_B), 26);
    }

    #[test]
    #[ignore = "slow"]
    fn part_2() {
        assert_eq!(solve_p2(INPUT_A), 5353);
        assert_eq!(solve_p2(INPUT_B), 61229);
    }
}

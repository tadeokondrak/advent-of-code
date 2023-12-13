use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve_p1(&input));
    println!("p2: {}", solve_p2(&input));
}

fn ones_nones(count: usize, vals: &[u32]) -> ([u32; 32], [u32; 32]) {
    let mut ones = [0u32; 32];
    let mut nones = [0u32; 32];
    for num in vals {
        for i in 0..count {
            if num & (1 << (count - i - 1)) != 0 {
                ones[i] += 1;
            } else {
                nones[i] += 1;
            }
        }
    }
    (ones, nones)
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Gamma,
    Epsilon,
}

fn value(count: usize, ones: &[u32], nones: &[u32], kind: Value) -> u32 {
    let mut value = 0u32;
    for bit in 0..count {
        if let (Value::Gamma, true) | (Value::Epsilon, false) = (kind, ones[bit] >= nones[bit]) {
            value |= 1 << count - bit - 1;
        }
    }
    value
}

enum Rating {
    Oxygen,
    Co2,
}

fn rating(i: usize, count: usize, vals: &[u32], kind: Rating) -> u32 {
    if vals.len() == 1 {
        return vals[0];
    }
    let mut out = Vec::new();
    let (ones, nones) = ones_nones(count, vals);
    let gamma = value(count, &ones, &nones, Value::Gamma);
    let epsilon = value(count, &ones, &nones, Value::Epsilon);
    for val in vals {
        match kind {
            Rating::Oxygen => {
                if val & (1 << (count - 1 - i)) == gamma & (1 << (count - i - 1)) {
                    out.push(*val);
                }
            }
            Rating::Co2 => {
                if val & (1 << (count - 1 - i)) == epsilon & (1 << (count - i - 1)) {
                    out.push(*val);
                }
            }
        }
    }
    rating(i + 1, count, &out, kind)
}

fn solve_p1(input: &str) -> u32 {
    let lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    let vals: Vec<u32> = lines
        .iter()
        .map(|x| u32::from_str_radix(&x, 2).unwrap())
        .collect();
    let count = lines[0].len();
    let (ones, nones) = ones_nones(count, &vals);
    value(count, &ones, &nones, Value::Gamma) * value(count, &ones, &nones, Value::Epsilon)
}

fn solve_p2(input: &str) -> u32 {
    let lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    let vals: Vec<u32> = lines
        .iter()
        .map(|x| u32::from_str_radix(&x, 2).unwrap())
        .collect();
    let count = lines[0].len();
    rating(0, count, &vals, Rating::Oxygen) * rating(0, count, &vals, Rating::Co2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str =
        "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(INPUT), 198);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_p2(INPUT), 230);
    }
}

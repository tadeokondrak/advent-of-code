use std::{
    cmp::Ordering,
    io::{self, Read},
};

use serde::Deserialize;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(untagged)]
enum Value {
    Int(i32),
    Array(Vec<Value>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(<Self as Ord>::cmp(&self, other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Int(x), Value::Int(y)) => x.cmp(y),
            (&Value::Int(x), right @ Value::Array(_)) => {
                Value::Array(vec![Value::Int(x)]).cmp(&right)
            }
            (left @ Value::Array(_), &Value::Int(y)) => {
                left.cmp(&Value::Array(vec![Value::Int(y)]))
            }
            (Value::Array(x), Value::Array(y)) => {
                let mut left_iter = x.iter();
                let mut right_iter = y.iter();
                loop {
                    match (left_iter.next(), right_iter.next()) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(x), Some(y)) => match x.cmp(y) {
                            ordering @ (Ordering::Less | Ordering::Greater) => return ordering,
                            Ordering::Equal => {}
                        },
                    }
                }
            }
        }
    }
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .map(|(i, packets)| {
            let mut packets = packets.split("\n");
            let left = serde_json::from_str::<Value>(packets.next().unwrap()).unwrap();
            let right = serde_json::from_str::<Value>(packets.next().unwrap()).unwrap();
            (i, (left, right))
        })
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(input: &str) -> usize {
    let distress = [
        Value::Array(vec![Value::Array(vec![Value::Int(2)])]),
        Value::Array(vec![Value::Array(vec![Value::Int(6)])]),
    ];
    let mut packets = input
        .replace("\n\n", "\n")
        .split("\n")
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(_, line)| serde_json::from_str::<Value>(line).unwrap())
        .collect::<Vec<_>>();
    packets.extend(distress.iter().cloned());
    packets.sort();
    packets
        .iter()
        .enumerate()
        .filter(|&(_, v)| distress.contains(v))
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 140);
    }
}

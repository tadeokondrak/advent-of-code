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

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Value {
    Int(i32),
    Array(Vec<Value>),
}

fn less(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Int(x), Value::Int(y)) => x < y,
        (&Value::Int(x), right @ Value::Array(_)) => {
            less(&Value::Array(vec![Value::Int(x)]), right)
        }
        (left @ Value::Array(_), &Value::Int(y)) => less(left, &Value::Array(vec![Value::Int(y)])),
        (Value::Array(x), Value::Array(y)) => {
            let mut left_iter = x.iter();
            let mut right_iter = y.iter();
            loop {
                match (left_iter.next(), right_iter.next()) {
                    (None, None) => return false,
                    (None, Some(_)) => return true,
                    (Some(_), None) => return false,
                    (Some(x), Some(y)) => {
                        if less(x, y) {
                            return true;
                        }
                        if less(y, x) {
                            return false;
                        }
                    }
                }
            }
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut count = 0;
    for (i, packets) in input.split("\n\n").enumerate() {
        let mut packets = packets.split("\n");
        let left = serde_json::from_str::<Value>(packets.next().unwrap()).unwrap();
        let right = serde_json::from_str::<Value>(packets.next().unwrap()).unwrap();
        if less(&left, &right) {
            count += i as i32 + 1;
        }
    }
    count
}

fn part2(input: &str) -> usize {
    let mut packets = input
        .replace("\n\n", "\n")
        .split("\n")
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(_, line)| serde_json::from_str::<Value>(line).unwrap())
        .collect::<Vec<_>>();
    packets.push(Value::Array(vec![Value::Array(vec![Value::Int(2)])]));
    packets.push(Value::Array(vec![Value::Array(vec![Value::Int(6)])]));
    packets.sort_by(|a, b| {
        if less(a, b) {
            Ordering::Less
        } else if less(b, a) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
    let mut indices = packets
        .iter()
        .enumerate()
        .filter(|(_, v)| {
            if let Value::Array(w) = v {
                if w.len() != 1 {
                    return false;
                }
                if let Value::Array(x) = &w[0] {
                    if x.len() != 1 {
                        return false;
                    }
                    if let Value::Int(2 | 6) = x[0] {
                        return true;
                    }
                }
            }
            return false;
        })
        .map(|(i, _)| i + 1);
    indices.next().unwrap() * indices.next().unwrap()
}

#[cfg(test)]
mod test {
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

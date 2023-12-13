use std::{
    io::{self, Read},
    mem,
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

enum Operation {
    Square,
    Add(i64),
    Mul(i64),
}

impl Operation {
    fn parse(s: &str) -> Operation {
        let mut tokens = s.split(" ");
        match (
            tokens.next().unwrap(),
            tokens.next().unwrap(),
            tokens.next().unwrap(),
        ) {
            ("old", "*", "old") => Operation::Square,
            ("old", "+", other) => Operation::Add(other.parse().unwrap()),
            ("old", "*", other) => Operation::Mul(other.parse().unwrap()),
            _ => panic!(),
        }
    }

    fn eval(&self, old: i64) -> i64 {
        match self {
            Operation::Square => old * old,
            Operation::Add(rhs) => old + rhs,
            Operation::Mul(rhs) => old * rhs,
        }
    }
}

struct Monkey {
    worry_levels: Vec<i64>,
    operation: Operation,
    test_divisible_by: i64,
    throw_to_monkey_true: usize,
    throw_to_monkey_false: usize,
    inspect_count: i64,
}

impl Monkey {
    fn parse(s: &str) -> Monkey {
        let mut lines = s.lines();
        // Skip first line
        lines.next().unwrap();
        let mut line_without_prefix =
            |prefix: &str| lines.next().unwrap().strip_prefix(prefix).unwrap();
        let worry_levels = line_without_prefix("  Starting items: ");
        let operation = line_without_prefix("  Operation: new = ");
        let test_divisible_by = line_without_prefix("  Test: divisible by ");
        let throw_to_monkey_true = line_without_prefix("    If true: throw to monkey ");
        let throw_to_monkey_false = line_without_prefix("    If false: throw to monkey ");
        Monkey {
            worry_levels: worry_levels
                .split(", ")
                .map(|num| num.parse().unwrap())
                .collect(),
            operation: Operation::parse(&operation),
            test_divisible_by: test_divisible_by.parse().unwrap(),
            throw_to_monkey_true: throw_to_monkey_true.parse().unwrap(),
            throw_to_monkey_false: throw_to_monkey_false.parse().unwrap(),
            inspect_count: 0,
        }
    }
}

fn part1(input: &str) -> i64 {
    let mut monkeys = input.split("\n\n").map(Monkey::parse).collect::<Vec<_>>();
    for _ in 1..=20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let mut new_indices = Vec::new();
            for item in monkey.worry_levels.iter_mut() {
                *item = monkey.operation.eval(*item);
                *item /= 3;
                new_indices.push(if *item % monkey.test_divisible_by == 0 {
                    monkey.throw_to_monkey_true
                } else {
                    monkey.throw_to_monkey_false
                });
            }
            monkey.inspect_count += monkey.worry_levels.len() as i64;
            let worry_levels = mem::take(&mut monkeys[i].worry_levels);
            for (level, to_monkey_index) in worry_levels.into_iter().zip(new_indices) {
                monkeys[to_monkey_index].worry_levels.push(level);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    monkeys[0].inspect_count * monkeys[1].inspect_count
}

fn part2(input: &str) -> i64 {
    let mut monkeys = input.split("\n\n").map(Monkey::parse).collect::<Vec<_>>();
    let common_multiple = monkeys
        .iter()
        .map(|monkey| monkey.test_divisible_by)
        .product::<i64>();
    for _ in 1..=10000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let mut new_indices = Vec::new();
            for item in monkey.worry_levels.iter_mut() {
                *item = monkey.operation.eval(*item);
                new_indices.push(if *item % monkey.test_divisible_by == 0 {
                    monkey.throw_to_monkey_true
                } else {
                    monkey.throw_to_monkey_false
                });
            }
            monkey.inspect_count += monkey.worry_levels.len() as i64;
            let worry_levels = mem::take(&mut monkeys[i].worry_levels);
            for (level, to_monkey_index) in worry_levels.into_iter().zip(new_indices) {
                monkeys[to_monkey_index]
                    .worry_levels
                    .push(level % common_multiple);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    monkeys[0].inspect_count * monkeys[1].inspect_count
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2713310158);
    }
}

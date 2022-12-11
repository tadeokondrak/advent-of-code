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

#[derive(Clone, Debug, PartialEq, Eq)]
enum Operation {
    Old,
    Const(i64),
    Add(Box<Operation>, Box<Operation>),
    Mul(Box<Operation>, Box<Operation>),
}

impl Operation {
    fn parse(s: &str) -> Operation {
        let mut tokens = s.split(" ");
        let lhs = match tokens.next().unwrap() {
            "old" => Operation::Old,
            other => Operation::Const(other.parse().unwrap()),
        };
        let op = match tokens.next().unwrap() {
            "+" => Operation::Add,
            "*" => Operation::Mul,
            _ => panic!(),
        };
        let rhs = match tokens.next().unwrap() {
            "old" => Operation::Old,
            other => Operation::Const(other.parse().unwrap()),
        };
        op(lhs.into(), rhs.into())
    }

    fn eval(&self, old: i64) -> i64 {
        match self {
            Operation::Old => old,
            Operation::Const(x) => *x,
            Operation::Add(lhs, rhs) => lhs.eval(old) + rhs.eval(old),
            Operation::Mul(lhs, rhs) => lhs.eval(old) * rhs.eval(old),
        }
    }
}

#[derive(Debug)]
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
        _ = lines.next().unwrap();
        let worry_levels =
            sscanf::sscanf!(lines.next().unwrap().trim(), "Starting items: {String}")
                .unwrap()
                .split(", ")
                .map(|num| num.parse().unwrap())
                .collect();
        let operation = Operation::parse(
            &sscanf::sscanf!(lines.next().unwrap().trim(), "Operation: new = {String}").unwrap(),
        );
        let test_divisible_by =
            sscanf::sscanf!(lines.next().unwrap().trim(), "Test: divisible by {i64}").unwrap();
        let throw_to_monkey_true = sscanf::sscanf!(
            lines.next().unwrap().trim(),
            "If true: throw to monkey {usize}"
        )
        .unwrap();
        let throw_to_monkey_false = sscanf::sscanf!(
            lines.next().unwrap().trim(),
            "If false: throw to monkey {usize}"
        )
        .unwrap();
        Monkey {
            worry_levels,
            operation,
            test_divisible_by,
            throw_to_monkey_true,
            throw_to_monkey_false,
            inspect_count: 0,
        }
    }
}

fn part1(input: &str) -> i64 {
    let mut monkeys = input.split("\n\n").map(Monkey::parse).collect::<Vec<_>>();
    for _ in 1..=20 {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            let mut new_positions = Vec::new();
            for item in monkey.worry_levels.iter_mut() {
                *item = monkey.operation.eval(*item);
                *item /= 3;
                new_positions.push(if *item % monkey.test_divisible_by == 0 {
                    monkey.throw_to_monkey_true
                } else {
                    monkey.throw_to_monkey_false
                });
            }
            monkey.inspect_count += monkey.worry_levels.len() as i64;
            let worry_levels = mem::take(&mut monkeys[i].worry_levels);
            for (level, to_monkey_index) in worry_levels.into_iter().zip(new_positions) {
                monkeys[to_monkey_index].worry_levels.push(level);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspect_count.cmp(&a.inspect_count));
    monkeys[0].inspect_count * monkeys[1].inspect_count
}

fn part2(input: &str) -> i64 {
    let mut monkeys = input.split("\n\n").map(Monkey::parse).collect::<Vec<_>>();
    let lcm = monkeys
        .iter()
        .map(|monkey| monkey.test_divisible_by)
        .product::<i64>();
    for _ in 1..=10000 {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            let mut actions = Vec::new();
            for item in monkey.worry_levels.iter_mut() {
                *item = monkey.operation.eval(*item);
                actions.push(if *item % monkey.test_divisible_by == 0 {
                    monkey.throw_to_monkey_true
                } else {
                    monkey.throw_to_monkey_false
                });
            }
            monkey.inspect_count += monkey.worry_levels.len() as i64;
            let worry_levels = mem::take(&mut monkeys[i].worry_levels);
            for (level, to_monkey_index) in worry_levels.into_iter().zip(actions) {
                monkeys[to_monkey_index].worry_levels.push(level % lcm);
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
    fn test_operation() {
        assert_eq!(
            Operation::parse("old * 19"),
            Operation::Mul(Operation::Old.into(), Operation::Const(19).into())
        );
        assert_eq!(Operation::parse("old * 19").eval(2), 19 * 2);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2713310158);
    }
}

use std::{
    cmp::Ordering,
    collections::HashMap,
    io::{stdin, Read},
    ops::Range,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

#[derive(Debug, Clone)]
enum Action {
    Reject,
    Accept,
    Goto(String),
}

#[derive(Debug, Clone)]
enum Predicate {
    Any,
    Less(String, i64),
    Greater(String, i64),
}

impl Predicate {
    fn eval(&self, input: [i64; 4]) -> bool {
        match self {
            Predicate::Any => true,
            Predicate::Less(var, v) => extract(input, var) < *v,
            Predicate::Greater(var, v) => extract(input, var) > *v,
        }
    }

    fn eval2(&self, input: [Range<i64>; 4]) -> Vec<[Range<i64>; 4]> {
        eprintln!("eval2 {self:?} {input:?}");
        match self {
            Predicate::Any => vec![input],
            Predicate::Less(var, v) => split_less(extract(input.clone(), var), *v)
                .into_iter()
                .map(|val| replace(input.clone(), var, val))
                .collect(),
            Predicate::Greater(var, v) => split_greater(extract(input.clone(), var), *v)
                .into_iter()
                .map(|val| replace(input.clone(), var, val))
                .collect(),
        }
    }
}

fn split_greater(range: Range<i64>, v: i64) -> Vec<Range<i64>> {
    if range.start > v {
        return vec![range];
    }
    if range.end < v {
        return vec![];
    }
    vec![(v + 1)..range.end]
}

fn split_less(range: Range<i64>, v: i64) -> Vec<Range<i64>> {
    if range.end < v {
        return vec![range];
    }
    if range.start > v {
        return vec![];
    }
    vec![range.start..v]
}

fn complement(range: Range<i64>) -> Vec<Range<i64>> {
    vec![1..range.start, range.end..4001]
}

fn difference(lhs: Range<i64>, rhs: Range<i64>) -> Vec<Range<i64>> {
    todo!()
}

fn difference2(lhs: [Range<i64>; 4], rhs: [Range<i64>; 4]) -> Vec<[Range<i64>; 4]> {
    let mut result = Vec::new();

    for i in 0..4 {
        let diff = difference(lhs[i].clone(), rhs[i].clone());
        let mut new_lhs = lhs.clone();

        if !diff.is_empty() {
            new_lhs[i] = diff[0].clone();
            result.push(new_lhs);
        }
    }

    result
}

fn extract<T: Clone>(input: [T; 4], var: &str) -> T {
    match var {
        "x" => input[0].clone(),
        "m" => input[1].clone(),
        "a" => input[2].clone(),
        "s" => input[3].clone(),
        _ => panic!(),
    }
}

fn replace<T: Clone>(input: [T; 4], var: &str, val: T) -> [T; 4] {
    match var {
        "x" => [val, input[1].clone(), input[2].clone(), input[3].clone()],
        "m" => [input[0].clone(), val, input[2].clone(), input[3].clone()],
        "a" => [input[0].clone(), input[1].clone(), val, input[3].clone()],
        "s" => [input[0].clone(), input[1].clone(), input[2].clone(), val],
        _ => panic!(),
    }
}

#[derive(Debug, Clone)]
struct Rule {
    pred: Predicate,
    action: Action,
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
}

fn parse_rule(input: &str) -> Rule {
    match input.split_once(":") {
        Some((pred, action)) => Rule {
            pred: parse_predicate(pred),
            action: parse_action(action),
        },
        None => Rule {
            pred: Predicate::Any,
            action: parse_action(input),
        },
    }
}

fn parse_predicate(pred: &str) -> Predicate {
    if let Some((var, num)) = pred.split_once(">") {
        Predicate::Greater(var.to_owned(), num.parse().unwrap())
    } else if let Some((var, num)) = pred.split_once("<") {
        Predicate::Less(var.to_owned(), num.parse().unwrap())
    } else {
        panic!();
    }
}

fn parse_action(input: &str) -> Action {
    match input {
        "A" => Action::Accept,
        "R" => Action::Reject,
        _ => Action::Goto(input.to_owned()),
    }
}

fn parse_workflow(line: &str) -> (&str, Workflow) {
    let (name, rest) = line.split_once("{").unwrap();
    let rules = rest
        .strip_suffix("}")
        .unwrap()
        .split(",")
        .map(|part| parse_rule(part));
    (
        name,
        Workflow {
            rules: rules.collect(),
        },
    )
}

impl Workflow {
    fn run(&self, workflows: &HashMap<&str, Workflow>, input: [i64; 4]) -> bool {
        for rule in &self.rules {
            if rule.pred.eval(input) {
                match &rule.action {
                    Action::Reject => return false,
                    Action::Accept => return true,
                    Action::Goto(workflow) => return workflows[&**workflow].run(workflows, input),
                }
            }
        }
        panic!()
    }

    fn run2(
        &self,
        workflows: &HashMap<&str, Workflow>,
        range: [Range<i64>; 4],
    ) -> Vec<[Range<i64>; 4]> {
        eprintln!("run2: {self:?} {range:?}");
        let mut accepted_ranges = Vec::new();
        let mut ranges = vec![range];
        for rule in &self.rules {
            if ranges.is_empty() {
                eprintln!("short circuiting");
                break;
            }
            eprintln!("rule={rule:?} ranges={ranges:?}");
            let mut all_matched_ranges = Vec::new();
            for range in ranges.iter().cloned() {
                let matched_ranges = rule.pred.eval2(range.clone());
                all_matched_ranges.extend(matched_ranges.clone());
                for matched_range in matched_ranges {
                    match &rule.action {
                        Action::Reject => {
                            // TODO
                        }
                        Action::Accept => {
                            eprintln!("accepting {matched_range:?}!");
                            accepted_ranges.push(matched_range);
                        }
                        Action::Goto(workflow) => {
                            eprintln!("gotoing {workflow:?} with {matched_range:?}!");
                            accepted_ranges.extend(
                                workflows[&**workflow].run2(workflows, matched_range.clone()),
                            );
                            // TODO reject
                        }
                    }
                }

                // TODO now how do we process the unmatched ranges???
            }
            let mut new_ranges = Vec::new();
            for range in ranges.iter().cloned() {
                for matched_range in all_matched_ranges.clone() {
                    new_ranges.extend(difference2(range.clone(), matched_range))
                }
            }
            ranges = new_ranges;
            eprintln!("rule={rule:?} new_ranges={ranges:?}");
        }
        accepted_ranges
    }
}

fn solve_p1(input: &str) -> i64 {
    let (workflows, inputs) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .split("\n")
        .map(|text| parse_workflow(text))
        .collect::<HashMap<_, _>>();
    //let result = workflows["in"].run2(&workflows, [1..4001, 1..4001, 1..4001, 1..4001]);
    //panic!("{result:?}");

    inputs
        .lines()
        .map(|line| {
            <[i64; 4]>::try_from(
                line.strip_prefix("{")
                    .unwrap()
                    .strip_suffix("}")
                    .unwrap()
                    .split(",")
                    .map(|field| field.split_once("=").unwrap().1.parse().unwrap())
                    .collect::<Vec<_>>(),
            )
            .unwrap()
        })
        //.filter(|&input| {
        //    !workflows["in"]
        //        .run2(
        //            &workflows,
        //            [
        //                input[0]..(input[0] + 1),
        //                input[1]..(input[1] + 1),
        //                input[2]..(input[2] + 1),
        //                input[3]..(input[3] + 1),
        //            ],
        //        )
        //        .is_empty()
        //})
        .filter(|&input| workflows["in"].run(&workflows, input))
        .map(|array| array.into_iter().sum::<i64>())
        .sum::<i64>()
}

fn solve_p2(input: &str) -> i64 {
    let (workflows, _inputs) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .split("\n")
        .map(|text| parse_workflow(text))
        .collect::<HashMap<_, _>>();
    let result = workflows["in"].run2(&workflows, [1..4001, 1..4001, 1..4001, 1..4001]);
    eprintln!("{result:?}");
    result
        .into_iter()
        .map(|ranges| {
            ranges
                .map(|range| range.end - range.start)
                .iter()
                .copied()
                .product::<i64>()
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
                "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
            ),
            19114
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve_p2(
                "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
            ),
            19114
        );
    }
}

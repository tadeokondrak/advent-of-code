use std::{
    cmp::{max, min},
    collections::HashMap,
    io::{stdin, Read},
};

#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: i64,
    end: i64,
}

impl std::fmt::Debug for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {})", self.start, self.end)
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Split<T> {
    matching: T,
    not_matching: T,
}

impl Range {
    fn len(self) -> i64 {
        assert!(self.start <= self.end);
        self.end - self.start
    }

    fn is_empty(self) -> bool {
        self.len() == 0
    }

    fn split(self, predicate: Predicate) -> Split<Range> {
        match predicate {
            Predicate::Any => Split {
                matching: self,
                not_matching: Range::default(),
            },
            Predicate::Less(_, v) => Split {
                matching: Range {
                    start: min(self.start, v),
                    end: min(self.end, v),
                },
                not_matching: Range {
                    start: max(self.start, v),
                    end: max(max(self.start, v), self.end),
                },
            },
            Predicate::Greater(_, v) => Split {
                matching: Range {
                    start: max(self.start, v + 1),
                    end: max(self.end, v + 1),
                },
                not_matching: Range {
                    start: min(min(self.end, v + 1), self.start),
                    end: min(self.end, v + 1),
                },
            },
        }
    }
}

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

#[derive(Debug, Copy, Clone)]
enum Predicate {
    Any,
    Less(usize, i64),
    Greater(usize, i64),
}

impl Predicate {
    fn eval(self, input: [Range; 4]) -> Split<[Range; 4]> {
        for i in 0..4 {
            match self {
                Predicate::Less(j, _) | Predicate::Greater(j, _) if i == j => {
                    let Split {
                        matching,
                        not_matching,
                    } = input[i].split(self);
                    let mut matching_input = input;
                    let mut not_matching_input = input;
                    matching_input[i] = matching;
                    not_matching_input[i] = not_matching;
                    return Split {
                        matching: matching_input,
                        not_matching: not_matching_input,
                    };
                }
                _ => {}
            };
        }
        Split {
            matching: input,
            not_matching: Default::default(),
        }
    }
}

fn letter_index(var: &str) -> Option<usize> {
    ["x", "m", "a", "s"].into_iter().position(|it| it == var)
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
        let i = letter_index(var).unwrap();
        Predicate::Greater(i, num.parse().unwrap())
    } else if let Some((var, num)) = pred.split_once("<") {
        let i = letter_index(var).unwrap();
        Predicate::Less(i, num.parse().unwrap())
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
    fn run(
        &self,
        workflows: &HashMap<&str, Workflow>,
        input: [Range; 4],
        out: &mut Vec<[Range; 4]>,
    ) {
        let mut cur = input;
        for rule in &self.rules {
            let Split {
                matching,
                not_matching,
            } = rule.pred.eval(cur);
            if matching.iter().any(|r| r.is_empty()) {
                continue;
            }
            match &rule.action {
                Action::Reject => {}
                Action::Accept => {
                    out.push(matching);
                }
                Action::Goto(workflow) => {
                    workflows[workflow.as_str()].run(workflows, matching, out);
                }
            }
            cur = not_matching;
        }
        assert!(cur.into_iter().any(|x| x.is_empty()));
    }
}

fn count_possibilities(matching: [Range; 4]) -> i64 {
    matching.into_iter().map(|range| range.len()).product()
}

fn parse_input_line(line: &str) -> [Range; 4] {
    line.strip_prefix("{")
        .unwrap()
        .strip_suffix("}")
        .unwrap()
        .split(",")
        .map(|field| field.split_once("=").unwrap().1.parse().unwrap())
        .map(|value| Range {
            start: value,
            end: value + 1,
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn solve_p1(input: &str) -> i64 {
    let (workflows, inputs) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .split("\n")
        .map(|text| parse_workflow(text))
        .collect::<HashMap<_, _>>();
    inputs
        .lines()
        .map(|line| parse_input_line(line))
        .filter(|&input| {
            let mut out = Vec::new();
            workflows["in"].run(&workflows, input, &mut out);
            out.iter().any(|&output| count_possibilities(output) > 0)
        })
        .map(|input| {
            input
                .into_iter()
                .inspect(|x| assert_eq!(x.len(), 1))
                .map(|x| x.start)
                .sum::<i64>()
        })
        .sum::<i64>()
}

fn solve_p2(input: &str) -> i64 {
    let (workflows, _inputs) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .split("\n")
        .map(|text| parse_workflow(text))
        .collect::<HashMap<_, _>>();
    let range = Range {
        start: 1,
        end: 4001,
    };
    let ranges = [range; 4];
    let mut out = Vec::new();
    workflows["in"].run(&workflows, ranges, &mut out);
    out.iter()
        .map(|&good_range| count_possibilities(good_range))
        .sum::<i64>()
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
            167409079868000
        );
    }
}

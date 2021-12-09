/*
  the code in this file is not good.
  it was written as fast as possible and it will never be used again
*/

fn main() {
    aoc_driver::aoc_complete! {
        session_file: ".session"
        input_dir: "input"
        challenges: [
            {
                "2021-9-2": day9part2,
                tests: [
                    { name: "0", input: "2199943210
3987894921
9856789892
8767896789
9899965678", output: "1134" }
                ]
            }
            {
                "2021-9-1": day9part1,
                tests: [
                    { name: "0", input: "2199943210
3987894921
9856789892
8767896789
9899965678", output: "15" }
                ]
            }
            {
                "2021-8-2": day8part2,
                tests: [
                    { name: "0", input: "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf", output: "5353" }
                    { name: "1", input: "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce", output: "61229" }
                ]
            }
            {
                "2021-8-1": day8part1,
                tests: [
                    { name: "1", input: "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce", output: "26" }
                ]
            }
            {
                "2021-7-2": day7part2,
                tests: [
                    { name: "1", input: "16,1,2,0,4,2,7,1,2,14", output: "168" }
                ]
            }
            {
                "2021-7-1": day7part1,
                tests: [
                    { name: "1", input: "16,1,2,0,4,2,7,1,2,14", output: "37" }
                ]
            }
            {
                "2021-6-2": day6part2,
                tests: [
                    { name: "1", input: "3,4,3,1,2", output: "26984457539" }
                ]
            }
            {
                "2021-6-1": day6part1,
                tests: [
                    { name: "1", input: "3,4,3,1,2", output: "5934" }
                ]
            }
            {
                "2021-5-2": day5part2,
                tests: [
                    { name: "1", input: "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2", output: "12" }
                ]
            }
            {
                "2021-5-1": day5part1,
                tests: [
                    { name: "1", input: "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2", output: "5" }
                ]
            }
            {
                "2021-4-2": day4part2,
                tests: [
                    { name: "1", input: "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7", output: "1924" }
                ]
            }
            {
                "2021-4-1": day4part1,
                tests: [
                    { name: "1", input: "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7", output: "4512" }
                ]
            }
            {
                "2021-3-2": day3part2,
                tests: [
                    { name: "1", input: "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010", output: "230" }
                ]
            }
            {
                "2021-3-1": day3part1,
                tests: [
                    { name: "1", input: "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010", output: "198" }
                ]
            }            {
                "2021-2-2": day2part2,
                tests: [
                    { name: "1", input: "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2", output: "900" }
                ]
            }
            {
                "2021-2-1": day2part1,
                tests: [
                    { name: "1", input: "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2", output: "150" }
                ]
            }
            {
                "2021-1-2": day1part2,
                tests: [
                    { name: "1", input: "199\n200\n208\n210\n200\n207\n240\n269\n260\n263", output: "5" }
                ]
            }
            {
                "2021-1-1": day1part1,
                tests: [
                    { name: "1", input: "199\n200\n208\n210\n200\n207\n240\n269\n260\n263", output: "7" }
                ]
            }
        ]
    }
}

fn day9part2(input: &str) -> usize {
    use std::collections::BTreeSet;
    let input = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let width = input[0].len();
    let height = input.len();
    let mut low_points = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let h = input[y][x];
            let left_neighbor = (x != 0).then(|| input[y][x - 1]);
            let right_neighbor = (x + 1 < width).then(|| input[y][x + 1]);
            let lower_neighbor = (y != 0).then(|| input[y - 1][x]);
            let upper_neighbor = (y + 1 < height).then(|| input[y + 1][x]);
            let is_low = left_neighbor.map_or(true, |n| h < n)
                && right_neighbor.map_or(true, |n| h < n)
                && lower_neighbor.map_or(true, |n| h < n)
                && upper_neighbor.map_or(true, |n| h < n);
            if is_low {
                low_points.push((x, y));
            }
        }
    }
    let mut sums = Vec::new();
    for (x, y) in low_points {
        fn visit(
            input: &Vec<Vec<usize>>,
            width: usize,
            height: usize,
            visited: &mut BTreeSet<(usize, usize)>,
            (x, y): (usize, usize),
        ) -> usize {
            if !visited.insert((x, y)) {
                return 0;
            }
            if input[y][x] == 9 {
                return 0;
            }
            let mut sum = 1;
            sum += (x != 0)
                .then(|| visit(input, width, height, visited, (x - 1, y)))
                .unwrap_or(0);
            sum += (x + 1 != width)
                .then(|| visit(input, width, height, visited, (x + 1, y)))
                .unwrap_or(0);
            sum += (y != 0)
                .then(|| visit(input, width, height, visited, (x, y - 1)))
                .unwrap_or(0);
            sum += (y + 1 != height)
                .then(|| visit(input, width, height, visited, (x, y + 1)))
                .unwrap_or(0);
            sum
        }
        sums.push(visit(&input, width, height, &mut BTreeSet::new(), (x, y)));
    }
    sums.sort_unstable();
    sums.iter().rev().take(3).product::<usize>()
}

fn day9part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let width = input[0].len();
    let height = input.len();
    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            let h = input[y][x];
            let left_neighbor = (x != 0).then(|| input[y][x - 1]);
            let right_neighbor = (x + 1 < width).then(|| input[y][x + 1]);
            let lower_neighbor = (y != 0).then(|| input[y - 1][x]);
            let upper_neighbor = (y + 1 < height).then(|| input[y + 1][x]);
            let is_low = left_neighbor.map_or(true, |n| h < n)
                && right_neighbor.map_or(true, |n| h < n)
                && lower_neighbor.map_or(true, |n| h < n)
                && upper_neighbor.map_or(true, |n| h < n);
            if is_low {
                let risk = 1 + h;
                sum += risk;
            }
        }
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

fn day8part2(input: &str) -> usize {
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

fn day8part1(input: &str) -> usize {
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

fn day7part2(input: &str) -> i32 {
    let crabs = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let biggest_crab = crabs.iter().copied().max().unwrap();
    let mut list = Vec::new();
    for i in 0..biggest_crab {
        let mut fuel_needed = 0;
        for &crab in &crabs {
            let dist = (crab - i).abs();
            let cost = dist * (dist + 1) / 2;
            fuel_needed += cost;
        }
        list.push((fuel_needed, i));
    }
    list.sort();
    list[0].0
}

fn day7part1(input: &str) -> i32 {
    let crabs = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let biggest_crab = crabs.iter().copied().max().unwrap();
    let mut list = Vec::new();
    for i in 0..biggest_crab {
        let mut fuel_needed = 0;
        for &crab in &crabs {
            fuel_needed += (crab - i).abs();
        }
        list.push((fuel_needed, i));
    }
    list.sort();
    list[0].0
}

fn day6(input: &str, iterations: usize) -> u64 {
    let mut fishies = [0u64; 9];
    for fishy in input.trim().split(",").map(|x| x.parse::<usize>().unwrap()) {
        fishies[fishy] += 1;
    }
    for day in 0.. {
        let sum = fishies.iter().sum::<u64>();
        if day == iterations {
            return sum;
        }
        let tmp = fishies[0];
        for i in 0..8 {
            fishies[i] = fishies[i + 1];
        }
        fishies[6] += tmp;
        fishies[8] = tmp;
    }
    todo!()
}

fn day6part2(input: &str) -> u64 {
    day6(input, 256)
}

fn day6part1(input: &str) -> usize {
    let mut fishies = input
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    for day in 1.. {
        let mut new = 0;
        for fishy in fishies.iter_mut() {
            if *fishy == 0 {
                *fishy = 6;
                new += 1;
            } else {
                *fishy -= 1;
            }
        }
        for _ in 0..new {
            fishies.push(8);
        }
        if day == 80 {
            return fishies.len();
        }
    }
    panic!()
}

struct VentMap {
    map: Vec<u32>,
}

const SIZE: u32 = 1000;
impl VentMap {
    fn parse(s: &str) -> Self {
        let mut this = Self {
            map: vec![0; (SIZE * SIZE) as usize],
        };
        for line in s.lines() {
            let (first_pair, second_pair) = line.split_once(" -> ").unwrap();
            let (first_x, first_y) = first_pair.split_once(",").unwrap();
            let (second_x, second_y) = second_pair.split_once(",").unwrap();
            let mut first_x = first_x.parse::<u32>().unwrap();
            let mut first_y = first_y.parse::<u32>().unwrap();
            let mut second_x = second_x.parse::<u32>().unwrap();
            let mut second_y = second_y.parse::<u32>().unwrap();
            if first_x > second_x {
                std::mem::swap(&mut first_x, &mut second_x);
            }
            if first_y > second_y {
                std::mem::swap(&mut first_y, &mut second_y);
            }
            if first_x == second_x {
                for y in first_y..=second_y {
                    *this.get_mut(first_x, y) += 1;
                }
            } else if first_y == second_y {
                for x in first_x..=second_x {
                    *this.get_mut(x, second_y) += 1;
                }
            } else {
            }
            //this.dump();
        }
        this
    }

    fn parse2(s: &str) -> Self {
        let mut this = Self {
            map: vec![0; (SIZE * SIZE) as usize],
        };
        for line in s.lines() {
            let (first_pair, second_pair) = line.split_once(" -> ").unwrap();
            let (first_x, first_y) = first_pair.split_once(",").unwrap();
            let (second_x, second_y) = second_pair.split_once(",").unwrap();
            let mut first_x = first_x.parse::<u32>().unwrap();
            let mut first_y = first_y.parse::<u32>().unwrap();
            let mut second_x = second_x.parse::<u32>().unwrap();
            let mut second_y = second_y.parse::<u32>().unwrap();
            if first_x == second_x {
                if first_y > second_y {
                    std::mem::swap(&mut first_y, &mut second_y);
                }
                for y in first_y..=second_y {
                    *this.get_mut(first_x, y) += 1;
                }
            } else if first_y == second_y {
                if first_x > second_x {
                    std::mem::swap(&mut first_x, &mut second_x);
                }
                for x in first_x..=second_x {
                    *this.get_mut(x, second_y) += 1;
                }
            } else {
                //this.dump();
                let x_dir: i32 = match second_x.cmp(&first_x) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => panic!(),
                    std::cmp::Ordering::Greater => 1,
                };
                let y_dir: i32 = match second_y.cmp(&first_y) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => panic!(),
                    std::cmp::Ordering::Greater => 1,
                };
                let mut x = if x_dir == 1 {
                    first_x.min(second_x)
                } else {
                    first_x.max(second_x)
                };
                let mut y = if y_dir == 1 {
                    first_y.min(second_y)
                } else {
                    first_y.max(second_y)
                };
                let x_range = first_x.min(second_x)..=second_x.max(first_x);
                let y_range = first_y.min(second_y)..=second_y.max(first_y);
                loop {
                    let old_x = x;
                    let old_y = y;
                    let next_x = (x as i32 + x_dir) as u32;
                    let next_y = (y as i32 + y_dir) as u32;
                    *this.get_mut(x, y) += 1;
                    if x_range.contains(&next_x) {
                        x = next_x;
                    }
                    if y_range.contains(&next_y) {
                        y = next_y;
                    }
                    if old_x == x && old_y == y {
                        break;
                    }
                }
                //this.dump();
            }
        }
        this
    }

    fn get(&self, x: u32, y: u32) -> u32 {
        self.map[(x * SIZE + y) as usize]
    }

    fn get_mut(&mut self, x: u32, y: u32) -> &mut u32 {
        &mut self.map[(x * SIZE + y) as usize]
    }

    fn part1(&self) -> usize {
        self.map.iter().filter(|&&x| x > 1).count()
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for y in 0..SIZE {
            for x in 0..SIZE {
                let n = self.get(x, y);
                if n == 0 {
                    eprint!(".");
                } else {
                    eprint!("{}", n);
                }
            }
            eprintln!();
        }
        eprintln!();
    }
}

fn day5part2(input: &str) -> usize {
    let map = VentMap::parse2(input);
    map.part1()
}

fn day5part1(input: &str) -> usize {
    let map = VentMap::parse(input);
    map.part1()
}

#[derive(Clone)]
struct Board {
    nums: Vec<(u32, bool)>,
}

impl Board {
    fn parse(s: &str) -> Self {
        Self {
            nums: s
                .split(&[' ', '\n'])
                .map(|x| x.trim_end())
                .filter(|x| !x.is_empty())
                .map(|x| (x.parse::<u32>().unwrap(), false))
                .collect(),
        }
    }

    fn call_number(&mut self, calling: u32) {
        for (num, is_marked) in &mut self.nums {
            if *num == calling {
                *is_marked = true;
            }
        }
    }

    fn is_marked(&self, row: usize, col: usize) -> bool {
        self.nums[row * 5 + col].1
    }

    fn check_rows(&self) -> bool {
        'outer: for row in 0..5 {
            for col in 0..5 {
                if !self.is_marked(row, col) {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }

    fn check_cols(&self) -> bool {
        'outer: for col in 0..5 {
            for row in 0..5 {
                if !self.is_marked(row, col) {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }

    fn score(&self, last_called: u32) -> Option<u32> {
        if !self.check_rows() && !self.check_cols() {
            return None;
        }
        let sum: u32 = self
            .nums
            .iter()
            .filter(|(_, is_marked)| !*is_marked)
            .map(|(x, _)| u32::from(*x))
            .sum();
        Some(sum * u32::from(last_called))
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..5 {
            for col in 0..5 {
                let (num, marked) = self.nums[row * 5 + col];
                if !marked {
                    write!(f, "{:2}", num)?;
                } else {
                    write!(f, "\x1b[31;1m{:2}\x1b[0m", num)?;
                }
                if col != 4 {
                    write!(f, " ")?;
                }
            }
            if row != 5 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

fn day4part2(input: &str) -> u32 {
    let mut iter = input.split("\n\n");
    let sequence = iter
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap());
    let mut boards = Vec::new();
    while let Some(board_str) = iter.next() {
        boards.push(Some(Board::parse(board_str)));
    }
    let mut last_win = None;
    for num in sequence {
        for slot in &mut boards {
            if let Some(board) = slot {
                board.call_number(num);
                if let Some(score) = board.score(num) {
                    *slot = None;
                    last_win = Some(score);
                }
            }
        }
    }
    last_win.unwrap()
}

fn day4part1(input: &str) -> u32 {
    let mut iter = input.split("\n\n");
    let sequence = iter
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap());
    let mut boards = Vec::new();
    while let Some(board_str) = iter.next() {
        boards.push(Board::parse(board_str));
    }
    for num in sequence {
        for board in &mut boards {
            board.call_number(num);
            if let Some(score) = board.score(num) {
                return score;
            }
        }
    }
    panic!()
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

fn day3part2(input: &str) -> u32 {
    let lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    let vals: Vec<u32> = lines
        .iter()
        .map(|x| u32::from_str_radix(&x, 2).unwrap())
        .collect();
    let count = lines[0].len();
    rating(0, count, &vals, Rating::Oxygen) * rating(0, count, &vals, Rating::Co2)
}

fn day3part1(input: &str) -> u32 {
    let lines: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    let vals: Vec<u32> = lines
        .iter()
        .map(|x| u32::from_str_radix(&x, 2).unwrap())
        .collect();
    let count = lines[0].len();
    let (ones, nones) = ones_nones(count, &vals);
    value(count, &ones, &nones, Value::Gamma) * value(count, &ones, &nones, Value::Epsilon)
}

fn day2part2(input: &str) -> i32 {
    let mut aim = 0;
    let mut xpos = 0;
    let mut ypos = 0;
    for cmd in input.lines() {
        let mut split = cmd.split(' ');
        let cmd = split.next().unwrap();
        let val: i32 = split.next().unwrap().parse().unwrap();
        match cmd {
            "forward" => {
                xpos += val;
                ypos += val * aim;
            }
            "down" => {
                aim += val;
            }
            "up" => {
                aim -= val;
            }
            _ => panic!(),
        }
    }
    xpos * ypos
}

fn day2part1(input: &str) -> i32 {
    let mut xpos = 0;
    let mut ypos = 0;
    for cmd in input.lines() {
        let mut split = cmd.split(' ');
        let cmd = split.next().unwrap();
        let val: i32 = split.next().unwrap().parse().unwrap();
        match cmd {
            "forward" => {
                xpos += val;
            }
            "down" => {
                ypos += val;
            }
            "up" => {
                ypos -= val;
            }
            _ => panic!(),
        }
    }
    xpos * ypos
}

fn day1part2(input: &str) -> i32 {
    let mut last_sum = None;
    let mut n = 0;
    let vals: Vec<i32> = input.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    for arr in vals.windows(3) {
        let sum = arr[0] + arr[1] + arr[2];
        if let Some(last_sum) = last_sum {
            if sum > last_sum {
                n += 1;
            }
        }
        last_sum = Some(sum);
    }
    n
}

fn day1part1(input: &str) -> i32 {
    let mut n = 0;
    let mut last_val = None;
    for val in input.lines().map(|x| x.parse::<i32>().unwrap()) {
        if let Some(last_val) = last_val {
            if val > last_val {
                n += 1;
            }
        }
        last_val = Some(val);
    }
    n
}

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
                todo!()
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

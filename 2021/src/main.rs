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

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve_p1(&input));
    println!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> usize {
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

fn solve_p2(input: &str) -> u64 {
    day6(input, 256)
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3,4,3,1,2";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(INPUT), 5934);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_p2(INPUT), 26984457539);
    }
}

use std::{
    io::{self, Read},
    ops::RangeFrom,
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().into())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let mut total = 0i64;
    for line in lines {
        let mut largest = 0;
        for i in 0..line.len() {
            for j in i + 1..line.len() {
                largest = largest.max(line[i] * 10 + line[j]);
            }
        }
        total += largest;
    }
    total
}

// brute force
fn part2(input: &str) -> i64 {
    fn go(
        line: &[i64],
        amt: i64,
        start: RangeFrom<usize>,
        remaining: i64,
        largest_seen: &mut i64,
    ) -> i64 {
        let shifted = amt * 10i64.pow((remaining) as u32);
        let nines = 10i64.pow((remaining) as u32) - 1;
        let largest_possible = shifted + nines;
        if largest_possible < *largest_seen {
            return 0;
        }
        if remaining == 0 || line[start.clone()].is_empty() {
            return amt;
        }
        let without_first = go(
            line,
            amt,
            start.clone().start + 1..,
            remaining,
            largest_seen,
        );
        let with_first = go(
            line,
            amt * 10 + line[start.clone()][0],
            start.clone().start + 1..,
            remaining - 1,
            largest_seen,
        );
        let result = with_first.max(without_first);
        *largest_seen = (*largest_seen).max(result);
        result
    }
    let lines = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().into())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let total = std::sync::atomic::AtomicI64::new(0);
    std::thread::scope(|s| {
        for line in lines {
            let total = &total;
            s.spawn(move || {
                let largest = go(&line, 0, 0.., 12, &mut 0);
                total.fetch_add(largest, std::sync::atomic::Ordering::SeqCst);
            });
        }
    });
    total.load(std::sync::atomic::Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 3121910778619);
    }
}

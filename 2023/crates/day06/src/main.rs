use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> u64 {
    let (time_line, dist_line) = input.trim().split_once("\n").unwrap();
    let times = time_line
        .strip_prefix("Time:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let dists = dist_line
        .strip_prefix("Distance:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    times
        .iter()
        .copied()
        .zip(dists.iter().copied())
        .map(|(race_time, dist_record)| solve(race_time, dist_record))
        .product()
}

fn solve_p2(input: &str) -> u64 {
    let (time_line, dist_line) = input.trim().split_once("\n").unwrap();
    let race_time = time_line
        .strip_prefix("Time:")
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let dist_record = dist_line
        .strip_prefix("Distance:")
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    solve(race_time, dist_record)
}

fn solve(race_time: u64, dist_record: u64) -> u64 {
    // x = hold_time
    // C = race_time
    // D = dist_record
    // -x^2 + C * x - D = 0

    let a = -1.0;
    let b = race_time as f64;
    let c = -((dist_record + 1) as f64);

    let x_0 = (-b + f64::sqrt(b * b - 4.0 * a * c)) / (2.0 * a);
    let x_1 = (-b - f64::sqrt(b * b - 4.0 * a * c)) / (2.0 * a);

    (x_1.floor() - x_0.ceil() + 1.0) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "
Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(INPUT), 288);
    }
    #[test]
    fn part_2() {
        assert_eq!(solve_p2(INPUT), 71503);
    }
}

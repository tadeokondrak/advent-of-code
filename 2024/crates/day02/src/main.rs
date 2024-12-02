use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut count = 0;
    for report in input.lines() {
        let mut safe = true;
        let mut increasing = false;
        let mut decreasing = false;
        let mut last_level = None;
        for level in report.split_ascii_whitespace() {
            let level = level.parse::<i32>().unwrap();
            if let Some(last_level) = last_level {
                if level > last_level {
                    increasing = true;
                }
                if level < last_level {
                    decreasing = true;
                }
                let diff = level.abs_diff(last_level);
                if diff < 1 || diff > 3 {
                    safe = false;
                }
            }
            last_level = Some(level);
        }
        if increasing && decreasing {
            safe = false;
        }
        count += safe as i32;
    }
    count
}

fn part2(input: &str) -> i32 {
    let mut count = 0;
    for report in input.lines() {
        let levels = report
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut any_safe = false;
        for i in 0..levels.len() {
            let mut levels_ = levels.clone();
            levels_.remove(i);
            let safe = check_report(&levels_);
            any_safe |= safe;
        }

        count += any_safe as i32;
    }

    count
}

fn check_report(levels: &[i32]) -> bool {
    let mut safe = true;
    let mut increasing = false;
    let mut decreasing = false;
    let mut last_level = None;
    for &level in levels {
        if let Some(last_level) = last_level {
            if level > last_level {
                increasing = true;
            }
            if level < last_level {
                decreasing = true;
            }
            let diff = level.abs_diff(last_level);
            if diff < 1 || diff > 3 {
                safe = false;
            }
        }
        last_level = Some(level);
    }
    if increasing && decreasing {
        safe = false;
    }
    safe
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}

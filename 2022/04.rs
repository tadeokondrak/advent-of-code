use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        let (left_s, right_s) = line.split_once(',').unwrap();
        let (l_start, l_end) = left_s.split_once('-').unwrap();
        let (r_start, r_end) = right_s.split_once('-').unwrap();
        let (l_start, l_end, r_start, r_end) = (
            l_start.parse::<i32>().unwrap(),
            l_end.parse::<i32>().unwrap(),
            r_start.parse::<i32>().unwrap(),
            r_end.parse::<i32>().unwrap(),
        );
        let left = l_start..=l_end;
        let right = r_start..=r_end;
        if (left.contains(right.start()) && left.contains(right.end()))
            || (right.contains(left.start()) && right.contains(left.end()))
        {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> i32 {
    let mut count = 0;
    for line in input.lines() {
        let (left_s, right_s) = line.split_once(',').unwrap();
        let (l_start, l_end) = left_s.split_once('-').unwrap();
        let (r_start, r_end) = right_s.split_once('-').unwrap();
        let (l_start, l_end, r_start, r_end) = (
            l_start.parse::<i32>().unwrap(),
            l_end.parse::<i32>().unwrap(),
            r_start.parse::<i32>().unwrap(),
            r_end.parse::<i32>().unwrap(),
        );
        let left = l_start..=l_end;
        let right = r_start..=r_end;
        if left.contains(right.start())
            || left.contains(right.end())
            || right.contains(left.start())
            || right.contains(left.end())
        {
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}

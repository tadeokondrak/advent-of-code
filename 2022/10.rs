use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2:\n{}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut x = 1;
    let mut cycle = 1;
    let mut pending = None;
    let mut solution = 0;
    let mut lines = input.lines();
    loop {
        if matches!(cycle, 20 | 60 | 100 | 140 | 180 | 220) {
            solution += x * cycle;
        }
        if let Some(num) = pending.take() {
            x += num;
        } else if let Some(line) = lines.next() {
            let args = line.split(" ").collect::<Vec<_>>();
            match *args {
                ["noop"] => {}
                ["addx", num] => {
                    pending = Some(num.parse::<i32>().unwrap());
                }
                _ => panic!(),
            }
        } else {
            break;
        }
        cycle += 1;
    }
    solution
}

fn part2(input: &str) -> String {
    let mut x = 1;
    let mut cycle = 1;
    let mut pending = None;
    let mut lines = input.lines();
    let mut grid = [false; 40 * 6];
    loop {
        if let Some(num) = pending.take() {
            x += num;
        } else if let Some(line) = lines.next() {
            let args = line.split(" ").collect::<Vec<_>>();
            match *args {
                ["noop"] => {}
                ["addx", num] => {
                    pending = Some(num.parse::<i32>().unwrap());
                }
                _ => panic!(),
            }
        } else {
            break;
        }
        if cycle < grid.len() {
            grid[cycle] = (cycle as i32 % 40) == x
                || (cycle as i32 % 40) == x - 1
                || (cycle as i32 % 40) == x + 1;
        }
        cycle += 1;
    }
    let mut out = String::new();
    for y in 0..6 {
        for x in 0..40 {
            out += if grid[y * 40 + x] { "#" } else { "." };
        }
        if y != 5 {
            out += "\n";
        }
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13140);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(TEST_INPUT),
            "##..##..##..##..##..##..##..##..##..##..\n\
             ###...###...###...###...###...###...###.\n\
             ####....####....####....####....####....\n\
             #####.....#####.....#####.....#####.....\n\
             ######......######......######......####\n\
             #######.......#######.......#######....."
        );
    }
}

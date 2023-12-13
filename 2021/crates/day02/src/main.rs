use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve_p1(&input));
    println!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> i32 {
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

fn solve_p2(input: &str) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(INPUT), 150);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_p2(INPUT), 900);
    }
}

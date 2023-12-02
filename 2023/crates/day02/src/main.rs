use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> u32 {
    let mut good_sum = 0;

    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let (_, rest) = line.split_once("Game ").unwrap();
        let (game_id, rest) = rest.split_once(": ").unwrap();
        let game_id = game_id.parse::<u32>().unwrap();
        let mut bad = false;
        for set in rest.split("; ") {
            for thing in set.split(", ") {
                let (count, color) = thing.split_once(" ").unwrap();
                let count = count.parse::<u32>().unwrap();
                match color {
                    "red" => bad |= count > 12,
                    "green" => bad |= count > 13,
                    "blue" => bad |= count > 14,
                    other => unreachable!("bad color {other}"),
                }
            }
        }
        if !bad {
            good_sum += game_id;
        }
    }

    good_sum
}

fn solve_p2(input: &str) -> u32 {
    let mut good_sum = 0;

    for line in input.lines().filter(|line| !line.trim().is_empty()) {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        let (_, rest) = line.split_once("Game ").unwrap();
        let (_, rest) = rest.split_once(": ").unwrap();
        for set in rest.split("; ") {
            for thing in set.split(", ") {
                let (count, color) = thing.split_once(" ").unwrap();
                let count = count.parse::<u32>().unwrap();
                match color {
                    "red" => min_red = min_red.max(count),
                    "green" => min_green = min_green.max(count),
                    "blue" => min_blue = min_blue.max(count),
                    other => unreachable!("bad color {other}"),
                }
            }
        }
        good_sum += min_red * min_blue * min_green;
    }
    good_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            solve_p1(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            ),
            8
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve_p2(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            ),
            2286
        );
    }
}

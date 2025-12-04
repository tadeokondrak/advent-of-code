use std::io::{self, Read};
use util::{Grid, offset, point};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let grid = Grid::parse(input, |c| c);
    let mut count = 0;
    for y in 0..grid.height as i32 {
        for x in 0..grid.width as i32 {
            if grid.get(point(x, y)).copied() != Some('@') {
                continue;
            }
            let mut inner_count = 0;
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if grid.get(point(x, y) + offset(dx, dy)).copied() == Some('@') {
                        inner_count += 1;
                    }
                }
            }
            if inner_count < 4 {
                count += 1;
            }
        }
    }
    count
}

fn part2(input: &str) -> i64 {
    let mut grid = Grid::parse(input, |c| c);
    let mut count = 0;
    loop {
        let mut did_anything = false;
        for y in 0..grid.height as i32 {
            for x in 0..grid.width as i32 {
                if grid.get(point(x, y)).copied() != Some('@') {
                    continue;
                }
                let mut inner_count = 0;
                for dx in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if grid.get(point(x, y) + offset(dx, dy)).copied() == Some('@') {
                            inner_count += 1;
                        }
                    }
                }
                if inner_count < 4 {
                    count += 1;
                    grid.set(point(x, y), '.');
                    did_anything = true;
                }
            }
        }
        if !did_anything {
            return count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 3121910778619);
    }
}

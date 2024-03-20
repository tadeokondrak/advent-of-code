use std::{
    collections::HashSet,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve(&input, 64));
}

fn solve(input: &str, max_steps: i32) -> i32 {
    let width = input.lines().next().unwrap().len() as i32;
    let height = input.lines().count() as i32;
    let grid: Vec<u8> = input.bytes().filter(|&c| c != b'\n').collect();
    let start_idx = grid.iter().position(|&c| c == b'S').unwrap() as i32;
    let start_x = start_idx % width;
    let start_y = start_idx / width;
    let mut queue = Vec::new();
    queue.push((start_x, start_y, 0));
    let mut reachable = HashSet::new();
    let mut seen = HashSet::new();
    while let Some((x, y, steps)) = queue.pop() {
        if steps == max_steps {
            reachable.insert((x, y));
            continue;
        }
        let mut add = |dx, dy| {
            let next: (i32, i32, i32) = (x + dx, y + dy, steps + 1);
            if grid[((next.1.rem_euclid(height)) * width + (next.0.rem_euclid(width))) as usize]
                != b'#'
                && seen.insert(next)
            {
                queue.push(next);
            }
        };
        add(-1, 0);
        add(1, 0);
        add(0, -1);
        add(0, 1);
    }
    reachable.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[test]
    fn part_1() {
        assert_eq!(solve(EXAMPLE_INPUT, 6), 16);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve(EXAMPLE_INPUT, 6), 16);
        assert_eq!(solve(EXAMPLE_INPUT, 10), 50);
        assert_eq!(solve(EXAMPLE_INPUT, 50), 1594);
        assert_eq!(solve(EXAMPLE_INPUT, 100), 6536);
        assert_eq!(solve(EXAMPLE_INPUT, 500), 167004);
        assert_eq!(solve(EXAMPLE_INPUT, 1000), 668697);
        assert_eq!(solve(EXAMPLE_INPUT, 5000), 16733044);
    }
}

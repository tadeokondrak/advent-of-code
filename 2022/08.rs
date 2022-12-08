use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut grid = Vec::new();
    let mut visible = Vec::new();
    let size = input.lines().next().unwrap().trim().len();
    for line in input.lines() {
        grid.extend(line.trim().chars().map(|c| c as i32 - b'0' as i32));
        visible.extend(line.trim().chars().map(|_| false));
    }
    for y in 0..size {
        let mut max = -1;
        for x in 0..size {
            let tree = grid[y * size + x];
            if tree > max {
                max = tree;
                visible[y * size + x] = true
            }
        }
    }
    for y in (0..size).rev() {
        let mut max = -1;
        for x in (0..size).rev() {
            let tree = grid[y * size + x];
            if tree > max {
                max = tree;
                visible[y * size + x] = true;
            }
        }
    }
    for x in 0..size {
        let mut max = -1;
        for y in 0..size {
            let tree = grid[y * size + x];
            if tree > max {
                max = tree;
                visible[y * size + x] = true;
            }
        }
    }
    for x in (0..size).rev() {
        let mut max = -1;
        for y in (0..size).rev() {
            let tree = grid[y * size + x];
            if tree > max {
                max = tree;
                visible[y * size + x] = true;
            }
        }
    }
    visible.iter().copied().filter(|&x| x).count()
}

fn part2(input: &str) -> i32 {
    fn check(
        grid: &[i32],
        size: isize,
        x: isize,
        y: isize,
        max: i32,
        xdir: isize,
        ydir: isize,
    ) -> i32 {
        let x_offset = x + xdir;
        let y_offset = y + ydir;
        if x_offset < 0 || x_offset >= size || y_offset < 0 || y_offset >= size {
            return 0;
        }
        let val = grid[(y_offset * size + x_offset) as usize];
        if val > max {
            return 1;
        }
        check(grid, size, x_offset, y_offset, max, xdir, ydir) + 1
    }
    fn score(grid: &[i32], size: isize, x: isize, y: isize) -> i32 {
        let max = grid[(y * size + x) as usize] - 1;
        check(grid, size, x, y, max, 1, 0)
            * check(grid, size, x, y, max, 0, 1)
            * check(grid, size, x, y, max, -1, 0)
            * check(grid, size, x, y, max, 0, -1)
    }
    let mut grid = Vec::new();
    let mut scores = Vec::new();
    let size = input.lines().next().unwrap().trim().len();
    for line in input.lines() {
        grid.extend(line.trim().chars().map(|c| c as i32 - b'0' as i32));
        scores.extend(line.trim().chars().map(|_| 0));
    }
    for y in 0..size {
        for x in 0..size {
            scores[y * size + x] = score(&grid, size as isize, x as isize, y as isize);
        }
    }
    scores.iter().copied().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 8);
    }
}

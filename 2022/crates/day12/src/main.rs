use util::{point, Point};

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn index(len: usize, stride: usize, point: Point) -> Option<usize> {
    if point.x < 0 || point.y < 0 {
        return None;
    }
    let x = point.x as usize;
    let y = point.y as usize;
    let i = y * stride + x;
    if i >= len {
        return None;
    }
    Some(i)
}

fn unindex(stride: usize, index: usize) -> Point {
    point((index % stride) as i32, (index / stride) as i32)
}

fn solve(stride: usize, grid: &[u8], start: Point, end: Point) -> Option<i32> {
    let mut visited = vec![false; grid.len()];
    let mut cost = vec![u32::MAX; grid.len()];
    let mut prev = vec![None::<Point>; grid.len()];
    cost[index(grid.len(), stride, start).unwrap()] = 0;
    while let Some(i) = (0..grid.len())
        .filter(|&i| !visited[i])
        .min_by_key(|&i| cost[i])
    {
        let current = unindex(stride, i);
        if current == end {
            break;
        }
        if cost[i] == u32::MAX {
            return None;
        }
        visited[i] = true;
        let to_update = [point(-1, 0), point(1, 0), point(0, 1), point(0, -1)]
            .into_iter()
            .map(|pt| current + pt)
            .filter_map(|pt| index(grid.len(), stride, pt))
            .filter(|&j| !visited[j])
            .filter(|&j| grid[j] <= grid[i] + 1)
            .collect::<Vec<_>>();
        for j in to_update {
            if cost[i] + 1 < cost[j] {
                cost[j] = cost[i] + 1;
                prev[j] = Some(current);
            }
        }
    }
    let mut count = 0;
    let mut current = end;
    while current != start {
        match prev[index(grid.len(), stride, current).unwrap()] {
            Some(x) => {
                current = x;
                count += 1;
            }
            None => return None,
        }
    }
    Some(count)
}

fn part1(input: &str) -> i32 {
    let stride = input.lines().next().unwrap().len();
    let raw_grid = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<Vec<char>>();
    let start = unindex(
        stride,
        raw_grid
            .iter()
            .copied()
            .enumerate()
            .find(|&(_, c)| c == 'S')
            .map(|(i, _)| i)
            .unwrap(),
    );
    let end = unindex(
        stride,
        raw_grid
            .iter()
            .copied()
            .enumerate()
            .find(|&(_, c)| c == 'E')
            .map(|(i, _)| i)
            .unwrap(),
    );
    let grid = raw_grid
        .iter()
        .copied()
        .map(|c| match c {
            'a'..='z' => c as u8 - b'a',
            'S' => b'a' - b'a',
            'E' => b'z' - b'a',
            _ => panic!(),
        })
        .collect::<Vec<u8>>();
    solve(stride, &grid, start, end).unwrap()
}

fn part2(input: &str) -> i32 {
    let stride = input.lines().next().unwrap().len();
    let raw_grid = input
        .lines()
        .flat_map(|line| line.chars())
        .collect::<Vec<char>>();
    let end = unindex(
        stride,
        raw_grid
            .iter()
            .copied()
            .enumerate()
            .find(|&(_, c)| c == 'E')
            .map(|(i, _)| i)
            .unwrap(),
    );
    let grid = raw_grid
        .iter()
        .copied()
        .map(|c| match c {
            'a'..='z' => c as u8 - b'a',
            'S' => b'a' - b'a',
            'E' => b'z' - b'a',
            _ => panic!(),
        })
        .collect::<Vec<u8>>();
    (0..grid.len())
        .filter(|&i| grid[i] == 0)
        .map(|i| unindex(stride, i))
        .filter_map(|start| solve(stride, &grid, start, end))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 29);
    }
}

use std::collections::BTreeSet;
use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve_p1(&input));
    println!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let width = input[0].len();
    let height = input.len();
    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            let h = input[y][x];
            let left_neighbor = (x != 0).then(|| input[y][x - 1]);
            let right_neighbor = (x + 1 < width).then(|| input[y][x + 1]);
            let lower_neighbor = (y != 0).then(|| input[y - 1][x]);
            let upper_neighbor = (y + 1 < height).then(|| input[y + 1][x]);
            let is_low = left_neighbor.map_or(true, |n| h < n)
                && right_neighbor.map_or(true, |n| h < n)
                && lower_neighbor.map_or(true, |n| h < n)
                && upper_neighbor.map_or(true, |n| h < n);
            if is_low {
                let risk = 1 + h;
                sum += risk;
            }
        }
    }
    sum
}

fn solve_p2(input: &str) -> usize {
    let input = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| x.to_string().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let width = input[0].len();
    let height = input.len();
    let mut low_points = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let h = input[y][x];
            let left_neighbor = (x != 0).then(|| input[y][x - 1]);
            let right_neighbor = (x + 1 < width).then(|| input[y][x + 1]);
            let lower_neighbor = (y != 0).then(|| input[y - 1][x]);
            let upper_neighbor = (y + 1 < height).then(|| input[y + 1][x]);
            let is_low = left_neighbor.map_or(true, |n| h < n)
                && right_neighbor.map_or(true, |n| h < n)
                && lower_neighbor.map_or(true, |n| h < n)
                && upper_neighbor.map_or(true, |n| h < n);
            if is_low {
                low_points.push((x, y));
            }
        }
    }
    let mut sums = Vec::new();
    for (x, y) in low_points {
        fn visit(
            input: &Vec<Vec<usize>>,
            width: usize,
            height: usize,
            visited: &mut BTreeSet<(usize, usize)>,
            (x, y): (usize, usize),
        ) -> usize {
            if !visited.insert((x, y)) {
                return 0;
            }
            if input[y][x] == 9 {
                return 0;
            }
            let mut sum = 1;
            sum += (x != 0)
                .then(|| visit(input, width, height, visited, (x - 1, y)))
                .unwrap_or(0);
            sum += (x + 1 != width)
                .then(|| visit(input, width, height, visited, (x + 1, y)))
                .unwrap_or(0);
            sum += (y != 0)
                .then(|| visit(input, width, height, visited, (x, y - 1)))
                .unwrap_or(0);
            sum += (y + 1 != height)
                .then(|| visit(input, width, height, visited, (x, y + 1)))
                .unwrap_or(0);
            sum
        }
        sums.push(visit(&input, width, height, &mut BTreeSet::new(), (x, y)));
    }
    sums.sort_unstable();
    sums.iter().rev().take(3).product::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(INPUT), 15);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_p2(INPUT), 1134);
    }
}

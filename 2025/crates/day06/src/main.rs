#![feature(test)]
use std::io::{self, Read};
use util::{Grid, point};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim_matches('\n');
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let mut rows: Vec<Vec<i64>> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    for line in &lines[..lines.len() - 1] {
        rows.push(
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect(),
        );
    }
    let mut cols: Vec<Vec<i64>> = Vec::new();
    for colidx in 0..rows[0].len() {
        let mut col = Vec::new();
        for rowidx in 0..rows.len() {
            col.push(rows[rowidx][colidx])
        }
        cols.push(col);
    }
    lines
        .last()
        .unwrap()
        .split_whitespace()
        .enumerate()
        .map(|(i, op)| {
            let (op, id): (fn(i64, i64) -> i64, i64) = match op {
                "+" => (|x, y| x + y, 0),
                "*" => (|x, y| x * y, 1),
                _ => panic!(),
            };
            cols[i].iter().fold(id, |x, &y| op(x, y))
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let (nums, ops) = input.rsplit_once("\n").unwrap();
    assert_eq!(nums.lines().next().unwrap().len(), ops.len());
    let grid = Grid::parse(nums, |c| c);
    let mut it = ops.char_indices().filter(|&(_, c)| c != ' ').peekable();
    let mut result = 0;
    while let Some((start, op)) = it.next() {
        let (op, id): (fn(i64, i64) -> i64, i64) = match op {
            '+' => (|x, y| x + y, 0),
            '*' => (|x, y| x * y, 1),
            c => panic!("{c:?}"),
        };
        let end = match it.peek() {
            Some(next_start) => next_start.0,
            None => ops.len(),
        };
        let width = end - start;
        let mut total = id;
        for x in start..(start + width) {
            let mut num = 0i64;
            let mut seen_digit = false;
            for y in 0..grid.height {
                let pt = point(x as i32, y as i32);
                let digit = grid.get(pt).copied().unwrap();
                if digit != ' ' {
                    seen_digit = true;
                    num *= 10;
                    num += digit.to_digit(10).unwrap() as i64;
                }
            }
            if seen_digit {
                total = op(total, num);
            }
        }
        result += total;
    }
    result
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 3263827);
    }

    #[bench]
    fn real_p1(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let input = input.trim_matches('\n');
        b.iter(|| assert_eq!(part1(test::black_box(&input)), 5977759036837));
    }

    #[bench]
    fn real_p2(b: &mut test::Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        let input = input.trim_matches('\n');
        b.iter(|| assert_eq!(part2(test::black_box(&input)), 9630000828442));
    }
}

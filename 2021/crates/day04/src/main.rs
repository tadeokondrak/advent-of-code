use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve_p1(&input));
    println!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> u32 {
    let mut iter = input.split("\n\n");
    let sequence = iter
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap());
    let mut boards = Vec::new();
    while let Some(board_str) = iter.next() {
        boards.push(Board::parse(board_str));
    }
    for num in sequence {
        for board in &mut boards {
            board.call_number(num);
            if let Some(score) = board.score(num) {
                return score;
            }
        }
    }
    panic!()
}

fn solve_p2(input: &str) -> u32 {
    let mut iter = input.split("\n\n");
    let sequence = iter
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap());
    let mut boards = Vec::new();
    while let Some(board_str) = iter.next() {
        boards.push(Some(Board::parse(board_str)));
    }
    let mut last_win = None;
    for num in sequence {
        for slot in &mut boards {
            if let Some(board) = slot {
                board.call_number(num);
                if let Some(score) = board.score(num) {
                    *slot = None;
                    last_win = Some(score);
                }
            }
        }
    }
    last_win.unwrap()
}

#[derive(Clone)]
struct Board {
    nums: Vec<(u32, bool)>,
}

impl Board {
    fn parse(s: &str) -> Self {
        Self {
            nums: s
                .split(&[' ', '\n'])
                .map(|x| x.trim_end())
                .filter(|x| !x.is_empty())
                .map(|x| (x.parse::<u32>().unwrap(), false))
                .collect(),
        }
    }

    fn call_number(&mut self, calling: u32) {
        for (num, is_marked) in &mut self.nums {
            if *num == calling {
                *is_marked = true;
            }
        }
    }

    fn is_marked(&self, row: usize, col: usize) -> bool {
        self.nums[row * 5 + col].1
    }

    fn check_rows(&self) -> bool {
        'outer: for row in 0..5 {
            for col in 0..5 {
                if !self.is_marked(row, col) {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }

    fn check_cols(&self) -> bool {
        'outer: for col in 0..5 {
            for row in 0..5 {
                if !self.is_marked(row, col) {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }

    fn score(&self, last_called: u32) -> Option<u32> {
        if !self.check_rows() && !self.check_cols() {
            return None;
        }
        let sum: u32 = self
            .nums
            .iter()
            .filter(|(_, is_marked)| !*is_marked)
            .map(|(x, _)| u32::from(*x))
            .sum();
        Some(sum * u32::from(last_called))
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..5 {
            for col in 0..5 {
                let (num, marked) = self.nums[row * 5 + col];
                if !marked {
                    write!(f, "{:2}", num)?;
                } else {
                    write!(f, "\x1b[31;1m{:2}\x1b[0m", num)?;
                }
                if col != 4 {
                    write!(f, " ")?;
                }
            }
            if row != 5 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(INPUT), 4512);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_p2(INPUT), 1924);
    }
}

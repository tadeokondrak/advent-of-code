use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut lines = vec![line
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<_>>()];
        loop {
            let next_line_index = lines.len();
            let mut next_line = vec![0; lines[next_line_index - 1].len() - 1];
            for (i, x) in next_line.iter_mut().enumerate() {
                *x = lines[next_line_index - 1][i + 1] - lines[next_line_index - 1][i];
            }
            lines.push(next_line);
            if lines[next_line_index].iter().all(|&x| x == 0) {
                break;
            }
        }
        for line in lines.iter_mut() {
            line.push(0);
        }
        for i in (0..lines.len() - 1).rev() {
            let line_len = lines[i].len();
            lines[i][line_len - 1] = lines[i + 1][line_len - 2] + lines[i][line_len - 2];
        }
        sum += *lines.first().unwrap().last().unwrap();
    }
    sum
}

fn solve_p2(input: &str) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut lines = VecDeque::from([line
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<VecDeque<_>>()]);
        loop {
            let next_line_index = lines.len();
            let mut next_line = VecDeque::from_iter(vec![0; lines[next_line_index - 1].len() - 1]);
            for (i, x) in next_line.iter_mut().enumerate() {
                *x = lines[next_line_index - 1][i + 1] - lines[next_line_index - 1][i];
            }
            lines.push_back(next_line);
            if lines[next_line_index].iter().all(|&x| x == 0) {
                break;
            }
        }
        for line in lines.iter_mut() {
            line.push_front(0);
        }
        for i in (0..lines.len() - 1).rev() {
            lines[i][0] = lines[i][1] - lines[i + 1][0];
        }
        sum += *lines.front().unwrap().front().unwrap();
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            solve_p1(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            114
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve_p2(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            2
        );
    }
}

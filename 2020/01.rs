use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let nums = stdin
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<i32>>();

    let mut p1 = None;
    'p1: for x in &nums {
        for y in &nums {
            if x + y == 2020 {
                p1 = Some(x * y);
                break 'p1;
            }
        }
    }
    println!("Part 1: {}", p1.unwrap());

    let mut p2 = None;
    'p2: for x in &nums {
        for y in &nums {
            for z in &nums {
                if x + y + z == 2020 {
                    p2 = Some(x * y * z);
                    break 'p2;
                }
            }
        }
    }
    println!("Part 2: {}", p2.unwrap());
}

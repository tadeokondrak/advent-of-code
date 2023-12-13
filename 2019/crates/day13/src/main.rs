use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::stdin;

use intcode::{Intcode, Step};

fn run(mut intcode: Intcode, quarters: i64) -> (i64, i64) {
    intcode.mem[0] = quarters;
    let mut grid = HashMap::new();
    let mut score = 0;
    loop {
        match intcode.step().unwrap() {
            Step::Continue => continue,
            Step::Output(x) => {
                let y = intcode.run_once(|| unreachable!()).unwrap().unwrap();
                let id = intcode.run_once(|| unreachable!()).unwrap().unwrap();
                if x == -1 {
                    score = id;
                } else {
                    grid.insert((x, y), id);
                }
            }
            Step::Input(i) => {
                let ((px, _), _) = grid.iter().find(|(_, &tile)| tile == 3).unwrap();
                let ((bx, _), _) = grid.iter().find(|(_, &tile)| tile == 4).unwrap();
                *i = match bx.cmp(px) {
                    Ordering::Less => -1,
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                };
            }
            Step::End => {
                break (
                    grid.iter().filter(|(_, &tile)| tile == 2).count() as i64,
                    score,
                )
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let intcode = Intcode::new(input.trim()).unwrap();
    println!(
        "Part 1: {}\nPart 2: {}",
        run(intcode.clone(), 1).0,
        run(intcode.clone(), 2).1,
    );
}

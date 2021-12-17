use std::collections::HashMap;
use std::io::{stdin, BufRead};

fn main() {
    let wire = stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split(',')
                .scan((0, 0, 0), |(ref mut x, ref mut y, ref mut step), seg| {
                    let (dir, steps) = seg.split_at(1);
                    let steps = steps.parse::<isize>().unwrap();
                    let entries = (*step..*step + steps)
                        .map(|i| {
                            match dir {
                                "L" => *x -= 1,
                                "R" => *x += 1,
                                "U" => *y += 1,
                                "D" => *y -= 1,
                                _ => unreachable!(),
                            };
                            ((*x, *y), i + 1)
                        })
                        .collect::<Vec<_>>();
                    *step += steps;
                    Some(entries)
                })
                .flatten()
                .collect()
        })
        .collect::<Vec<HashMap<(isize, isize), isize>>>();

    let iter = wire[0].iter().filter(|(pos, _)| wire[1].contains_key(pos));
    let p1 = iter.clone().map(|(pos, _)| pos.0.abs() + pos.1.abs()).min();
    let p2 = iter.map(|(pos, cost)| cost + wire[1][pos]).min();

    println!("Part 1: {}\nPart 2: {}", p1.unwrap(), p2.unwrap());
}

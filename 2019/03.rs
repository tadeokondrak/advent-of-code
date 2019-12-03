use std::{
    collections::HashMap,
    i32,
    io::{stdin, BufRead},
};

fn main() {
    let lines = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            line.split(',')
                .map(|cmd| cmd.split_at(1))
                .map(|(dir, steps)| (dir.chars().next().unwrap(), steps.parse().unwrap()))
                .collect()
        })
        .map(|line: Vec<(char, i32)>| {
            let mut grid = HashMap::new();
            let (mut x, mut y, mut step): (i32, i32, i32) = (0, 0, 0);
            grid.insert((x, y), step);
            for (dir, steps) in line {
                for _ in 0..steps {
                    step += 1;
                    match dir {
                        'L' => x -= 1,
                        'R' => x += 1,
                        'U' => y += 1,
                        'D' => y -= 1,
                        _ => unreachable!(),
                    };
                    grid.entry((x, y)).or_insert(step);
                }
            }
            grid
        })
        .collect::<Vec<_>>();

    let (p1, p2) = lines[0]
        .iter()
        .filter(|(&(x, y), _)| x != 0 && y != 0)
        .filter(|(pos, _)| lines[1].contains_key(pos))
        .fold((i32::MAX, i32::MAX), |(p1, p2), (&(x, y), &cost1)| {
            (p1.min(x.abs() + y.abs()), p2.min(cost1 + lines[1][&(x, y)]))
        });

    println!("Part 1: {}\nPart 2: {}", p1, p2);
}

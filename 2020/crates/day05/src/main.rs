use std::io::{stdin, BufRead};

fn seat(line: &str) -> usize {
    let (mut minrow, mut maxrow) = (0, 127);
    let (mut mincol, mut maxcol) = (0, 7);
    for c in line.chars() {
        match c {
            'F' => maxrow = minrow + (maxrow - minrow) / 2,
            'B' => minrow = maxrow - (maxrow - minrow) / 2,
            'L' => maxcol = mincol + (maxcol - mincol) / 2,
            'R' => mincol = maxcol - (maxcol - mincol) / 2,
            _ => unreachable!(),
        }
    }
    maxrow * 8 + maxcol
}

fn main() {
    let mut p1 = 0;
    let mut taken = [false; 1024];
    for line in stdin().lock().lines() {
        let id = seat(&line.unwrap());
        taken[id] = true;
        p1 = p1.max(id);
    }
    let p2 = taken
        .iter()
        .enumerate()
        .skip_while(|&(_, &taken)| !taken)
        .filter(|&(_, &taken)| !taken)
        .next()
        .unwrap()
        .0;
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

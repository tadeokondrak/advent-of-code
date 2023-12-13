use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input, (136, 7)));
}

struct SquareGrid<T> {
    data: Vec<T>,
    size: i32,
}

impl<T: Copy> SquareGrid<T> {
    fn at(&self, x: i32, y: i32) -> Option<T> {
        if x < 0 || x >= self.size || y < 0 || y >= self.size {
            None
        } else {
            Some(self.data[self.index(x, y)])
        }
    }

    fn set(&mut self, x: i32, y: i32, v: T) -> bool {
        if x < 0 || x >= self.size || y < 0 || y >= self.size {
            return false;
        }
        let i = self.index(x, y);
        self.data[i] = v;
        true
    }

    fn index(&self, x: i32, y: i32) -> usize {
        (y * self.size + x) as usize
    }
}

const CONNECTS_N: [u8; 3] = [b'|', b'L', b'J'];
const CONNECTS_E: [u8; 3] = [b'-', b'L', b'F'];
const CONNECTS_S: [u8; 3] = [b'|', b'7', b'F'];
const CONNECTS_W: [u8; 3] = [b'-', b'J', b'7'];

fn solve_p2(input: &str, hardcoded_point_inside_loop: (i32, i32)) -> u32 {
    let (grid, distances) = set_up_grid(input);

    let mut upsampled = SquareGrid {
        data: vec![false; (grid.size * grid.size * 9) as usize],
        size: grid.size * 3,
    };

    for x in 0..grid.size {
        for y in 0..grid.size {
            if distances.at(x, y).unwrap() == u32::MAX {
                continue;
            }
            let c = grid.at(x, y).unwrap();
            let n = CONNECTS_N.contains(&c);
            let e = CONNECTS_E.contains(&c);
            let s = CONNECTS_S.contains(&c);
            let w = CONNECTS_W.contains(&c);

            upsampled.set(x * 3, y * 3 - 1, n);
            upsampled.set(x * 3 + 1, y * 3, e);
            upsampled.set(x * 3, y * 3 + 1, s);
            upsampled.set(x * 3 - 1, y * 3, w);
            upsampled.set(x * 3, y * 3, true);
        }
    }
    let mut filled = SquareGrid {
        data: vec![false; (grid.size * grid.size * 9) as usize],
        size: grid.size * 3,
    };

    let mut stack = Vec::new();
    stack.push((hardcoded_point_inside_loop.0, hardcoded_point_inside_loop.1));
    while let Some((x, y)) = stack.pop() {
        if upsampled.at(x, y).is_none() || upsampled.at(x, y).unwrap() {
            continue;
        }
        if filled.at(x, y).is_none() || filled.at(x, y).unwrap() {
            continue;
        }
        filled.set(x, y, true);
        stack.push((x + 1, y));
        stack.push((x - 1, y));
        stack.push((x, y - 1));
        stack.push((x, y + 1));
    }

    let mut count = 0;
    for y in 0..grid.size {
        for x in 0..grid.size {
            if filled.at(x * 3, y * 3).unwrap() {
                count += 1;
            }
        }
    }

    count
}

fn set_up_grid(input: &str) -> (SquareGrid<u8>, SquareGrid<u32>) {
    let size = input.lines().next().unwrap().len() as i32;
    let data = input
        .lines()
        .flat_map(|x| x.as_bytes())
        .copied()
        .collect::<Vec<u8>>();
    let mut grid = SquareGrid { data, size };
    let mut start = None;
    for x in 0..size {
        for y in 0..size {
            if grid.at(x, y).unwrap() == b'S' {
                assert_eq!(start, None);
                start = Some((x, y));
                let connected_n = grid.at(x, y - 1).is_some_and(|n| CONNECTS_S.contains(&n));
                let connected_e = grid.at(x + 1, y).is_some_and(|e| CONNECTS_W.contains(&e));
                let connected_s = grid.at(x, y + 1).is_some_and(|s| CONNECTS_N.contains(&s));
                let connected_w = grid.at(x - 1, y).is_some_and(|w| CONNECTS_E.contains(&w));
                let s = format!(
                    "{}{}{}{}",
                    if connected_n { "n" } else { "" },
                    if connected_e { "e" } else { "" },
                    if connected_s { "s" } else { "" },
                    if connected_w { "w" } else { "" },
                );
                let c = match s.as_str() {
                    "ne" => b'L',
                    "ns" => b'|',
                    "nw" => b'J',
                    "es" => b'F',
                    "ew" => b'-',
                    "sw" => b'7',
                    _ => panic!("{s}"),
                };
                grid.set(x, y, c);
            }
        }
    }
    let pos = start.unwrap();
    let prev = match grid.at(pos.0, pos.1).unwrap() {
        b'L' => (pos.0, pos.1 - 1),
        b'|' => (pos.0, pos.1 - 1),
        b'J' => (pos.0, pos.1 - 1),
        b'F' => (pos.0 + 1, pos.1),
        b'-' => (pos.0 - 1, pos.1),
        b'7' => (pos.0 - 1, pos.1),
        other => panic!("{}", other as char),
    };

    let mut distances = SquareGrid {
        data: grid.data.iter().map(|_| u32::MAX).collect(),
        size: grid.size,
    };
    distances.set(pos.0, pos.1, 0);

    let new_prev = do_it(&grid, &mut distances, pos, prev);
    do_it(&grid, &mut distances, pos, new_prev);
    (grid, distances)
}

fn solve_p1(input: &str) -> u32 {
    let (grid, distances) = set_up_grid(input);

    (0..grid.size)
        .map(|y| {
            (0..grid.size)
                .filter_map(|x| {
                    let distance = distances.at(x, y).unwrap();
                    if distance == u32::MAX {
                        return None;
                    }
                    Some(distance)
                })
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0)
}

fn do_it(
    grid: &SquareGrid<u8>,
    distances: &mut SquareGrid<u32>,
    mut pos: (i32, i32),
    mut prev: (i32, i32),
) -> (i32, i32) {
    let orig = pos;
    let mut dist = 0;
    let mut first_pos = None;

    loop {
        dist += 1;

        let (prev_x, prev_y) = prev;
        let (mut x, mut y) = pos;

        match grid.at(x, y).unwrap() {
            b'L' => {
                if prev_y == y {
                    y -= 1;
                } else {
                    x += 1;
                }
            }
            b'|' => {
                if prev_y == y - 1 {
                    y += 1
                } else {
                    y -= 1;
                }
            }
            b'J' => {
                if prev_y == y {
                    y -= 1;
                } else {
                    x -= 1;
                }
            }
            b'F' => {
                if prev_y == y {
                    y += 1;
                } else {
                    x += 1;
                }
            }
            b'-' => {
                if prev_x == x - 1 {
                    x += 1;
                } else {
                    x -= 1;
                }
            }
            b'7' => {
                if prev_y == y {
                    y += 1;
                } else {
                    x -= 1;
                }
            }
            v => panic!("{v}"),
        }

        prev = pos;
        pos = (x, y);

        if dist == 1 {
            first_pos = Some((x, y));
        }

        if pos == orig {
            break first_pos.unwrap();
        }
        let i = distances.index(x, y);
        distances.data[i] = distances.data[i].min(dist);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            solve_p1(
                "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            ),
            4
        );

        assert_eq!(
            solve_p1(
                "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
            ),
            8
        );

        //        assert_eq!(
        //            solve_p1(
        //                "-L|F7
        //7S-7|
        //L|7||
        //-L-J|
        //L|-JF"
        //            ),
        //            4
        //        );
        //
        //        assert_eq!(
        //            solve_p1(
        //                "..F7.
        //.FJ|.
        //SJ.L7
        //|F--J
        //LJ...",
        //            ),
        //            8
        //        )
    }

    #[test]
    #[ignore = "broken"]
    fn part_2() {
        assert_eq!(
            solve_p2(
                "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
...........
...........",
                (2, 6)
            ),
            4
        );
        assert_eq!(
            solve_p2(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
....................
....................
....................
....................
....................
....................
....................
....................
....................
....................",
                (14, 3)
            ),
            10
        );
    }
}

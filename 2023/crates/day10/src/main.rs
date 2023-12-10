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

const CONNECTS_LEFT: [u8; 3] = [b'-', b'J', b'7'];
const CONNECTS_RIGHT: [u8; 3] = [b'-', b'L', b'F'];
const CONNECTS_UP: [u8; 3] = [b'|', b'L', b'J'];
const CONNECTS_DOWN: [u8; 3] = [b'|', b'7', b'F'];

fn solve_p2(input: &str, hardcoded_point_inside_loop: (i32, i32)) -> u32 {
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
                assert!(start.is_none());
                start = Some((x, y));
                let n = grid.at(x, y - 1);
                let e = grid.at(x + 1, y);
                let s = grid.at(x, y + 1);
                let w = grid.at(x - 1, y);
                let connected_n = n.is_some_and(|n| CONNECTS_DOWN.contains(&n));
                let connected_e = e.is_some_and(|e| CONNECTS_LEFT.contains(&e));
                let connected_s = s.is_some_and(|s| CONNECTS_UP.contains(&s));
                let connected_w = w.is_some_and(|w| CONNECTS_RIGHT.contains(&w));
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
                let i = grid.index(x, y);
                grid.data[i] = c;
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
    {
        let i = distances.index(pos.0, pos.1);
        distances.data[i] = 0;
    }
    let new_prev = fun_name(&grid, &mut distances, pos, prev);
    //eprintln!("\n\n\n\n\n");
    fun_name(&grid, &mut distances, pos, new_prev);

    let mut upsampled = SquareGrid {
        data: vec![false; (size * size * 9) as usize],
        size: size * 3,
    };

    for x in 0..size {
        for y in 0..size {
            if distances.at(x, y).unwrap() == u32::MAX {
                continue;
            }
            let (n, e, s, w) = match grid.at(x, y).unwrap() {
                b'L' => (true, true, false, false),
                b'|' => (true, false, true, false),
                b'J' => (true, false, false, true),
                b'F' => (false, true, true, false),
                b'-' => (false, true, false, true),
                b'7' => (false, false, true, true),
                _ => panic!(),
            };

            upsampled.set(x * 3, y * 3 - 1, n);
            upsampled.set(x * 3 + 1, y * 3, e);
            upsampled.set(x * 3, y * 3 + 1, s);
            upsampled.set(x * 3 - 1, y * 3, w);
            upsampled.set(x * 3, y * 3, true);
        }
    }

    for y in 0..upsampled.size {
        for x in 0..upsampled.size {
            let filled = upsampled.at(x, y).unwrap();
            if filled {
                eprint!("X")
            } else {
                eprint!(".")
            }
        }
        eprintln!();
    }

    let mut filled = SquareGrid {
        data: vec![false; (size * size * 9) as usize],
        size: size * 3,
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

    for y in 0..upsampled.size {
        for x in 0..upsampled.size {
            let filled = filled.at(x, y).unwrap();
            if filled {
                eprint!("X")
            } else {
                eprint!(".")
            }
        }
        eprintln!();
    }

    for y in 0..grid.size {
        for x in 0..grid.size {
            if distances.at(x, y).unwrap() != u32::MAX {
                eprint!("X");
            } else {
                eprint!(".");
            }
        }
        eprintln!();
    }

    let mut count = 0;
    for y in 0..grid.size {
        for x in 0..grid.size {
            if filled.at(x * 3, y * 3).unwrap() {
                count += 1;
                eprint!("X")
            } else {
                eprint!(".")
            }
        }
        eprintln!()
    }

    count
}

fn solve_p1(input: &str) -> u32 {
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
                assert!(start.is_none());
                start = Some((x, y));
                let n = grid.at(x, y - 1);
                let e = grid.at(x + 1, y);
                let s = grid.at(x, y + 1);
                let w = grid.at(x - 1, y);
                let connected_n = n.is_some_and(|n| CONNECTS_DOWN.contains(&n));
                let connected_e = e.is_some_and(|e| CONNECTS_LEFT.contains(&e));
                let connected_s = s.is_some_and(|s| CONNECTS_UP.contains(&s));
                let connected_w = w.is_some_and(|w| CONNECTS_RIGHT.contains(&w));
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
                let i = grid.index(x, y);
                grid.data[i] = c;
            }
        }
    }
    let pos = start.unwrap();
    let  prev = match grid.at(pos.0, pos.1).unwrap() {
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
    let new_prev = fun_name(&grid, &mut distances, pos, prev);
    //eprintln!("\n\n\n\n\n");
    fun_name(&grid, &mut distances, pos, new_prev);

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

fn fun_name(
    grid: &SquareGrid<u8>,
    distances: &mut SquareGrid<u32>,
    mut pos: (i32, i32),
    mut prev: (i32, i32),
) -> (i32, i32) {
    let orig = pos;
    let mut dist = 0;
    let mut first_pos = None;

    for y in 0..grid.size {
        for x in 0..grid.size {
            if x == pos.0 && y == pos.1 {
                //eprint!("!");
            } else {
                //eprint!("{}", grid.at(x, y).unwrap() as char);
            }
        }
        //eprintln!();
    }

    loop {
        dist += 1;

        //eprintln!("{prev:?} -> {pos:?}");
        let (prev_x, prev_y) = prev;
        let (mut x, mut y) = pos;

        match grid.at(x, y).unwrap() {
            b'L' => {
                if prev_y == y {
                    //dbg!();
                    y -= 1;
                } else {
                    //dbg!();
                    x += 1;
                }
            }
            b'|' => {
                if prev_y == y - 1 {
                    //dbg!();
                    y += 1
                } else {
                    //dbg!();
                    y -= 1;
                }
            }
            b'J' => {
                if prev_y == y {
                    //dbg!();
                    y -= 1;
                } else {
                    //dbg!();
                    x -= 1;
                }
            }
            b'F' => {
                if prev_y == y {
                    //dbg!();
                    y += 1;
                } else {
                    //dbg!();
                    x += 1;
                }
            }
            b'-' => {
                if prev_x == x - 1 {
                    //dbg!();
                    x += 1;
                } else {
                    //dbg!();
                    x -= 1;
                }
            }
            b'7' => {
                if prev_y == y {
                    //dbg!();
                    y += 1;
                } else {
                    //dbg!();
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

        //eprintln!("--");
        for y in 0..grid.size {
            for x in 0..grid.size {
                if x == pos.0 && y == pos.1 {
                    //eprint!("!");
                } else {
                    //eprint!("{}", grid.at(x, y).unwrap() as char);
                }
            }
            //eprintln!();
        }
        for y in 0..grid.size {
            for x in 0..grid.size {
                let d = distances.at(x, y).unwrap();
                if d == u32::MAX {
                    //eprint!(".");
                } else {
                    //eprint!("{}", d);
                }
            }
            //eprintln!();
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

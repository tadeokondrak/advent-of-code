#![feature(test)]
use std::{
    collections::HashMap,
    io::{self, Read},
};
use util::{offset, point, Grid, Offset, Point};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut pos = find_starting_point(grid);
    let mut grid = Grid::parse(grid, |c| if c == '@' { '.' } else { c });
    let moves = moves.replace("\n", "");
    for dir in moves.chars() {
        let dir = match dir {
            '^' => offset(0, -1),
            '>' => offset(1, 0),
            '<' => offset(-1, 0),
            'v' => offset(0, 1),
            _ => unreachable!(),
        };
        match grid[pos + dir] {
            '#' => {}
            '.' => {
                pos += dir;
            }
            'O' => {
                if try_push_box(&mut grid, pos + dir, dir) {
                    pos += dir;
                }
            }
            c => todo!("{c}"),
        }
    }

    fn try_push_box(grid: &mut Grid<char>, pos: Point, dir: Offset) -> bool {
        assert_eq!(grid[pos], 'O');
        match grid[pos + dir] {
            '#' => {
                return false;
            }
            '.' => {
                grid[pos] = '.';
                grid[pos + dir] = 'O';
                assert_eq!(grid[pos], '.');
                return true;
            }
            'O' => {
                let result = try_push_box(grid, pos + dir, dir);
                if result {
                    assert_eq!(grid[pos + dir], '.');
                    grid[pos] = '.';
                    grid[pos + dir] = 'O';
                }
                return result;
            }
            _ => todo!(),
        };
    }

    let mut count = 0;
    for y in 0..grid.height as i32 {
        for x in 0..grid.width as i32 {
            if grid[point(x, y)] == 'O' {
                count += x + 100 * y;
            }
        }
    }

    count
}

fn find_starting_point(input: &str) -> Point {
    let mut pos = None;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '@' {
                pos = Some(point(x as i32, y as i32));
            }
        }
    }
    pos.unwrap()
}

fn part2(input: &str) -> i32 {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let grid = {
        let grid = Grid::parse(grid, |c| c);
        let mut newgrid = Grid::new(grid.width * 2, grid.height);
        for y in 0..grid.height as i32 {
            for x in 0..grid.width as i32 {
                match grid[point(x, y)] {
                    c @ ('#' | '.') => {
                        newgrid[point(x * 2, y)] = c;
                        newgrid[point(x * 2 + 1, y)] = c;
                    }
                    'O' => {
                        newgrid[point(x * 2, y)] = '[';
                        newgrid[point(x * 2 + 1, y)] = ']';
                    }
                    '@' => {
                        newgrid[point(x * 2, y)] = '@';
                        newgrid[point(x * 2 + 1, y)] = '.';
                    }
                    _ => unreachable!(),
                }
            }
        }

        newgrid
    };
    part2_wide(&format!("{grid:?}\n\n{moves}"))
}

fn part2_wide(input: &str) -> i32 {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    let mut pos = find_starting_point(grid);
    let mut grid = Grid::parse(grid, |c| if c == '@' { '.' } else { c });
    let moves = moves.replace("\n", "");
    eprintln!("{grid:?}");
    let orig_grid = grid.clone();
    let mut i = 0;
    for c in moves.chars() {
        let dir = match c {
            '^' => offset(0, -1),
            '>' => offset(1, 0),
            '<' => offset(-1, 0),
            'v' => offset(0, 1),
            _ => unreachable!(),
        };
        let old_grid = grid.clone();
        match grid[pos + dir] {
            '#' => {}
            '.' => {
                pos += dir;
            }
            '[' | ']' => {
                if can_push_box(&old_grid, pos + dir, dir) {
                    assert!(try_push_box(&old_grid, &mut grid, pos + dir, dir));
                    pos += dir;
                }
            }
            c => todo!("{c}"),
        }
        //eprintln!("{c} {pos:?}\n{grid:?}");
        i += 1;
        if i % 50 == 0 {
            eprint!("\x1B[2J\x1B[1;1H");
            for y in 0..grid.height as i32 {
                for x in 0..grid.width as i32 {
                    let pt = point(x, y);
                    if pt == pos {
                        eprint!("@");
                    } else {
                        eprint!("{}", grid[pt]);
                    }
                }
                eprintln!();
            }
            std::thread::sleep_ms(1);
        }
    }
    std::thread::sleep_ms(1000);
    eprintln!("{grid:?}");
    assert_eq!(
        orig_grid.data.iter().filter(|&&c| c == '#').count(),
        grid.data.iter().filter(|&&c| c == '#').count()
    );

    assert_eq!(
        orig_grid.data.iter().filter(|&&c| c == '[').count(),
        grid.data.iter().filter(|&&c| c == '[').count()
    );
    assert_eq!(
        orig_grid.data.iter().filter(|&&c| c == ']').count(),
        grid.data.iter().filter(|&&c| c == ']').count()
    );

    fn can_push_box(grid: &Grid<char>, pos: Point, dir: Offset) -> bool {
        if dir.y == 0 {
            assert!((dir.x == 1 && grid[pos] == '[') || (dir.x == -1 && grid[pos] == ']'));
            match grid[pos + dir * 2] {
                '.' => true,
                '#' => false,
                c @ ('[' | ']') => {
                    assert!((dir.x == 1 && c == '[') || (dir.x == -1 && c == ']'));
                    can_push_box(grid, pos + dir * 2, dir)
                }
                _ => unreachable!(),
            }
        } else {
            let lb_pt = match grid[pos] {
                '[' => pos,
                ']' => pos + offset(-1, 0),
                c => unreachable!("{c}"),
            };
            let rb_pt = lb_pt + offset(1, 0);
            let lb_above_pt = lb_pt + dir;
            let lb_above = grid[lb_above_pt];
            let rb_above_pt = rb_pt + dir;
            let rb_above = grid[rb_above_pt];
            match (lb_above, rb_above) {
                ('.', '.') => true,
                ('#', _) | (_, '#') => false,
                ('[' | ']', _) | (_, '[' | ']') => {
                    (lb_above == '.' || can_push_box(grid, lb_pt + dir, dir))
                        && (rb_above == '.' || can_push_box(grid, rb_pt + dir, dir))
                }
                other => todo!("{other:?}"),
            }
        }
    }

    fn try_push_box(
        old_grid: &Grid<char>,
        new_grid: &mut Grid<char>,
        pos: Point,
        dir: Offset,
    ) -> bool {
        assert!(can_push_box(old_grid, pos, dir));
        if dir.y == 0 {
            if matches!(old_grid[pos + dir * 2], '[' | ']') {
                try_push_box(old_grid, new_grid, pos + dir * 2, dir);
            }
            new_grid[pos + dir * 2] = new_grid[pos + dir];
            new_grid[pos + dir] = new_grid[pos];
            new_grid[pos] = '.';
        } else {
            let lb_pt = match old_grid[pos] {
                '[' => pos,
                ']' => pos + offset(-1, 0),
                c => unreachable!("{c}"),
            };
            let rb_pt = lb_pt + offset(1, 0);
            let lb_above_pt = lb_pt + dir;
            let lb_above = old_grid[lb_above_pt];
            let rb_above_pt = rb_pt + dir;
            let rb_above = old_grid[rb_above_pt];
            if matches!((lb_above, rb_above), ('[' | ']', _) | (_, '[' | ']')) {
                if lb_above != '.' {
                    try_push_box(old_grid, new_grid, lb_pt + dir, dir);
                }
                if rb_above != '.' {
                    try_push_box(old_grid, new_grid, rb_pt + dir, dir);
                }
            }
            new_grid[lb_pt] = '.';
            new_grid[rb_pt] = '.';
            new_grid[lb_above_pt] = '[';
            new_grid[rb_above_pt] = ']';
        }
        true
    }

    let mut count = 0;
    for y in 0..grid.height as i32 {
        for x in 0..grid.width as i32 {
            if grid[point(x, y)] == '[' {
                count += x + 100 * y;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    const LARGE_TEST_INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part1() {
        assert_eq!(part1(LARGE_TEST_INPUT), 10092);
    }

    #[test]
    fn test_part2() {
        //        assert_eq!(
        //            part2(
        //                "#######
        //#...#.#
        //#.....#
        //#..OO@#
        //#..O..#
        //#.....#
        //#######
        //
        //<vv<<^^<<^^"
        //            ),
        //            618
        //        );

        //        part2_wide(
        //            "############
        //#..@.......#
        //#..[]......#
        //#...[].....#
        //#....[]....#
        //#.....[]...#
        //#....[]....#
        //#...[][]...#
        //#..[]..[]..#
        //#.[]....[].#
        //#..[]..#...#
        //#..........#
        //############
        //
        //vvv",
        //        );

        //assert_eq!(part2(LARGE_TEST_INPUT), 9021);
    }

    #[bench]
    #[ignore = "todo"]
    fn real_p1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part1(black_box(&input)), 0));
    }

    #[bench]
    #[ignore = "todo"]
    fn real_p2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input").unwrap();
        b.iter(|| assert_eq!(part2(black_box(&input)), 0));
    }
}

use std::{
    cmp,
    collections::{BinaryHeap, HashMap, HashSet},
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coord {
    x: u32,
    y: u32,
    dx: i32,
    dy: i32,
    steps: u32,
}

fn solve_p1(input: &str) -> u32 {
    let width = input.lines().next().unwrap().len() as u32;
    let height = input.lines().count() as u32;
    let grid: Vec<u8> = input.bytes().filter(|&c| c != b'\n').collect();
    let mut cost: HashMap<Coord, u32> = HashMap::new();
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut queue = BinaryHeap::new();
    let startx = Coord {
        x: 1,
        y: 0,
        dx: 1,
        dy: 0,
        steps: 1,
    };
    let starty = Coord {
        x: 0,
        y: 1,
        dx: 0,
        dy: 1,
        steps: 1,
    };
    let startxcost = u32::from(grid[1] - b'0');
    let startycost = u32::from(grid[width as usize] - b'0');
    cost.insert(startx, startxcost);
    cost.insert(starty, startycost);
    queue.push((cmp::Reverse(startxcost), startx));
    queue.push((cmp::Reverse(startycost), starty));
    while let Some((cmp::Reverse(cur_cost), cur)) = queue.pop() {
        //eprintln!("{cur:?}");
        if cur.x == width - 1 && cur.y == height - 1 {
            break;
        }
        visited.insert(cur);
        let mut add = |x, y| {
            let dx = x as i32 - cur.x as i32;
            let dy = y as i32 - cur.y as i32;
            let steps = if dx == cur.dx && dy == cur.dy {
                cur.steps + 1
            } else {
                1
            };
            if !(steps <= 3) {
                return;
            }
            let next = Coord {
                x,
                y,
                dx,
                dy,
                steps,
            };
            let last_cost = u32::from(grid[(next.y * height + next.x) as usize] - b'0');
            let new_cost = last_cost + cur_cost;
            let cost_entry = cost.entry(next).or_insert(u32::MAX);
            *cost_entry = (*cost_entry).min(new_cost);
            if visited.insert(next) {
                queue.push((cmp::Reverse(new_cost), next));
            }
        };
        if cur.x + 1 < width && cur.dx != -1 {
            add(cur.x + 1, cur.y);
        }
        if cur.y + 1 < height && cur.dy != -1 {
            add(cur.x, cur.y + 1);
        }
        if cur.x > 0 && cur.dx != 1 {
            add(cur.x - 1, cur.y);
        }
        if cur.y > 0 && cur.dy != 1 {
            add(cur.x, cur.y - 1);
        }
    }
    cost.into_iter()
        .filter_map(|(coord, cost)| (coord.x == width - 1 && coord.y == height - 1).then_some(cost))
        .min()
        .unwrap()
}

fn solve_p2(input: &str) -> u32 {
    let width = input.lines().next().unwrap().len() as u32;
    let height = input.lines().count() as u32;
    let grid: Vec<u8> = input.bytes().filter(|&c| c != b'\n').collect();
    let mut cost: HashMap<Coord, u32> = HashMap::new();
    let mut visited: HashSet<Coord> = HashSet::new();
    let mut queue = BinaryHeap::new();
    let startx = Coord {
        x: 1,
        y: 0,
        dx: 1,
        dy: 0,
        steps: 1,
    };
    let starty = Coord {
        x: 0,
        y: 1,
        dx: 0,
        dy: 1,
        steps: 1,
    };
    let startxcost = u32::from(grid[1] - b'0');
    let startycost = u32::from(grid[width as usize] - b'0');
    cost.insert(startx, startxcost);
    cost.insert(starty, startycost);
    queue.push((cmp::Reverse(startxcost), startx));
    queue.push((cmp::Reverse(startycost), starty));
    while let Some((cmp::Reverse(cur_cost), cur)) = queue.pop() {
        if cur.x == width - 1 && cur.y == height - 1 {
            break;
        }
        visited.insert(cur);
        let mut add = |x, y| {
            let dx = x as i32 - cur.x as i32;
            let dy = y as i32 - cur.y as i32;
            let steps = if dx == cur.dx && dy == cur.dy {
                cur.steps + 1
            } else {
                1
            };
            let turned = (cur.dx == 0) != (dx == 0);
            if turned && cur.steps < 4 {
                return;
            }
            if !(steps <= 10) {
                return;
            }
            let next = Coord {
                x,
                y,
                dx,
                dy,
                steps,
            };
            let last_cost = u32::from(grid[(next.y * height + next.x) as usize] - b'0');
            let new_cost = last_cost + cur_cost;
            let cost_entry = cost.entry(next).or_insert(u32::MAX);
            *cost_entry = (*cost_entry).min(new_cost);
            if visited.insert(next) {
                queue.push((cmp::Reverse(new_cost), next));
            }
        };
        if cur.x + 1 < width && cur.dx != -1 {
            add(cur.x + 1, cur.y);
        }
        if cur.y + 1 < height && cur.dy != -1 {
            add(cur.x, cur.y + 1);
        }
        if cur.x > 0 && cur.dx != 1 {
            add(cur.x - 1, cur.y);
        }
        if cur.y > 0 && cur.dy != 1 {
            add(cur.x, cur.y - 1);
        }
    }
    cost.into_iter()
        .filter_map(|(coord, cost)| (coord.x == width - 1 && coord.y == height - 1).then_some(cost))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(EXAMPLE_INPUT), 102);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_p2(EXAMPLE_INPUT), 94);
    }
}

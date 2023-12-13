use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve_p1(&input));
    println!("p2: {}", solve_p2(&input));
}

fn solve_p1(input: &str) -> usize {
    let map = VentMap::parse(input);
    map.part1()
}

fn solve_p2(input: &str) -> usize {
    let map = VentMap::parse2(input);
    map.part1()
}

struct VentMap {
    map: Vec<u32>,
}

const SIZE: u32 = 1000;
impl VentMap {
    fn parse(s: &str) -> Self {
        let mut this = Self {
            map: vec![0; (SIZE * SIZE) as usize],
        };
        for line in s.lines() {
            let (first_pair, second_pair) = line.split_once(" -> ").unwrap();
            let (first_x, first_y) = first_pair.split_once(",").unwrap();
            let (second_x, second_y) = second_pair.split_once(",").unwrap();
            let mut first_x = first_x.parse::<u32>().unwrap();
            let mut first_y = first_y.parse::<u32>().unwrap();
            let mut second_x = second_x.parse::<u32>().unwrap();
            let mut second_y = second_y.parse::<u32>().unwrap();
            if first_x > second_x {
                std::mem::swap(&mut first_x, &mut second_x);
            }
            if first_y > second_y {
                std::mem::swap(&mut first_y, &mut second_y);
            }
            if first_x == second_x {
                for y in first_y..=second_y {
                    *this.get_mut(first_x, y) += 1;
                }
            } else if first_y == second_y {
                for x in first_x..=second_x {
                    *this.get_mut(x, second_y) += 1;
                }
            } else {
            }
            //this.dump();
        }
        this
    }

    fn parse2(s: &str) -> Self {
        let mut this = Self {
            map: vec![0; (SIZE * SIZE) as usize],
        };
        for line in s.lines() {
            let (first_pair, second_pair) = line.split_once(" -> ").unwrap();
            let (first_x, first_y) = first_pair.split_once(",").unwrap();
            let (second_x, second_y) = second_pair.split_once(",").unwrap();
            let mut first_x = first_x.parse::<u32>().unwrap();
            let mut first_y = first_y.parse::<u32>().unwrap();
            let mut second_x = second_x.parse::<u32>().unwrap();
            let mut second_y = second_y.parse::<u32>().unwrap();
            if first_x == second_x {
                if first_y > second_y {
                    std::mem::swap(&mut first_y, &mut second_y);
                }
                for y in first_y..=second_y {
                    *this.get_mut(first_x, y) += 1;
                }
            } else if first_y == second_y {
                if first_x > second_x {
                    std::mem::swap(&mut first_x, &mut second_x);
                }
                for x in first_x..=second_x {
                    *this.get_mut(x, second_y) += 1;
                }
            } else {
                //this.dump();
                let x_dir: i32 = match second_x.cmp(&first_x) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => panic!(),
                    std::cmp::Ordering::Greater => 1,
                };
                let y_dir: i32 = match second_y.cmp(&first_y) {
                    std::cmp::Ordering::Less => -1,
                    std::cmp::Ordering::Equal => panic!(),
                    std::cmp::Ordering::Greater => 1,
                };
                let mut x = if x_dir == 1 {
                    first_x.min(second_x)
                } else {
                    first_x.max(second_x)
                };
                let mut y = if y_dir == 1 {
                    first_y.min(second_y)
                } else {
                    first_y.max(second_y)
                };
                let x_range = first_x.min(second_x)..=second_x.max(first_x);
                let y_range = first_y.min(second_y)..=second_y.max(first_y);
                loop {
                    let old_x = x;
                    let old_y = y;
                    let next_x = (x as i32 + x_dir) as u32;
                    let next_y = (y as i32 + y_dir) as u32;
                    *this.get_mut(x, y) += 1;
                    if x_range.contains(&next_x) {
                        x = next_x;
                    }
                    if y_range.contains(&next_y) {
                        y = next_y;
                    }
                    if old_x == x && old_y == y {
                        break;
                    }
                }
                //this.dump();
            }
        }
        this
    }

    fn get(&self, x: u32, y: u32) -> u32 {
        self.map[(x * SIZE + y) as usize]
    }

    fn get_mut(&mut self, x: u32, y: u32) -> &mut u32 {
        &mut self.map[(x * SIZE + y) as usize]
    }

    fn part1(&self) -> usize {
        self.map.iter().filter(|&&x| x > 1).count()
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for y in 0..SIZE {
            for x in 0..SIZE {
                let n = self.get(x, y);
                if n == 0 {
                    eprint!(".");
                } else {
                    eprint!("{}", n);
                }
            }
            eprintln!();
        }
        eprintln!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(INPUT), 5);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_p2(INPUT), 12);
    }
}

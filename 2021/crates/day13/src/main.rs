use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("p1: {}", solve_p1(&input));
    //println!("p2: {}", solve_p2(&input));
}

struct Sheet {
    height: i32,
    width: i32,
    stride: i32,
    data: Vec<u8>,
}

impl Sheet {
    fn is_valid(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }
    fn inc(&mut self, x: i32, y: i32) {
        self.add(x, y, 1);
    }
    fn get(&self, x: i32, y: i32) -> u8 {
        assert!(
            self.is_valid(x, y),
            "x={}, y={}, width={}, height={}, stride={}",
            x,
            y,
            self.width,
            self.height,
            self.stride
        );
        self.data[(y as usize * self.stride as usize + x as usize) as usize]
    }
    #[track_caller]
    fn add(&mut self, x: i32, y: i32, n: u8) {
        if x < 0 || y < 0 {
            return;
        }
        assert!(
            self.is_valid(x, y),
            "x={}, y={}, width={}, height={}, stride={}",
            x,
            y,
            self.width,
            self.height,
            self.stride
        );
        self.data[(y as usize * self.stride as usize + x as usize) as usize] += n;
    }
    fn fold(&mut self, dir: &str, line: i32) {
        if dir == "y" {
            let y_top = self.height / 2 - line;
            for y in 0..self.height - line {
                for x in 0..self.width {
                    let n = self.get(x, self.height - y - 1);
                    self.add(x, y + y_top, n);
                }
            }
            self.resize(self.width, self.height - line - 1);
        } else if dir == "x" {
            let x_left = self.width / 2 - line;
            for y in 0..self.height {
                for x in 0..self.width - line {
                    let n = self.get(self.width - x - 1, y);
                    self.add(x + x_left, y, n);
                }
            }
            self.resize(self.width - line - 1, self.height);
        } else {
            unreachable!()
        }
    }
    #[track_caller]
    fn resize(&mut self, width: i32, height: i32) {
        assert!(width <= self.width);
        assert!(height <= self.height);
        if width < self.width {
            self.width = width;
        }
        if height < self.height {
            self.height = height;
        }
    }
    fn visible_count(&self) -> i32 {
        let mut sum = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) != 0 {
                    sum += 1;
                }
            }
        }
        sum
    }
}

impl std::fmt::Debug for Sheet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        /*
        if f.alternate() {
            let mut x_iter = (0..self.width).step_by(2);
            let mut y_iter = (0..self.height).step_by(8);
            while let Some(y_base) = y_iter.next() {
                let x_base = x_iter.next().unwrap();
                let mut n = 0u8;
                for y in y_base..y_base + 4 {
                    for x in x_base..x_base + 2 {
                        if self.is_valid(x, y) {
                            n |= (self.get(x, y) != 0) as u8;
                        }
                        n <<= 1;
                    }
                }
                write!(f, "{}", to_braille(n))?;
            }
            writeln!(f)?;
        } else {
        */
        {
            for y in 0..self.height {
                for x in 0..self.width {
                    match self.get(x, y) {
                        0 => write!(f, ".")?,
                        n => write!(f, "{}", n)?,
                    }
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[allow(dead_code)]
fn to_braille(c: u8) -> char {
    // https://github.com/iirelu/braillify
    let warped_dotmap: u8 = (c >> 7 & 0b00000001u8)
        | (c >> 3 & 0b00001000u8)
        | (c >> 4 & 0b00000010u8)
        | (c >> 0 & 0b00010000u8)
        | (c >> 1 & 0b00000100u8)
        | (c << 3 & 0b00100000u8)
        | (c << 5 & 0b01000000u8)
        | (c << 7 & 0b10000000u8);
    char::from_u32(10240 + warped_dotmap as u32).unwrap()
}

fn solve_p1(input: &str) -> i32 {
    let (points, folds) = input.split_once("\n\n").unwrap();
    let points = points
        .lines()
        .map(|x| x.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect::<Vec<(i32, i32)>>();
    let folds = folds
        .lines()
        .map(|x| x.trim_start_matches("fold along "))
        .map(|x| x.split_once("=").unwrap())
        .map(|(dir, num)| (dir.to_owned(), num.parse().unwrap()))
        .collect::<Vec<(String, i32)>>();
    let width = points
        .iter()
        .copied()
        .map(|(x, _)| x as usize)
        .max()
        .unwrap()
        + 1;
    let height = points
        .iter()
        .copied()
        .map(|(_, y)| y as usize)
        .max()
        .unwrap()
        + 1;
    let mut sheet = Sheet {
        width: width as i32,
        height: height as i32,
        stride: width as i32,
        data: vec![0u8; (width * height) as usize],
    };
    for (x, y) in points {
        sheet.inc(x, y);
    }
    //eprintln!("{:#?}", sheet);
    for (dir, num) in folds.iter().take(1) {
        sheet.fold(dir, *num);
        eprintln!("{:#?}", sheet);
    }
    sheet.visible_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn part_1() {
        assert_eq!(solve_p1(INPUT), 17);
    }
}

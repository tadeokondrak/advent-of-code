use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    eprintln!("p1: {}", solve_p1(&input));
    eprintln!("p2: {}", solve_p2(&input));
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Vec2<T> {
    x: T,
    y: T,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Light {
    pos: Vec2<i32>,
    vel: Vec2<i32>,
}

fn solve_p1(input: &str) -> u32 {
    let grid = Grid::new(&input);
    do_the_thing(&grid, Vec2 { x: 0, y: 0 }, Vec2 { x: 1, y: 0 })
}
fn solve_p2(input: &str) -> u32 {
    let grid = Grid::new(&input);

    [
        (0..grid.height as i32)
            .map(|y| {
                let start = Vec2 { x: 0, y };
                let start_vel = Vec2 { x: 1, y: 0 };
                do_the_thing(&grid, start, start_vel)
            })
            .max()
            .unwrap(),
        (0..grid.height as i32)
            .map(|y| {
                let start = Vec2 {
                    x: grid.width as i32 - 1,
                    y: y as i32,
                };
                let start_vel = Vec2 { x: -1, y };
                do_the_thing(&grid, start, start_vel)
            })
            .max()
            .unwrap(),
        (0..grid.width as i32)
            .map(|x| {
                let start = Vec2 { x: x as i32, y: 0 };
                let start_vel = Vec2 { x: 0, y: 1 };
                do_the_thing(&grid, start, start_vel)
            })
            .max()
            .unwrap(),
        (0..grid.width as i32)
            .map(|x| {
                let start = Vec2 {
                    x: x as i32,
                    y: grid.height as i32 - 1,
                };
                let start_vel = Vec2 { x: 0, y: -1 };
                do_the_thing(&grid, start, start_vel)
            })
            .max()
            .unwrap(),
    ]
    .iter()
    .copied()
    .max()
    .unwrap()
}

fn do_the_thing(grid: &Grid<u8>, start: Vec2<i32>, start_vel: Vec2<i32>) -> u32 {
    eprintln!("{start:?}");
    let mut lights: Vec<Light> = vec![Light {
        pos: start,
        vel: start_vel,
    }];

    let mut filled_grid = Grid {
        data: grid.data.iter().map(|_| b'.').collect(),
        width: grid.width,
        height: grid.height,
    };
    let mut i = 0;
    let mut last_count = u32::MAX;
    loop {
        let count = filled_grid.data.iter().filter(|&&b| b == b'#').count() as usize as u32;
        if count == last_count {
            i += 1;
        } else {
            i = 0;
        }
        if i > 10 {
            break;
        }
        last_count = count;

        i += 1;
        //eprintln!("{lights:?}");
        for light in &lights {
            filled_grid.set(light.pos.x, light.pos.y, b'#');
        }
        //eprintln!("{filled_grid:?}");
        let mut new_lights = Vec::new();
        for light in lights.iter_mut() {
            match grid.get(light.pos.x, light.pos.y) {
                b'.' => {}
                b'/' => {
                    //eprintln!("b'/'");
                    *light = Light {
                        pos: Vec2 {
                            x: light.pos.x,
                            y: light.pos.y,
                        },
                        vel: Vec2 {
                            x: -light.vel.y,
                            y: -light.vel.x,
                        },
                    };
                }
                b'\\' => {
                    //eprintln!("b'\\'");
                    *light = Light {
                        pos: Vec2 {
                            x: light.pos.x,
                            y: light.pos.y,
                        },
                        vel: Vec2 {
                            x: light.vel.y,
                            y: light.vel.x,
                        },
                    };
                }
                b'|' => {
                    //eprintln!("b'|'");
                    if light.vel.y != 0 {
                        *light = Light {
                            pos: Vec2 {
                                x: light.pos.x,
                                y: light.pos.y,
                            },
                            vel: light.vel,
                        };
                    } else {
                        assert!(light.vel.x != 0);
                        *light = Light {
                            pos: Vec2 {
                                x: light.pos.x,
                                y: light.pos.y,
                            },
                            vel: Vec2 {
                                x: light.vel.y,
                                y: light.vel.x,
                            },
                        };

                        new_lights.push(Light {
                            pos: Vec2 {
                                x: light.pos.x,
                                y: light.pos.y,
                            },
                            vel: Vec2 {
                                x: -light.vel.y,
                                y: -light.vel.x,
                            },
                        });
                    }
                }
                b'-' => {
                    //eprintln!("b'-'");
                    if light.vel.x != 0 {
                        *light = Light {
                            pos: Vec2 {
                                x: light.pos.x,
                                y: light.pos.y,
                            },
                            vel: light.vel,
                        };
                    } else {
                        assert!(light.vel.y != 0);
                        *light = Light {
                            pos: Vec2 {
                                x: light.pos.x,
                                y: light.pos.y,
                            },
                            vel: Vec2 {
                                x: light.vel.y,
                                y: light.vel.x,
                            },
                        };

                        new_lights.push(Light {
                            pos: Vec2 {
                                x: light.pos.x,
                                y: light.pos.y,
                            },
                            vel: Vec2 {
                                x: -light.vel.y,
                                y: -light.vel.x,
                            },
                        });
                    }
                }
                other => panic!("{other}"),
            }
            *light = Light {
                pos: Vec2 {
                    x: light.pos.x + light.vel.x,
                    y: light.pos.y + light.vel.y,
                },
                vel: light.vel,
            };
        }

        lights.extend(new_lights);
        lights.sort();
        lights.dedup();
    }
    //eprintln!("{filled_grid:?}");
    filled_grid.data.iter().filter(|&&b| b == b'#').count() as usize as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        assert_eq!(
            solve_p1(
                r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#
            ),
            46
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            solve_p2(
                r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#
            ),
            51
        );
    }
}
#[derive(Clone)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl Grid<u8> {
    fn get(&self, x: i32, y: i32) -> u8 {
        if !self.is_valid_point(x, y) {
            return b'.';
        }
        self.data[(y * self.width as i32 + x) as usize]
    }

    fn set(&mut self, x: i32, y: i32, v: u8) {
        if !self.is_valid_point(x, y) {
            return;
        }
        self.data[(y * self.width as i32 + x) as usize] = v;
    }

    fn is_valid_point(&self, x: i32, y: i32) -> bool {
        if x < 0 {
            return false;
        }
        if y < 0 {
            return false;
        }
        if x >= self.width as i32 {
            return false;
        }
        if y >= self.height as i32 {
            return false;
        }
        true
    }
}

impl Grid<u8> {
    fn new(s: &str) -> Self {
        let mut data = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in s.lines() {
            width = line.len();
            height += 1;
            for &c in line.as_bytes() {
                assert_ne!(c, b'\n');
                data.push(c);
            }
        }
        assert_eq!(data.len(), width * height);
        Self {
            data,
            width,
            height,
        }
    }
}

impl std::fmt::Debug for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height as i32 {
            write!(f, "{y:2} ")?;
            for x in 0..self.width as i32 {
                write!(f, "{}", self.get(x, y) as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

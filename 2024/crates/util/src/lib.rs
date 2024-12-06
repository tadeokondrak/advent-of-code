use std::{
    fmt,
    ops::{Index, IndexMut},
};

pub fn point(x: i32, y: i32) -> Point {
    Point { x, y }
}

pub const fn offset(x: i32, y: i32) -> Offset {
    Offset { x, y }
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Point")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl fmt::Debug for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Offset")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl Point {
    pub const ZERO: Point = Point { x: 0, y: 0 };

    pub fn distance_from(self, other: Point) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl std::ops::Add<Offset> for Point {
    type Output = Self;

    fn add(self, rhs: Offset) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Offset> for Point {
    type Output = Self;

    fn sub(self, rhs: Offset) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::AddAssign<Offset> for Point {
    fn add_assign(&mut self, rhs: Offset) {
        *self = *self + rhs;
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Default + Copy> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T> {
        let data = vec![T::default(); width * height];
        Grid {
            data,
            width,
            height,
        }
    }
}
impl<T> Grid<T> {
    pub fn parse(s: &str, f: impl Fn(char) -> T) -> Grid<T> {
        let mut data = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in s.lines() {
            width = line.len();
            height += 1;
            for c in line.chars() {
                assert_ne!(c, '\n');
                data.push(f(c));
            }
        }
        assert_eq!(data.len(), width * height);
        Grid {
            data,
            width,
            height,
        }
    }
}

impl<T> Grid<T> {
    pub fn is_in_bounds(&self, pos: impl GridIndex) -> bool {
        let (x, y) = pos.coords();
        0 <= x && x < self.width as i32 && 0 <= y && y < self.width as i32
    }

    pub fn get(&self, pos: impl GridIndex) -> Option<&T> {
        let (x, y) = pos.coords();
        if !self.is_in_bounds((x, y)) {
            return None;
        }
        Some(&self.data[(y * self.width as i32 + x) as usize])
    }

    pub fn get_mut(&mut self, pos: impl GridIndex) -> Option<&mut T> {
        let (x, y) = pos.coords();
        if !self.is_in_bounds((x, y)) {
            return None;
        }
        Some(&mut self.data[(y * self.width as i32 + x) as usize])
    }

    pub fn set(&mut self, pos: impl GridIndex, v: T) {
        let (x, y) = pos.coords();
        *self.get_mut((x, y)).unwrap() = v;
    }
}

pub trait GridIndex: Copy {
    fn coords(self) -> (i32, i32);
}

impl GridIndex for (i32, i32) {
    fn coords(self) -> (i32, i32) {
        (self.0, self.1)
    }
}

impl GridIndex for Point {
    fn coords(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl<T, I: GridIndex> Index<I> for Grid<T> {
    type Output = T;

    fn index(&self, i: I) -> &T {
        self.get(i).unwrap()
    }
}

impl<T, I: GridIndex> IndexMut<I> for Grid<T> {
    fn index_mut(&mut self, i: I) -> &mut T {
        self.get_mut(i).unwrap()
    }
}

//impl std::fmt::Debug for Grid<u8> {
//    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//        for y in 0..self.height {
//            write!(f, "{y:2} ")?;
//            for x in 0..self.width {
//                write!(f, "{}", self.get(x, y))?;
//            }
//            writeln!(f)?;
//        }
//        Ok(())
//    }
//}

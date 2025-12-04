use std::{
    fmt::{self, Debug, Display},
    ops::{Index, IndexMut},
};

pub fn find_point(input: &str, mark: char) -> Point {
    let mut pos = None;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == mark {
                pos = Some(point(x as i32, y as i32));
            }
        }
    }
    pos.unwrap()
}

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

impl Offset {
    pub const NORTH: Offset = offset(0, -1);
    pub const EAST: Offset = offset(1, 0);
    pub const SOUTH: Offset = offset(0, 1);
    pub const WEST: Offset = offset(-1, 0);
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

    pub fn neighbors(self) -> [Point; 8] {
        [
            point(self.x - 1, self.y - 1),
            point(self.x, self.y - 1),
            point(self.x + 1, self.y - 1),
            point(self.x - 1, self.y),
            point(self.x + 1, self.y),
            point(self.x - 1, self.y + 1),
            point(self.x, self.y + 1),
            point(self.x + 1, self.y + 1),
        ]
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

impl std::ops::Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<i32> for Offset {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Offset {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<i32> for Point {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::AddAssign<Offset> for Point {
    fn add_assign(&mut self, rhs: Offset) {
        *self = *self + rhs;
    }
}

impl From<Point> for [i32; 2] {
    fn from(val: Point) -> [i32; 2] {
        [val.x, val.y]
    }
}

impl From<Offset> for [i32; 2] {
    fn from(val: Offset) -> [i32; 2] {
        [val.x, val.y]
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
        0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32
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

    pub fn points(&self) -> impl Iterator<Item = Point> + 'static {
        let height = self.height as i32;
        let width = self.width as i32;
        (0..height)
            .flat_map(move |y| (0..width).map(move |x| (x, y)))
            .map(|(x, y)| point(x, y))
    }
}

impl<T: Display> Debug for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height as i32 {
            for x in 0..self.width as i32 {
                let pt = point(x, y);
                write!(f, "{}", self[pt])?;
            }
            writeln!(f)?;
        }
        Ok(())
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

//

pub fn point64(x: i64, y: i64) -> Point64 {
    Point64 { x, y }
}

pub const fn offset64(x: i64, y: i64) -> Offset64 {
    Offset64 { x, y }
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point64 {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Offset64 {
    pub x: i64,
    pub y: i64,
}

impl fmt::Debug for Point64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Point64")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl fmt::Debug for Offset64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Offset64")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl Point64 {
    pub const ZERO: Point64 = Point64 { x: 0, y: 0 };

    pub fn distance_from(self, other: Point64) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl std::ops::Add<Offset64> for Point64 {
    type Output = Self;

    fn add(self, rhs: Offset64) -> Self::Output {
        Point64 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Offset64> for Point64 {
    type Output = Self;

    fn sub(self, rhs: Offset64) -> Self::Output {
        Point64 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add<Point64> for Point64 {
    type Output = Self;

    fn add(self, rhs: Point64) -> Self::Output {
        Point64 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Point64> for Point64 {
    type Output = Self;

    fn sub(self, rhs: Point64) -> Self::Output {
        Point64 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Mul<i64> for Point64 {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Point64 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<i64> for Offset64 {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Offset64 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Div<i64> for Point64 {
    type Output = Self;

    fn div(self, rhs: i64) -> Self::Output {
        Point64 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl std::ops::AddAssign<Offset64> for Point64 {
    fn add_assign(&mut self, rhs: Offset64) {
        *self = *self + rhs;
    }
}

impl From<Point64> for [i64; 2] {
    fn from(val: Point64) -> [i64; 2] {
        [val.x, val.y]
    }
}

impl From<Offset64> for [i64; 2] {
    fn from(val: Offset64) -> [i64; 2] {
        [val.x, val.y]
    }
}

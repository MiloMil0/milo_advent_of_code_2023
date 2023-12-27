#![allow(unused)]
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}
// pub const ORTHO_DIRECTIONS: [Point; 4] = [Point::UP, Point::DOWN, Point::LEFT, Point::RIGHT];

impl Point {
    // pub const UP: Point = Point { x: 0, y: -1 };
    // pub const DOWN: Point = Point { x: 0, y: 1 };
    // pub const LEFT: Point = Point { x: -1, y: 0 };
    // pub const RIGHT: Point = Point { x: 1, y: 0 };
    // pub const UP_LEFT: Point = Point { x: -1, y: 1 };
    // pub const UP_RIGHT: Point = Point { x: 1, y: 1 };
    // pub const DOWN_LEFT: Point = Point { x: -1, y: -1 };
    // pub const DOWN_RIGHT: Point = Point { x: 1, y: -1 };

    #[inline]
    pub fn new<T>(x: T, y: T) -> Self
    where
        T: TryInto<isize>,
    {
        Point {
            x: x.try_into().ok().unwrap(),
            y: y.try_into().ok().unwrap(),
        }
    }

    // #[inline]
    pub fn manhattan(&self, other: Point) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline]
    // pub fn from_tuple<T>(t: (T, T)) -> Self
    // where
    //     T: TryInto<i32>,
    // {
    //     // Point::new(t.0, t.1)
    // }
    #[inline]
    pub fn to_index<T>(&self, width: T) -> usize
    where
        T: TryInto<usize>,
    {
        let x: usize = self.x.try_into().ok().unwrap();
        let y: usize = self.y.try_into().ok().unwrap();
        let w: usize = width.try_into().ok().unwrap();
        (y * w) + x
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// impl Div<i32> for Point {
//     type Output = Self;
//
//     fn div(self, other: i32) -> Self {
//         Point::new(self.x / other, self.y / other)
//     }
// }
//
// impl Mul<i32> for Point {
//     type Output = Self;
//
//     fn mul(self, other: i32) -> Self {
//         Point::new(self.x * other, self.y * other)
//     }
// }
//
// impl Mul<Point> for i32 {
//     type Output = Point;
//
//     fn mul(self, other: Point) -> Point {
//         Point::new(other.x * self, other.y * self)
//     }
// }

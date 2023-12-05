use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub const UP: Point = Point { x: 0, y: 1 };
    pub const DOWN: Point = Point { x: 0, y: -1 };
    pub const LEFT: Point = Point { x: -1, y: 0 };
    pub const RIGHT: Point = Point { x: 1, y: 0 };
    pub const UP_LEFT: Point = Point { x: -1, y: 1 };
    pub const UP_RIGHT: Point = Point { x: 1, y: 1 };
    pub const DOWN_LEFT: Point = Point { x: -1, y: -1 };
    pub const DOWN_RIGHT: Point = Point { x: 1, y: -1 };

    #[inline]
    pub fn new<T>(x: T, y: T) -> Self
    where
        T: TryInto<i32>,
    {
        Point {
            x: x.try_into().ok().unwrap(),
            y: y.try_into().ok().unwrap(),
        }
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

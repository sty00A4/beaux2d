use std::ops::*;
use sdl2::rect::Point;

pub struct Vec2<T> {
    pub x: T,
    pub y: T
}
impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T: Clone> Clone for Vec2<T> {
    fn clone(&self) -> Self {
        Self { x: self.x.clone(), y: self.y.clone() }
    }
}
impl<T: Copy> Copy for Vec2<T> {}
impl<T> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Self { x: value.0, y: value.1 }
    }
}
impl From<Point> for Vec2<i32> {
    fn from(value: Point) -> Self {
        Self { x: value.x, y: value.y }
    }
}
impl Into<Point> for Vec2<i32> {
    fn into(self) -> Point {
        Point::new(self.x, self.y)
    }
}

impl<T: Add<Output = T>> Add for Vec2<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
impl<T: Sub<Output = T>> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}
impl<T: Mul<Output = T>> Mul for Vec2<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}
impl<T: Div<Output = T>> Div for Vec2<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}
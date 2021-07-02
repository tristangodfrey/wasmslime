use std::ops::Add;
use std::{f64::consts::PI, ops::Mul};
use std::convert::TryFrom;



#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl<T, U> Mul<U> for Point<T> where U: Into<T>, T: Mul<Output = T> + Copy {
    type Output = Self;
    fn mul(self, rhs: U) -> Self::Output {
        let val: T = rhs.into();
        Self {
            x: self.x * val,
            y: self.y * val
        }
    }
}

impl<T> Add<Point<T>> for Point<T> where T: Add<T, Output = T> {
    type Output = Self;
    fn add(self, rhs: Point<T>) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Point<f64> {
    pub fn from_degrees(degrees: f64) -> Self {
        let radians = degrees.to_radians();
        Self { x: radians.cos(), y: radians.sin() }
    }
}

impl Point<usize> {
    pub fn get_index(&self, width: usize, height: usize) -> usize {
        // TODO: something is wrong here, fix it
        ((width * (self.y)) + self.x) % (width * height)
    }
}

impl From<Point<f64>> for Point<usize> {
    fn from(point: Point<f64>) -> Self {
        Self {
            x: point.x.round() as usize,
            y: point.y.round() as usize
        }
    }
}

impl From<Point<f64>> for Point<i64> {
    fn from(point: Point<f64>) -> Self {
        Self {
            x: point.x.round() as i64,
            y: point.y.round() as i64
        }
    }
}

#[cfg(test)]
pub mod test {

    use super::*;

    #[test]
    fn into_discrete()
    {
        let point = Point { x: 3.45f64, y: 5.9f64 };

        let discrete: Point<i64> = point.into();

        assert_eq!(discrete.x, 3);
        assert_eq!(discrete.y, 6);
    }

    #[test]
    fn from_degrees()
    {

        let a: Point<i64> = Point::from_degrees(90f64).into();
        let b: Point<i64> = Point::from_degrees(180f64).into();
        
        assert_eq!(a, Point { x: 0, y: 1 });
        assert_eq!(b, Point { x: -1, y: 0});
        
    }

    #[test]
    fn add_point()
    {
        let original = Point::new(5f64, 5f64);
        let delta  = Point::from_degrees(180f64) * 2;
        let new = original + delta;

        assert_eq!(new, Point { x: 3.0f64, y: 5.0f64 });
    }

}
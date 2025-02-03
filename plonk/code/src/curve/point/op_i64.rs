use super::Point;

use ark_ff::fields::Field;
use halo_accumulation::group::PallasScalar;

use std::ops::{Div, Mul};

// Mul -----------------------------------------------------

impl Mul<i64> for Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs))
    }
}

impl Mul<&i64> for Point {
    type Output = Point;

    fn mul(self, rhs: &i64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs))
    }
}

impl Mul<i64> for &Point {
    type Output = Point;

    fn mul(self, rhs: i64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs))
    }
}

impl Mul<&i64> for &Point {
    type Output = Point;

    fn mul(self, rhs: &i64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs))
    }
}

impl Mul<Point> for i64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(self))
    }
}

impl Mul<&Point> for i64 {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(self))
    }
}

impl Mul<Point> for &i64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(*self))
    }
}

impl Mul<&Point> for &i64 {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(*self))
    }
}

// Div -----------------------------------------------------

impl Div<i64> for Point {
    type Output = Point;

    fn div(self, rhs: i64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs).inverse().unwrap())
    }
}

impl Div<&i64> for Point {
    type Output = Point;

    fn div(self, rhs: &i64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs).inverse().unwrap())
    }
}

impl Div<i64> for &Point {
    type Output = Point;

    fn div(self, rhs: i64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs).inverse().unwrap())
    }
}

impl Div<&i64> for &Point {
    type Output = Point;

    fn div(self, rhs: &i64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs).inverse().unwrap())
    }
}

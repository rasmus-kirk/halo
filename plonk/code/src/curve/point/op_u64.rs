use super::Point;

use ark_ff::fields::Field;
use halo_accumulation::group::PallasScalar;

use std::ops::{Div, Mul};

// Mul -----------------------------------------------------

impl Mul<u64> for Point {
    type Output = Point;

    fn mul(self, rhs: u64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs))
    }
}

impl Mul<&u64> for Point {
    type Output = Point;

    fn mul(self, rhs: &u64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs))
    }
}

impl Mul<u64> for &Point {
    type Output = Point;

    fn mul(self, rhs: u64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs))
    }
}

impl Mul<&u64> for &Point {
    type Output = Point;

    fn mul(self, rhs: &u64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs))
    }
}

impl Mul<Point> for u64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(self))
    }
}

impl Mul<&Point> for u64 {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(self))
    }
}

impl Mul<Point> for &u64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(*self))
    }
}

impl Mul<&Point> for &u64 {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(*self))
    }
}

// Div -----------------------------------------------------

impl Div<u64> for Point {
    type Output = Point;

    fn div(self, rhs: u64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs).inverse().unwrap())
    }
}

impl Div<&u64> for Point {
    type Output = Point;

    fn div(self, rhs: &u64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs).inverse().unwrap())
    }
}

impl Div<u64> for &Point {
    type Output = Point;

    fn div(self, rhs: u64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs).inverse().unwrap())
    }
}

impl Div<&u64> for &Point {
    type Output = Point;

    fn div(self, rhs: &u64) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs).inverse().unwrap())
    }
}

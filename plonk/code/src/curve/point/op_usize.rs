use super::Point;

use ark_ff::fields::Field;
use halo_accumulation::group::PallasScalar;

use std::ops::{Div, Mul};

// Mul -----------------------------------------------------

impl Mul<usize> for Point {
    type Output = Point;

    fn mul(self, rhs: usize) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs as u64))
    }
}

impl Mul<&usize> for Point {
    type Output = Point;

    fn mul(self, rhs: &usize) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs as u64))
    }
}

impl Mul<usize> for &Point {
    type Output = Point;

    fn mul(self, rhs: usize) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs as u64))
    }
}

impl Mul<&usize> for &Point {
    type Output = Point;

    fn mul(self, rhs: &usize) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs as u64))
    }
}

impl Mul<Point> for usize {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(self as u64))
    }
}

impl Mul<&Point> for usize {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(self as u64))
    }
}

impl Mul<Point> for &usize {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(*self as u64))
    }
}

impl Mul<&Point> for &usize {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(*self as u64))
    }
}

// Div -----------------------------------------------------

impl Div<usize> for Point {
    type Output = Point;

    fn div(self, rhs: usize) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs as u64).inverse().unwrap())
    }
}

impl Div<&usize> for Point {
    type Output = Point;

    fn div(self, rhs: &usize) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs as u64).inverse().unwrap())
    }
}

impl Div<usize> for &Point {
    type Output = Point;

    fn div(self, rhs: usize) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs as u64).inverse().unwrap())
    }
}

impl Div<&usize> for &Point {
    type Output = Point;

    fn div(self, rhs: &usize) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs as u64).inverse().unwrap())
    }
}

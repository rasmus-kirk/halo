use super::Point;

use ark_ff::fields::Field;
use halo_accumulation::group::PallasScalar;

use std::ops::{Div, Mul};

// Mul -----------------------------------------------------

impl Mul<u32> for Point {
    type Output = Point;

    fn mul(self, rhs: u32) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs))
    }
}

impl Mul<&u32> for Point {
    type Output = Point;

    fn mul(self, rhs: &u32) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs))
    }
}

impl Mul<u32> for &Point {
    type Output = Point;

    fn mul(self, rhs: u32) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs))
    }
}

impl Mul<&u32> for &Point {
    type Output = Point;

    fn mul(self, rhs: &u32) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs))
    }
}

impl Mul<Point> for u32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(self))
    }
}

impl Mul<&Point> for u32 {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(self))
    }
}

impl Mul<Point> for &u32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(*self))
    }
}

impl Mul<&Point> for &u32 {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(*self))
    }
}

// Div -----------------------------------------------------

impl Div<u32> for Point {
    type Output = Point;

    fn div(self, rhs: u32) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs).inverse().unwrap())
    }
}

impl Div<&u32> for Point {
    type Output = Point;

    fn div(self, rhs: &u32) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs).inverse().unwrap())
    }
}

impl Div<u32> for &Point {
    type Output = Point;

    fn div(self, rhs: u32) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs).inverse().unwrap())
    }
}

impl Div<&u32> for &Point {
    type Output = Point;

    fn div(self, rhs: &u32) -> Self::Output {
        Point::from(self.point * PallasScalar::from(*rhs).inverse().unwrap())
    }
}

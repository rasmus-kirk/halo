use super::Point;

use ark_ff::fields::Field;
use halo_accumulation::group::PallasScalar;

use std::ops::{Div, Mul};

// Mul -----------------------------------------------------

impl Mul<PallasScalar> for Point {
    type Output = Point;

    fn mul(self, rhs: PallasScalar) -> Self::Output {
        Point::from(self.point * rhs)
    }
}

impl Mul<&PallasScalar> for Point {
    type Output = Point;

    fn mul(self, rhs: &PallasScalar) -> Self::Output {
        Point::from(self.point * rhs)
    }
}

impl Mul<PallasScalar> for &Point {
    type Output = Point;

    fn mul(self, rhs: PallasScalar) -> Self::Output {
        Point::from(self.point * rhs)
    }
}

impl Mul<&PallasScalar> for &Point {
    type Output = Point;

    fn mul(self, rhs: &PallasScalar) -> Self::Output {
        Point::from(self.point * rhs)
    }
}

impl Mul<Point> for PallasScalar {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::from(rhs.point * self)
    }
}

impl Mul<&Point> for PallasScalar {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point::from(rhs.point * self)
    }
}

impl Mul<Point> for &PallasScalar {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::from(rhs.point * self)
    }
}

impl Mul<&Point> for &PallasScalar {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point::from(rhs.point * self)
    }
}

// Div -----------------------------------------------------

impl Div<PallasScalar> for Point {
    type Output = Point;

    fn div(self, rhs: PallasScalar) -> Self::Output {
        Point::from(self.point * rhs.inverse().unwrap())
    }
}

impl Div<&PallasScalar> for Point {
    type Output = Point;

    fn div(self, rhs: &PallasScalar) -> Self::Output {
        Point::from(self.point * rhs.inverse().unwrap())
    }
}

impl Div<PallasScalar> for &Point {
    type Output = Point;

    fn div(self, rhs: PallasScalar) -> Self::Output {
        Point::from(self.point * rhs.inverse().unwrap())
    }
}

impl Div<&PallasScalar> for &Point {
    type Output = Point;

    fn div(self, rhs: &PallasScalar) -> Self::Output {
        Point::from(self.point * rhs.inverse().unwrap())
    }
}

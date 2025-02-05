use crate::curve::{Point, Scalar};

use ark_ff::fields::Field;
use halo_accumulation::group::PallasScalar;

use std::ops::{Div, Mul};

// Mul -----------------------------------------------------

impl Mul<Scalar> for Point {
    type Output = Point;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs))
    }
}

impl Mul<&Scalar> for Point {
    type Output = Point;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs))
    }
}

impl Mul<Scalar> for &Point {
    type Output = Point;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs))
    }
}

impl Mul<&Scalar> for &Point {
    type Output = Point;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs))
    }
}

impl Mul<Point> for Scalar {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(self))
    }
}

impl Mul<&Point> for Scalar {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(self))
    }
}

impl Mul<Point> for &Scalar {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(self))
    }
}

impl Mul<&Point> for &Scalar {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        Point::from(rhs.point * PallasScalar::from(self))
    }
}

// Div -----------------------------------------------------

impl Div<Scalar> for Point {
    type Output = Point;

    fn div(self, rhs: Scalar) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs).inverse().unwrap())
    }
}

impl Div<&Scalar> for Point {
    type Output = Point;

    fn div(self, rhs: &Scalar) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs).inverse().unwrap())
    }
}

impl Div<Scalar> for &Point {
    type Output = Point;

    fn div(self, rhs: Scalar) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs).inverse().unwrap())
    }
}

impl Div<&Scalar> for &Point {
    type Output = Point;

    fn div(self, rhs: &Scalar) -> Self::Output {
        Point::from(self.point * PallasScalar::from(rhs).inverse().unwrap())
    }
}

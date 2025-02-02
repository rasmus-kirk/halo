mod op_i32;
mod op_i64;
mod op_pallas_scalar;
mod op_scalar;
mod op_u32;
mod op_u64;
mod op_usize;

use halo_accumulation::group::PallasPoint;

use std::ops::{Add, Neg, Sub};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    d: u64,
    point: PallasPoint,
}

impl Point {
    pub fn new(point: PallasPoint) -> Self {
        Self { d: 0, point }
    }

    pub fn new_d(d: u64, point: PallasPoint) -> Self {
        Self { d, point }
    }
}

impl From<PallasPoint> for Point {
    fn from(p: PallasPoint) -> Self {
        Point::new(p)
    }
}

impl From<&PallasPoint> for Point {
    fn from(p: &PallasPoint) -> Self {
        Point::new(*p)
    }
}

impl From<Point> for PallasPoint {
    fn from(p: Point) -> Self {
        p.point
    }
}

impl From<&Point> for PallasPoint {
    fn from(p: &Point) -> Self {
        p.point
    }
}

impl From<Point> for usize {
    fn from(p: Point) -> Self {
        p.d as usize
    }
}

impl From<&Point> for usize {
    fn from(p: &Point) -> Self {
        p.d as usize
    }
}

// Negate -----------------------------------------------------

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::new(-self.point)
    }
}

// Add -----------------------------------------------------

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point::new(self.point + other.point)
    }
}

impl Add<&Point> for Point {
    type Output = Point;

    fn add(self, other: &Point) -> Self::Output {
        Point::new(self.point + other.point)
    }
}

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point::new(self.point + other.point)
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point::new(self.point + other.point)
    }
}

// Sub -----------------------------------------------------
impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Self::Output {
        Point::new(self.point - other.point)
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: Self) -> Self::Output {
        Point::new(self.point - other.point)
    }
}

impl Sub<Point> for &Point {
    type Output = Point;

    fn sub(self, other: Point) -> Self::Output {
        Point::new(self.point - other.point)
    }
}

impl Sub<&Point> for Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Self::Output {
        Point::new(self.point - other.point)
    }
}

use super::Poly;

use ark_poly::polynomial::DenseUVPolynomial;
use halo_accumulation::group::{PallasPoly, PallasScalar};

use std::ops::{Add, Div, Mul, Sub};

impl From<i64> for Poly {
    fn from(val: i64) -> Self {
        Poly::new(PallasPoly::from_coefficients_vec(vec![PallasScalar::from(
            val,
        )]))
    }
}

impl From<&i64> for Poly {
    fn from(val: &i64) -> Self {
        Poly::new(PallasPoly::from_coefficients_vec(vec![PallasScalar::from(
            *val,
        )]))
    }
}

// Add -----------------------------------------------------

impl Add<i64> for Poly {
    type Output = Poly;

    fn add(self, rhs: i64) -> Self::Output {
        self + Poly::from(rhs)
    }
}

impl Add<&i64> for Poly {
    type Output = Poly;

    fn add(self, rhs: &i64) -> Self::Output {
        self + Poly::from(*rhs)
    }
}

impl Add<i64> for &Poly {
    type Output = Poly;

    fn add(self, rhs: i64) -> Self::Output {
        self + Poly::from(rhs)
    }
}

impl Add<&i64> for &Poly {
    type Output = Poly;

    fn add(self, rhs: &i64) -> Self::Output {
        self + Poly::from(*rhs)
    }
}

impl Add<Poly> for i64 {
    type Output = Poly;

    fn add(self, rhs: Poly) -> Self::Output {
        Poly::from(self) + rhs
    }
}

impl Add<&Poly> for i64 {
    type Output = Poly;

    fn add(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) + rhs
    }
}

impl Add<Poly> for &i64 {
    type Output = Poly;

    fn add(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) + rhs
    }
}

impl Add<&Poly> for &i64 {
    type Output = Poly;

    fn add(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) + rhs
    }
}

// Sub -----------------------------------------------------

impl Sub<i64> for Poly {
    type Output = Poly;

    fn sub(self, rhs: i64) -> Self::Output {
        self - Poly::from(rhs)
    }
}

impl Sub<&i64> for Poly {
    type Output = Poly;

    fn sub(self, rhs: &i64) -> Self::Output {
        self - Poly::from(*rhs)
    }
}

impl Sub<i64> for &Poly {
    type Output = Poly;

    fn sub(self, rhs: i64) -> Self::Output {
        self - Poly::from(rhs)
    }
}

impl Sub<&i64> for &Poly {
    type Output = Poly;

    fn sub(self, rhs: &i64) -> Self::Output {
        self - Poly::from(*rhs)
    }
}

impl Sub<Poly> for i64 {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        Poly::from(self) - rhs
    }
}

impl Sub<&Poly> for i64 {
    type Output = Poly;

    fn sub(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) - rhs
    }
}

impl Sub<Poly> for &i64 {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) - rhs
    }
}

impl Sub<&Poly> for &i64 {
    type Output = Poly;

    fn sub(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) - rhs
    }
}

// Mul -----------------------------------------------------

impl Mul<i64> for Poly {
    type Output = Poly;

    fn mul(self, rhs: i64) -> Self::Output {
        self * Poly::from(rhs)
    }
}

impl Mul<&i64> for Poly {
    type Output = Poly;

    fn mul(self, rhs: &i64) -> Self::Output {
        self * Poly::from(*rhs)
    }
}

impl Mul<i64> for &Poly {
    type Output = Poly;

    fn mul(self, rhs: i64) -> Self::Output {
        self * Poly::from(rhs)
    }
}

impl Mul<&i64> for &Poly {
    type Output = Poly;

    fn mul(self, rhs: &i64) -> Self::Output {
        self * Poly::from(*rhs)
    }
}

impl Mul<Poly> for i64 {
    type Output = Poly;

    fn mul(self, rhs: Poly) -> Self::Output {
        Poly::from(self) * rhs
    }
}

impl Mul<&Poly> for i64 {
    type Output = Poly;

    fn mul(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) * rhs
    }
}

impl Mul<Poly> for &i64 {
    type Output = Poly;

    fn mul(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) * rhs
    }
}

impl Mul<&Poly> for &i64 {
    type Output = Poly;

    fn mul(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) * rhs
    }
}

// Div -----------------------------------------------------

impl Div<i64> for Poly {
    type Output = Poly;

    fn div(self, rhs: i64) -> Self::Output {
        self / Poly::from(rhs)
    }
}

impl Div<&i64> for Poly {
    type Output = Poly;

    fn div(self, rhs: &i64) -> Self::Output {
        self / Poly::from(*rhs)
    }
}

impl Div<i64> for &Poly {
    type Output = Poly;

    fn div(self, rhs: i64) -> Self::Output {
        self / Poly::from(rhs)
    }
}

impl Div<&i64> for &Poly {
    type Output = Poly;

    fn div(self, rhs: &i64) -> Self::Output {
        self / Poly::from(*rhs)
    }
}

impl Div<Poly> for i64 {
    type Output = Poly;

    fn div(self, rhs: Poly) -> Self::Output {
        Poly::from(self) / rhs
    }
}

impl Div<&Poly> for i64 {
    type Output = Poly;

    fn div(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) / rhs
    }
}

impl Div<Poly> for &i64 {
    type Output = Poly;

    fn div(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) / rhs
    }
}

impl Div<&Poly> for &i64 {
    type Output = Poly;

    fn div(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) / rhs
    }
}

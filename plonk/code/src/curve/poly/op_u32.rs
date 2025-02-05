use super::Poly;

use ark_poly::polynomial::DenseUVPolynomial;
use halo_accumulation::group::{PallasPoly, PallasScalar};

use std::ops::{Add, Div, Mul, Sub};

impl From<u32> for Poly {
    fn from(val: u32) -> Self {
        Poly::new(PallasPoly::from_coefficients_vec(vec![PallasScalar::from(
            val,
        )]))
    }
}

impl From<&u32> for Poly {
    fn from(val: &u32) -> Self {
        Poly::new(PallasPoly::from_coefficients_vec(vec![PallasScalar::from(
            *val,
        )]))
    }
}

// Add -----------------------------------------------------

impl Add<u32> for Poly {
    type Output = Poly;

    fn add(self, rhs: u32) -> Self::Output {
        self + Poly::from(rhs)
    }
}

impl Add<&u32> for Poly {
    type Output = Poly;

    fn add(self, rhs: &u32) -> Self::Output {
        self + Poly::from(*rhs)
    }
}

impl Add<u32> for &Poly {
    type Output = Poly;

    fn add(self, rhs: u32) -> Self::Output {
        self + Poly::from(rhs)
    }
}

impl Add<&u32> for &Poly {
    type Output = Poly;

    fn add(self, rhs: &u32) -> Self::Output {
        self + Poly::from(*rhs)
    }
}

impl Add<Poly> for u32 {
    type Output = Poly;

    fn add(self, rhs: Poly) -> Self::Output {
        Poly::from(self) + rhs
    }
}

impl Add<&Poly> for u32 {
    type Output = Poly;

    fn add(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) + rhs
    }
}

impl Add<Poly> for &u32 {
    type Output = Poly;

    fn add(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) + rhs
    }
}

impl Add<&Poly> for &u32 {
    type Output = Poly;

    fn add(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) + rhs
    }
}

// Sub -----------------------------------------------------

impl Sub<u32> for Poly {
    type Output = Poly;

    fn sub(self, rhs: u32) -> Self::Output {
        self - Poly::from(rhs)
    }
}

impl Sub<&u32> for Poly {
    type Output = Poly;

    fn sub(self, rhs: &u32) -> Self::Output {
        self - Poly::from(*rhs)
    }
}

impl Sub<u32> for &Poly {
    type Output = Poly;

    fn sub(self, rhs: u32) -> Self::Output {
        self - Poly::from(rhs)
    }
}

impl Sub<&u32> for &Poly {
    type Output = Poly;

    fn sub(self, rhs: &u32) -> Self::Output {
        self - Poly::from(*rhs)
    }
}

impl Sub<Poly> for u32 {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        Poly::from(self) - rhs
    }
}

impl Sub<&Poly> for u32 {
    type Output = Poly;

    fn sub(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) - rhs
    }
}

impl Sub<Poly> for &u32 {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) - rhs
    }
}

impl Sub<&Poly> for &u32 {
    type Output = Poly;

    fn sub(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) - rhs
    }
}

// Mul -----------------------------------------------------

impl Mul<u32> for Poly {
    type Output = Poly;

    fn mul(self, rhs: u32) -> Self::Output {
        self * Poly::from(rhs)
    }
}

impl Mul<&u32> for Poly {
    type Output = Poly;

    fn mul(self, rhs: &u32) -> Self::Output {
        self * Poly::from(*rhs)
    }
}

impl Mul<u32> for &Poly {
    type Output = Poly;

    fn mul(self, rhs: u32) -> Self::Output {
        self * Poly::from(rhs)
    }
}

impl Mul<&u32> for &Poly {
    type Output = Poly;

    fn mul(self, rhs: &u32) -> Self::Output {
        self * Poly::from(*rhs)
    }
}

impl Mul<Poly> for u32 {
    type Output = Poly;

    fn mul(self, rhs: Poly) -> Self::Output {
        Poly::from(self) * rhs
    }
}

impl Mul<&Poly> for u32 {
    type Output = Poly;

    fn mul(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) * rhs
    }
}

impl Mul<Poly> for &u32 {
    type Output = Poly;

    fn mul(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) * rhs
    }
}

impl Mul<&Poly> for &u32 {
    type Output = Poly;

    fn mul(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) * rhs
    }
}

// Div -----------------------------------------------------

impl Div<u32> for Poly {
    type Output = Poly;

    fn div(self, rhs: u32) -> Self::Output {
        self / Poly::from(rhs)
    }
}

impl Div<&u32> for Poly {
    type Output = Poly;

    fn div(self, rhs: &u32) -> Self::Output {
        self / Poly::from(*rhs)
    }
}

impl Div<u32> for &Poly {
    type Output = Poly;

    fn div(self, rhs: u32) -> Self::Output {
        self / Poly::from(rhs)
    }
}

impl Div<&u32> for &Poly {
    type Output = Poly;

    fn div(self, rhs: &u32) -> Self::Output {
        self / Poly::from(*rhs)
    }
}

impl Div<Poly> for u32 {
    type Output = Poly;

    fn div(self, rhs: Poly) -> Self::Output {
        Poly::from(self) / rhs
    }
}

impl Div<&Poly> for u32 {
    type Output = Poly;

    fn div(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) / rhs
    }
}

impl Div<Poly> for &u32 {
    type Output = Poly;

    fn div(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) / rhs
    }
}

impl Div<&Poly> for &u32 {
    type Output = Poly;

    fn div(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) / rhs
    }
}

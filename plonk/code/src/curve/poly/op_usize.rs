use super::Poly;

use ark_poly::polynomial::DenseUVPolynomial;
use halo_accumulation::group::{PallasPoly, PallasScalar};

use std::ops::{Add, Div, Mul, Sub};

impl From<usize> for Poly {
    fn from(val: usize) -> Self {
        Poly::new(PallasPoly::from_coefficients_vec(vec![PallasScalar::from(
            val as u64,
        )]))
    }
}

impl From<&usize> for Poly {
    fn from(val: &usize) -> Self {
        Poly::new(PallasPoly::from_coefficients_vec(vec![PallasScalar::from(
            *val as u64,
        )]))
    }
}

// Add -----------------------------------------------------

impl Add<usize> for Poly {
    type Output = Poly;

    fn add(self, rhs: usize) -> Self::Output {
        self + Poly::from(rhs)
    }
}

impl Add<&usize> for Poly {
    type Output = Poly;

    fn add(self, rhs: &usize) -> Self::Output {
        self + Poly::from(*rhs)
    }
}

impl Add<usize> for &Poly {
    type Output = Poly;

    fn add(self, rhs: usize) -> Self::Output {
        self + Poly::from(rhs)
    }
}

impl Add<&usize> for &Poly {
    type Output = Poly;

    fn add(self, rhs: &usize) -> Self::Output {
        self + Poly::from(*rhs)
    }
}

impl Add<Poly> for usize {
    type Output = Poly;

    fn add(self, rhs: Poly) -> Self::Output {
        Poly::from(self) + rhs
    }
}

impl Add<&Poly> for usize {
    type Output = Poly;

    fn add(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) + rhs
    }
}

impl Add<Poly> for &usize {
    type Output = Poly;

    fn add(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) + rhs
    }
}

impl Add<&Poly> for &usize {
    type Output = Poly;

    fn add(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) + rhs
    }
}

// Sub -----------------------------------------------------

impl Sub<usize> for Poly {
    type Output = Poly;

    fn sub(self, rhs: usize) -> Self::Output {
        self - Poly::from(rhs)
    }
}

impl Sub<&usize> for Poly {
    type Output = Poly;

    fn sub(self, rhs: &usize) -> Self::Output {
        self - Poly::from(*rhs)
    }
}

impl Sub<usize> for &Poly {
    type Output = Poly;

    fn sub(self, rhs: usize) -> Self::Output {
        self - Poly::from(rhs)
    }
}

impl Sub<&usize> for &Poly {
    type Output = Poly;

    fn sub(self, rhs: &usize) -> Self::Output {
        self - Poly::from(*rhs)
    }
}

impl Sub<Poly> for usize {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        Poly::from(self) - rhs
    }
}

impl Sub<&Poly> for usize {
    type Output = Poly;

    fn sub(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) - rhs
    }
}

impl Sub<Poly> for &usize {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) - rhs
    }
}

impl Sub<&Poly> for &usize {
    type Output = Poly;

    fn sub(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) - rhs
    }
}

// Mul -----------------------------------------------------

impl Mul<usize> for Poly {
    type Output = Poly;

    fn mul(self, rhs: usize) -> Self::Output {
        self * Poly::from(rhs)
    }
}

impl Mul<&usize> for Poly {
    type Output = Poly;

    fn mul(self, rhs: &usize) -> Self::Output {
        self * Poly::from(*rhs)
    }
}

impl Mul<usize> for &Poly {
    type Output = Poly;

    fn mul(self, rhs: usize) -> Self::Output {
        self * Poly::from(rhs)
    }
}

impl Mul<&usize> for &Poly {
    type Output = Poly;

    fn mul(self, rhs: &usize) -> Self::Output {
        self * Poly::from(*rhs)
    }
}

impl Mul<Poly> for usize {
    type Output = Poly;

    fn mul(self, rhs: Poly) -> Self::Output {
        Poly::from(self) * rhs
    }
}

impl Mul<&Poly> for usize {
    type Output = Poly;

    fn mul(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) * rhs
    }
}

impl Mul<Poly> for &usize {
    type Output = Poly;

    fn mul(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) * rhs
    }
}

impl Mul<&Poly> for &usize {
    type Output = Poly;

    fn mul(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) * rhs
    }
}

// Div -----------------------------------------------------

impl Div<usize> for Poly {
    type Output = Poly;

    fn div(self, rhs: usize) -> Self::Output {
        self / Poly::from(rhs)
    }
}

impl Div<&usize> for Poly {
    type Output = Poly;

    fn div(self, rhs: &usize) -> Self::Output {
        self / Poly::from(*rhs)
    }
}

impl Div<usize> for &Poly {
    type Output = Poly;

    fn div(self, rhs: usize) -> Self::Output {
        self / Poly::from(rhs)
    }
}

impl Div<&usize> for &Poly {
    type Output = Poly;

    fn div(self, rhs: &usize) -> Self::Output {
        self / Poly::from(*rhs)
    }
}

impl Div<Poly> for usize {
    type Output = Poly;

    fn div(self, rhs: Poly) -> Self::Output {
        Poly::from(self) / rhs
    }
}

impl Div<&Poly> for usize {
    type Output = Poly;

    fn div(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) / rhs
    }
}

impl Div<Poly> for &usize {
    type Output = Poly;

    fn div(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) / rhs
    }
}

impl Div<&Poly> for &usize {
    type Output = Poly;

    fn div(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) / rhs
    }
}

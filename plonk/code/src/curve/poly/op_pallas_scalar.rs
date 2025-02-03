use super::Poly;

use ark_poly::polynomial::DenseUVPolynomial;
use halo_accumulation::group::{PallasPoly, PallasScalar};

use std::ops::{Add, Div, Mul, Sub};

impl From<PallasScalar> for Poly {
    fn from(scalar: PallasScalar) -> Self {
        Poly::new(PallasPoly::from_coefficients_vec(vec![scalar]))
    }
}

impl From<&PallasScalar> for Poly {
    fn from(scalar: &PallasScalar) -> Self {
        Poly::new(PallasPoly::from_coefficients_vec(vec![*scalar]))
    }
}

// Add -----------------------------------------------------

impl Add<PallasScalar> for Poly {
    type Output = Poly;

    fn add(self, rhs: PallasScalar) -> Self::Output {
        self + Poly::from(rhs)
    }
}

impl Add<&PallasScalar> for Poly {
    type Output = Poly;

    fn add(self, rhs: &PallasScalar) -> Self::Output {
        self + Poly::from(*rhs)
    }
}

impl Add<PallasScalar> for &Poly {
    type Output = Poly;

    fn add(self, rhs: PallasScalar) -> Self::Output {
        self + Poly::from(rhs)
    }
}

impl Add<&PallasScalar> for &Poly {
    type Output = Poly;

    fn add(self, rhs: &PallasScalar) -> Self::Output {
        self + Poly::from(*rhs)
    }
}

impl Add<Poly> for PallasScalar {
    type Output = Poly;

    fn add(self, rhs: Poly) -> Self::Output {
        Poly::from(self) + rhs
    }
}

impl Add<&Poly> for PallasScalar {
    type Output = Poly;

    fn add(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) + rhs
    }
}

impl Add<Poly> for &PallasScalar {
    type Output = Poly;

    fn add(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) + rhs
    }
}

impl Add<&Poly> for &PallasScalar {
    type Output = Poly;

    fn add(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) + rhs
    }
}

// Sub -----------------------------------------------------

impl Sub<PallasScalar> for Poly {
    type Output = Poly;

    fn sub(self, rhs: PallasScalar) -> Self::Output {
        self - Poly::from(rhs)
    }
}

impl Sub<&PallasScalar> for Poly {
    type Output = Poly;

    fn sub(self, rhs: &PallasScalar) -> Self::Output {
        self - Poly::from(*rhs)
    }
}

impl Sub<PallasScalar> for &Poly {
    type Output = Poly;

    fn sub(self, rhs: PallasScalar) -> Self::Output {
        self - Poly::from(rhs)
    }
}

impl Sub<&PallasScalar> for &Poly {
    type Output = Poly;

    fn sub(self, rhs: &PallasScalar) -> Self::Output {
        self - Poly::from(*rhs)
    }
}

impl Sub<Poly> for PallasScalar {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        Poly::from(self) - rhs
    }
}

impl Sub<&Poly> for PallasScalar {
    type Output = Poly;

    fn sub(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) - rhs
    }
}

impl Sub<Poly> for &PallasScalar {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) - rhs
    }
}

impl Sub<&Poly> for &PallasScalar {
    type Output = Poly;

    fn sub(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) - rhs
    }
}

// Mul -----------------------------------------------------

impl Mul<PallasScalar> for Poly {
    type Output = Poly;

    fn mul(self, rhs: PallasScalar) -> Self::Output {
        self * Poly::from(rhs)
    }
}

impl Mul<&PallasScalar> for Poly {
    type Output = Poly;

    fn mul(self, rhs: &PallasScalar) -> Self::Output {
        self * Poly::from(*rhs)
    }
}

impl Mul<PallasScalar> for &Poly {
    type Output = Poly;

    fn mul(self, rhs: PallasScalar) -> Self::Output {
        self * Poly::from(rhs)
    }
}

impl Mul<&PallasScalar> for &Poly {
    type Output = Poly;

    fn mul(self, rhs: &PallasScalar) -> Self::Output {
        self * Poly::from(*rhs)
    }
}

impl Mul<Poly> for PallasScalar {
    type Output = Poly;

    fn mul(self, rhs: Poly) -> Self::Output {
        Poly::from(self) * rhs
    }
}

impl Mul<&Poly> for PallasScalar {
    type Output = Poly;

    fn mul(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) * rhs
    }
}

impl Mul<Poly> for &PallasScalar {
    type Output = Poly;

    fn mul(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) * rhs
    }
}

impl Mul<&Poly> for &PallasScalar {
    type Output = Poly;

    fn mul(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) * rhs
    }
}

// Div -----------------------------------------------------

impl Div<PallasScalar> for Poly {
    type Output = Poly;

    fn div(self, rhs: PallasScalar) -> Self::Output {
        self / Poly::from(rhs)
    }
}

impl Div<&PallasScalar> for Poly {
    type Output = Poly;

    fn div(self, rhs: &PallasScalar) -> Self::Output {
        self / Poly::from(*rhs)
    }
}

impl Div<PallasScalar> for &Poly {
    type Output = Poly;

    fn div(self, rhs: PallasScalar) -> Self::Output {
        self / Poly::from(rhs)
    }
}

impl Div<&PallasScalar> for &Poly {
    type Output = Poly;

    fn div(self, rhs: &PallasScalar) -> Self::Output {
        self / Poly::from(*rhs)
    }
}

impl Div<Poly> for PallasScalar {
    type Output = Poly;

    fn div(self, rhs: Poly) -> Self::Output {
        Poly::from(self) / rhs
    }
}

impl Div<&Poly> for PallasScalar {
    type Output = Poly;

    fn div(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) / rhs
    }
}

impl Div<Poly> for &PallasScalar {
    type Output = Poly;

    fn div(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) / rhs
    }
}

impl Div<&Poly> for &PallasScalar {
    type Output = Poly;

    fn div(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) / rhs
    }
}

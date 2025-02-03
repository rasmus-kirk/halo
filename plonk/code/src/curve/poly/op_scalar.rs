use crate::curve::{Poly, Scalar};

use ark_poly::polynomial::DenseUVPolynomial;
use halo_accumulation::group::PallasPoly;

use std::ops::{Add, Div, Mul, Sub};

impl From<Scalar> for Poly {
    fn from(scalar: Scalar) -> Self {
        Poly::new(PallasPoly::from_coefficients_vec(vec![scalar.into()]))
    }
}

impl From<&Scalar> for Poly {
    fn from(scalar: &Scalar) -> Self {
        Poly::new(PallasPoly::from_coefficients_vec(vec![(*scalar).into()]))
    }
}

// Add -----------------------------------------------------

impl Add<Scalar> for Poly {
    type Output = Poly;

    fn add(self, rhs: Scalar) -> Self::Output {
        self + Poly::from(rhs)
    }
}

impl Add<&Scalar> for Poly {
    type Output = Poly;

    fn add(self, rhs: &Scalar) -> Self::Output {
        self + Poly::from(*rhs)
    }
}

impl Add<Scalar> for &Poly {
    type Output = Poly;

    fn add(self, rhs: Scalar) -> Self::Output {
        self + Poly::from(rhs)
    }
}

impl Add<&Scalar> for &Poly {
    type Output = Poly;

    fn add(self, rhs: &Scalar) -> Self::Output {
        self + Poly::from(*rhs)
    }
}

impl Add<Poly> for Scalar {
    type Output = Poly;

    fn add(self, rhs: Poly) -> Self::Output {
        Poly::from(self) + rhs
    }
}

impl Add<&Poly> for Scalar {
    type Output = Poly;

    fn add(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) + rhs
    }
}

impl Add<Poly> for &Scalar {
    type Output = Poly;

    fn add(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) + rhs
    }
}

impl Add<&Poly> for &Scalar {
    type Output = Poly;

    fn add(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) + rhs
    }
}

// Sub -----------------------------------------------------

impl Sub<Scalar> for Poly {
    type Output = Poly;

    fn sub(self, rhs: Scalar) -> Self::Output {
        self - Poly::from(rhs)
    }
}

impl Sub<&Scalar> for Poly {
    type Output = Poly;

    fn sub(self, rhs: &Scalar) -> Self::Output {
        self - Poly::from(*rhs)
    }
}

impl Sub<Scalar> for &Poly {
    type Output = Poly;

    fn sub(self, rhs: Scalar) -> Self::Output {
        self - Poly::from(rhs)
    }
}

impl Sub<&Scalar> for &Poly {
    type Output = Poly;

    fn sub(self, rhs: &Scalar) -> Self::Output {
        self - Poly::from(*rhs)
    }
}

impl Sub<Poly> for Scalar {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        Poly::from(self) - rhs
    }
}

impl Sub<&Poly> for Scalar {
    type Output = Poly;

    fn sub(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) - rhs
    }
}

impl Sub<Poly> for &Scalar {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) - rhs
    }
}

impl Sub<&Poly> for &Scalar {
    type Output = Poly;

    fn sub(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) - rhs
    }
}

// Mul -----------------------------------------------------

impl Mul<Scalar> for Poly {
    type Output = Poly;

    fn mul(self, rhs: Scalar) -> Self::Output {
        self * Poly::from(rhs)
    }
}

impl Mul<&Scalar> for Poly {
    type Output = Poly;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        self * Poly::from(*rhs)
    }
}

impl Mul<Scalar> for &Poly {
    type Output = Poly;

    fn mul(self, rhs: Scalar) -> Self::Output {
        self * Poly::from(rhs)
    }
}

impl Mul<&Scalar> for &Poly {
    type Output = Poly;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        self * Poly::from(*rhs)
    }
}

impl Mul<Poly> for Scalar {
    type Output = Poly;

    fn mul(self, rhs: Poly) -> Self::Output {
        Poly::from(self) * rhs
    }
}

impl Mul<&Poly> for Scalar {
    type Output = Poly;

    fn mul(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) * rhs
    }
}

impl Mul<Poly> for &Scalar {
    type Output = Poly;

    fn mul(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) * rhs
    }
}

impl Mul<&Poly> for &Scalar {
    type Output = Poly;

    fn mul(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) * rhs
    }
}

// Div -----------------------------------------------------

impl Div<Scalar> for Poly {
    type Output = Poly;

    fn div(self, rhs: Scalar) -> Self::Output {
        self / Poly::from(rhs)
    }
}

impl Div<&Scalar> for Poly {
    type Output = Poly;

    fn div(self, rhs: &Scalar) -> Self::Output {
        self / Poly::from(*rhs)
    }
}

impl Div<Scalar> for &Poly {
    type Output = Poly;

    fn div(self, rhs: Scalar) -> Self::Output {
        self / Poly::from(rhs)
    }
}

impl Div<&Scalar> for &Poly {
    type Output = Poly;

    fn div(self, rhs: &Scalar) -> Self::Output {
        self / Poly::from(*rhs)
    }
}

impl Div<Poly> for Scalar {
    type Output = Poly;

    fn div(self, rhs: Poly) -> Self::Output {
        Poly::from(self) / rhs
    }
}

impl Div<&Poly> for Scalar {
    type Output = Poly;

    fn div(self, rhs: &Poly) -> Self::Output {
        Poly::from(self) / rhs
    }
}

impl Div<Poly> for &Scalar {
    type Output = Poly;

    fn div(self, rhs: Poly) -> Self::Output {
        Poly::from(*self) / rhs
    }
}

impl Div<&Poly> for &Scalar {
    type Output = Poly;

    fn div(self, rhs: &Poly) -> Self::Output {
        Poly::from(*self) / rhs
    }
}

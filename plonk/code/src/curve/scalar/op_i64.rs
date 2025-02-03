use super::Scalar;

use halo_accumulation::group::PallasScalar;

use std::ops::{Add, Div, Mul, Sub};

impl From<i64> for Scalar {
    fn from(value: i64) -> Self {
        Scalar {
            scalar: PallasScalar::from(value),
        }
    }
}

impl From<&i64> for Scalar {
    fn from(value: &i64) -> Self {
        Scalar {
            scalar: PallasScalar::from(*value),
        }
    }
}

// Add -----------------------------------------------------

impl Add<i64> for Scalar {
    type Output = Scalar;

    fn add(self, rhs: i64) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<&i64> for Scalar {
    type Output = Scalar;

    fn add(self, rhs: &i64) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<i64> for &Scalar {
    type Output = Scalar;

    fn add(self, rhs: i64) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<&i64> for &Scalar {
    type Output = Scalar;

    fn add(self, rhs: &i64) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<Scalar> for i64 {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

impl Add<Scalar> for &i64 {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

impl Add<&Scalar> for i64 {
    type Output = Scalar;

    fn add(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

impl Add<&Scalar> for &i64 {
    type Output = Scalar;

    fn add(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

// Sub -----------------------------------------------------

impl Sub<i64> for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: i64) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<&i64> for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: &i64) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<i64> for &Scalar {
    type Output = Scalar;

    fn sub(self, rhs: i64) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<&i64> for &Scalar {
    type Output = Scalar;

    fn sub(self, rhs: &i64) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<Scalar> for i64 {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

impl Sub<Scalar> for &i64 {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

impl Sub<&Scalar> for i64 {
    type Output = Scalar;

    fn sub(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

impl Sub<&Scalar> for &i64 {
    type Output = Scalar;

    fn sub(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

// Mul -----------------------------------------------------

impl Mul<i64> for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: i64) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<&i64> for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &i64) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<i64> for &Scalar {
    type Output = Scalar;

    fn mul(self, rhs: i64) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<&i64> for &Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &i64) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<Scalar> for i64 {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

impl Mul<Scalar> for &i64 {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

impl Mul<&Scalar> for i64 {
    type Output = Scalar;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

impl Mul<&Scalar> for &i64 {
    type Output = Scalar;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

// Div -----------------------------------------------------

impl Div<i64> for Scalar {
    type Output = Scalar;

    fn div(self, rhs: i64) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<&i64> for Scalar {
    type Output = Scalar;

    fn div(self, rhs: &i64) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<i64> for &Scalar {
    type Output = Scalar;

    fn div(self, rhs: i64) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<&i64> for &Scalar {
    type Output = Scalar;

    fn div(self, rhs: &i64) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<Scalar> for i64 {
    type Output = Scalar;

    fn div(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

impl Div<Scalar> for &i64 {
    type Output = Scalar;

    fn div(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

impl Div<&Scalar> for i64 {
    type Output = Scalar;

    fn div(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

impl Div<&Scalar> for &i64 {
    type Output = Scalar;

    fn div(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

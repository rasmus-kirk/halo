use super::Scalar;

use halo_accumulation::group::PallasScalar;

use std::ops::{Add, Div, Mul, Sub};

impl From<i32> for Scalar {
    fn from(value: i32) -> Self {
        Scalar {
            scalar: PallasScalar::from(value),
        }
    }
}

impl From<&i32> for Scalar {
    fn from(value: &i32) -> Self {
        Scalar {
            scalar: PallasScalar::from(*value),
        }
    }
}

// Add -----------------------------------------------------

impl Add<i32> for Scalar {
    type Output = Scalar;

    fn add(self, rhs: i32) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<&i32> for Scalar {
    type Output = Scalar;

    fn add(self, rhs: &i32) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<i32> for &Scalar {
    type Output = Scalar;

    fn add(self, rhs: i32) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<&i32> for &Scalar {
    type Output = Scalar;

    fn add(self, rhs: &i32) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<Scalar> for i32 {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

impl Add<Scalar> for &i32 {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

impl Add<&Scalar> for i32 {
    type Output = Scalar;

    fn add(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

impl Add<&Scalar> for &i32 {
    type Output = Scalar;

    fn add(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

// Sub -----------------------------------------------------

impl Sub<i32> for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: i32) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<&i32> for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: &i32) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<i32> for &Scalar {
    type Output = Scalar;

    fn sub(self, rhs: i32) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<&i32> for &Scalar {
    type Output = Scalar;

    fn sub(self, rhs: &i32) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<Scalar> for i32 {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

impl Sub<Scalar> for &i32 {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

impl Sub<&Scalar> for i32 {
    type Output = Scalar;

    fn sub(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

impl Sub<&Scalar> for &i32 {
    type Output = Scalar;

    fn sub(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

// Mul -----------------------------------------------------

impl Mul<i32> for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: i32) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<&i32> for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &i32) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<i32> for &Scalar {
    type Output = Scalar;

    fn mul(self, rhs: i32) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<&i32> for &Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &i32) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<Scalar> for i32 {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

impl Mul<Scalar> for &i32 {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

impl Mul<&Scalar> for i32 {
    type Output = Scalar;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

impl Mul<&Scalar> for &i32 {
    type Output = Scalar;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

// Div -----------------------------------------------------

impl Div<i32> for Scalar {
    type Output = Scalar;

    fn div(self, rhs: i32) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<&i32> for Scalar {
    type Output = Scalar;

    fn div(self, rhs: &i32) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<i32> for &Scalar {
    type Output = Scalar;

    fn div(self, rhs: i32) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<&i32> for &Scalar {
    type Output = Scalar;

    fn div(self, rhs: &i32) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<Scalar> for i32 {
    type Output = Scalar;

    fn div(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

impl Div<Scalar> for &i32 {
    type Output = Scalar;

    fn div(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

impl Div<&Scalar> for i32 {
    type Output = Scalar;

    fn div(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

impl Div<&Scalar> for &i32 {
    type Output = Scalar;

    fn div(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

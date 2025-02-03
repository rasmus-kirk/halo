use super::Scalar;

use halo_accumulation::group::PallasScalar;

use std::ops::{Add, Div, Mul, Sub};

impl From<u32> for Scalar {
    fn from(value: u32) -> Self {
        Scalar {
            scalar: PallasScalar::from(value),
        }
    }
}

impl From<&u32> for Scalar {
    fn from(value: &u32) -> Self {
        Scalar {
            scalar: PallasScalar::from(*value),
        }
    }
}

// Add -----------------------------------------------------

impl Add<u32> for Scalar {
    type Output = Scalar;

    fn add(self, rhs: u32) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<&u32> for Scalar {
    type Output = Scalar;

    fn add(self, rhs: &u32) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<u32> for &Scalar {
    type Output = Scalar;

    fn add(self, rhs: u32) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<&u32> for &Scalar {
    type Output = Scalar;

    fn add(self, rhs: &u32) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<Scalar> for u32 {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

impl Add<Scalar> for &u32 {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

impl Add<&Scalar> for u32 {
    type Output = Scalar;

    fn add(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

impl Add<&Scalar> for &u32 {
    type Output = Scalar;

    fn add(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

// Sub -----------------------------------------------------

impl Sub<u32> for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: u32) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<&u32> for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: &u32) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<u32> for &Scalar {
    type Output = Scalar;

    fn sub(self, rhs: u32) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<&u32> for &Scalar {
    type Output = Scalar;

    fn sub(self, rhs: &u32) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<Scalar> for u32 {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

impl Sub<Scalar> for &u32 {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

impl Sub<&Scalar> for u32 {
    type Output = Scalar;

    fn sub(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

impl Sub<&Scalar> for &u32 {
    type Output = Scalar;

    fn sub(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

// Mul -----------------------------------------------------

impl Mul<u32> for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: u32) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<&u32> for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &u32) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<u32> for &Scalar {
    type Output = Scalar;

    fn mul(self, rhs: u32) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<&u32> for &Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &u32) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<Scalar> for u32 {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

impl Mul<Scalar> for &u32 {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

impl Mul<&Scalar> for u32 {
    type Output = Scalar;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

impl Mul<&Scalar> for &u32 {
    type Output = Scalar;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

// Div -----------------------------------------------------

impl Div<u32> for Scalar {
    type Output = Scalar;

    fn div(self, rhs: u32) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<&u32> for Scalar {
    type Output = Scalar;

    fn div(self, rhs: &u32) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<u32> for &Scalar {
    type Output = Scalar;

    fn div(self, rhs: u32) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<&u32> for &Scalar {
    type Output = Scalar;

    fn div(self, rhs: &u32) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<Scalar> for u32 {
    type Output = Scalar;

    fn div(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

impl Div<Scalar> for &u32 {
    type Output = Scalar;

    fn div(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

impl Div<&Scalar> for u32 {
    type Output = Scalar;

    fn div(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

impl Div<&Scalar> for &u32 {
    type Output = Scalar;

    fn div(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

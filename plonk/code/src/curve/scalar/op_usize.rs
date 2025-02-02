use super::Scalar;

use halo_accumulation::group::PallasScalar;

use std::ops::{Add, Div, Mul, Sub};

impl From<usize> for Scalar {
    fn from(value: usize) -> Self {
        Scalar {
            scalar: PallasScalar::from(value as u64),
        }
    }
}

impl From<&usize> for Scalar {
    fn from(value: &usize) -> Self {
        Scalar {
            scalar: PallasScalar::from(*value as u64),
        }
    }
}

// Add -----------------------------------------------------

impl Add<usize> for Scalar {
    type Output = Scalar;

    fn add(self, rhs: usize) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<&usize> for Scalar {
    type Output = Scalar;

    fn add(self, rhs: &usize) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<usize> for &Scalar {
    type Output = Scalar;

    fn add(self, rhs: usize) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<&usize> for &Scalar {
    type Output = Scalar;

    fn add(self, rhs: &usize) -> Self::Output {
        self + Scalar::from(rhs)
    }
}

impl Add<Scalar> for usize {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

impl Add<Scalar> for &usize {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

impl Add<&Scalar> for usize {
    type Output = Scalar;

    fn add(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

impl Add<&Scalar> for &usize {
    type Output = Scalar;

    fn add(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) + rhs
    }
}

// Sub -----------------------------------------------------

impl Sub<usize> for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: usize) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<&usize> for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: &usize) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<usize> for &Scalar {
    type Output = Scalar;

    fn sub(self, rhs: usize) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<&usize> for &Scalar {
    type Output = Scalar;

    fn sub(self, rhs: &usize) -> Self::Output {
        self - Scalar::from(rhs)
    }
}

impl Sub<Scalar> for usize {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

impl Sub<Scalar> for &usize {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

impl Sub<&Scalar> for usize {
    type Output = Scalar;

    fn sub(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

impl Sub<&Scalar> for &usize {
    type Output = Scalar;

    fn sub(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) - rhs
    }
}

// Mul -----------------------------------------------------

impl Mul<usize> for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: usize) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<&usize> for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &usize) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<usize> for &Scalar {
    type Output = Scalar;

    fn mul(self, rhs: usize) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<&usize> for &Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &usize) -> Self::Output {
        self * Scalar::from(rhs)
    }
}

impl Mul<Scalar> for usize {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

impl Mul<Scalar> for &usize {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

impl Mul<&Scalar> for usize {
    type Output = Scalar;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

impl Mul<&Scalar> for &usize {
    type Output = Scalar;

    fn mul(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) * rhs
    }
}

// Div -----------------------------------------------------

impl Div<usize> for Scalar {
    type Output = Scalar;

    fn div(self, rhs: usize) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<&usize> for Scalar {
    type Output = Scalar;

    fn div(self, rhs: &usize) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<usize> for &Scalar {
    type Output = Scalar;

    fn div(self, rhs: usize) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<&usize> for &Scalar {
    type Output = Scalar;

    fn div(self, rhs: &usize) -> Self::Output {
        self / Scalar::from(rhs)
    }
}

impl Div<Scalar> for usize {
    type Output = Scalar;

    fn div(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

impl Div<Scalar> for &usize {
    type Output = Scalar;

    fn div(self, rhs: Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

impl Div<&Scalar> for usize {
    type Output = Scalar;

    fn div(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

impl Div<&Scalar> for &usize {
    type Output = Scalar;

    fn div(self, rhs: &Scalar) -> Self::Output {
        Scalar::from(self) / rhs
    }
}

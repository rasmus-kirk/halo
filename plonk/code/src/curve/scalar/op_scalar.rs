use super::Scalar;

use halo_accumulation::group::PallasScalar;

use std::ops::{Add, Div, Mul, Sub};

impl From<PallasScalar> for Scalar {
    fn from(scalar: PallasScalar) -> Self {
        Scalar { scalar }
    }
}

impl From<Scalar> for PallasScalar {
    fn from(scalar: Scalar) -> Self {
        scalar.scalar
    }
}

impl From<&PallasScalar> for Scalar {
    fn from(scalar: &PallasScalar) -> Self {
        Scalar { scalar: *scalar }
    }
}

impl From<&Scalar> for PallasScalar {
    fn from(scalar: &Scalar) -> Self {
        scalar.scalar
    }
}

// Add -----------------------------------------------------

impl Add<PallasScalar> for Scalar {
    type Output = Scalar;

    fn add(self, other: PallasScalar) -> Self::Output {
        Scalar::from(self.scalar + other)
    }
}

impl Add<&PallasScalar> for Scalar {
    type Output = Scalar;

    fn add(self, other: &PallasScalar) -> Self::Output {
        Scalar::from(self.scalar + *other)
    }
}

impl Add<PallasScalar> for &Scalar {
    type Output = Scalar;

    fn add(self, other: PallasScalar) -> Self::Output {
        Scalar::from(self.scalar + other)
    }
}

impl Add<&PallasScalar> for &Scalar {
    type Output = Scalar;

    fn add(self, other: &PallasScalar) -> Self::Output {
        Scalar::from(self.scalar + *other)
    }
}

impl Add<Scalar> for PallasScalar {
    type Output = Scalar;

    fn add(self, other: Scalar) -> Self::Output {
        Scalar::from(self + other.scalar)
    }
}

impl Add<&Scalar> for PallasScalar {
    type Output = Scalar;

    fn add(self, other: &Scalar) -> Self::Output {
        Scalar::from(self + other.scalar)
    }
}

impl Add<Scalar> for &PallasScalar {
    type Output = Scalar;

    fn add(self, other: Scalar) -> Self::Output {
        Scalar::from(*self + other.scalar)
    }
}

impl Add<&Scalar> for &PallasScalar {
    type Output = Scalar;

    fn add(self, other: &Scalar) -> Self::Output {
        Scalar::from(*self + other.scalar)
    }
}

// Sub -----------------------------------------------------

impl Sub<PallasScalar> for Scalar {
    type Output = Scalar;

    fn sub(self, other: PallasScalar) -> Self::Output {
        Scalar::from(self.scalar - other)
    }
}

impl Sub<&PallasScalar> for Scalar {
    type Output = Scalar;

    fn sub(self, other: &PallasScalar) -> Self::Output {
        Scalar::from(self.scalar - *other)
    }
}

impl Sub<PallasScalar> for &Scalar {
    type Output = Scalar;

    fn sub(self, other: PallasScalar) -> Self::Output {
        Scalar::from(self.scalar - other)
    }
}

impl Sub<&PallasScalar> for &Scalar {
    type Output = Scalar;

    fn sub(self, other: &PallasScalar) -> Self::Output {
        Scalar::from(self.scalar - *other)
    }
}

impl Sub<Scalar> for PallasScalar {
    type Output = Scalar;

    fn sub(self, other: Scalar) -> Self::Output {
        Scalar::from(self - other.scalar)
    }
}

impl Sub<&Scalar> for PallasScalar {
    type Output = Scalar;

    fn sub(self, other: &Scalar) -> Self::Output {
        Scalar::from(self - other.scalar)
    }
}

impl Sub<Scalar> for &PallasScalar {
    type Output = Scalar;

    fn sub(self, other: Scalar) -> Self::Output {
        Scalar::from(*self - other.scalar)
    }
}

impl Sub<&Scalar> for &PallasScalar {
    type Output = Scalar;

    fn sub(self, other: &Scalar) -> Self::Output {
        Scalar::from(*self - other.scalar)
    }
}

// Mul -----------------------------------------------------

impl Mul<PallasScalar> for Scalar {
    type Output = Scalar;

    fn mul(self, other: PallasScalar) -> Self::Output {
        Scalar::from(self.scalar * other)
    }
}

impl Mul<&PallasScalar> for Scalar {
    type Output = Scalar;

    fn mul(self, other: &PallasScalar) -> Self::Output {
        Scalar::from(self.scalar * *other)
    }
}

impl Mul<PallasScalar> for &Scalar {
    type Output = Scalar;

    fn mul(self, other: PallasScalar) -> Self::Output {
        Scalar::from(self.scalar * other)
    }
}

impl Mul<&PallasScalar> for &Scalar {
    type Output = Scalar;

    fn mul(self, other: &PallasScalar) -> Self::Output {
        Scalar::from(self.scalar * *other)
    }
}

impl Mul<Scalar> for PallasScalar {
    type Output = Scalar;

    fn mul(self, other: Scalar) -> Self::Output {
        Scalar::from(self * other.scalar)
    }
}

impl Mul<&Scalar> for PallasScalar {
    type Output = Scalar;

    fn mul(self, other: &Scalar) -> Self::Output {
        Scalar::from(self * other.scalar)
    }
}

impl Mul<Scalar> for &PallasScalar {
    type Output = Scalar;

    fn mul(self, other: Scalar) -> Self::Output {
        Scalar::from(*self * other.scalar)
    }
}

impl Mul<&Scalar> for &PallasScalar {
    type Output = Scalar;

    fn mul(self, other: &Scalar) -> Self::Output {
        Scalar::from(*self * other.scalar)
    }
}

// Div -----------------------------------------------------

impl Div<PallasScalar> for Scalar {
    type Output = Scalar;

    fn div(self, other: PallasScalar) -> Self::Output {
        Scalar::from(self.scalar / other)
    }
}

impl Div<&PallasScalar> for Scalar {
    type Output = Scalar;

    fn div(self, other: &PallasScalar) -> Self::Output {
        Scalar::from(self.scalar / *other)
    }
}

impl Div<PallasScalar> for &Scalar {
    type Output = Scalar;

    fn div(self, other: PallasScalar) -> Self::Output {
        Scalar::from(self.scalar / other)
    }
}

impl Div<&PallasScalar> for &Scalar {
    type Output = Scalar;

    fn div(self, other: &PallasScalar) -> Self::Output {
        Scalar::from(self.scalar / *other)
    }
}

impl Div<Scalar> for PallasScalar {
    type Output = Scalar;

    fn div(self, other: Scalar) -> Self::Output {
        Scalar::from(self / other.scalar)
    }
}

impl Div<&Scalar> for PallasScalar {
    type Output = Scalar;

    fn div(self, other: &Scalar) -> Self::Output {
        Scalar::from(self / other.scalar)
    }
}

impl Div<Scalar> for &PallasScalar {
    type Output = Scalar;

    fn div(self, other: Scalar) -> Self::Output {
        Scalar::from(*self / other.scalar)
    }
}

impl Div<&Scalar> for &PallasScalar {
    type Output = Scalar;

    fn div(self, other: &Scalar) -> Self::Output {
        Scalar::from(*self / other.scalar)
    }
}

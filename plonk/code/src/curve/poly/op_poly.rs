use super::Poly;

use halo_accumulation::group::PallasPoly;

use std::ops::{Add, Div, Mul, Sub};

impl From<PallasPoly> for Poly {
    fn from(poly: PallasPoly) -> Self {
        Poly::new(poly)
    }
}

impl From<Poly> for PallasPoly {
    fn from(poly: Poly) -> Self {
        poly.poly
    }
}

impl From<&PallasPoly> for Poly {
    fn from(poly: &PallasPoly) -> Self {
        Poly::new(poly.clone())
    }
}

impl From<&Poly> for PallasPoly {
    fn from(poly: &Poly) -> Self {
        poly.poly.clone()
    }
}

// Add -----------------------------------------------------

impl Add<PallasPoly> for Poly {
    type Output = Self;

    fn add(self, rhs: PallasPoly) -> Self::Output {
        Poly::from(self.poly + rhs)
    }
}

impl Add<&PallasPoly> for Poly {
    type Output = Self;

    fn add(self, rhs: &PallasPoly) -> Self::Output {
        Poly::from(self.poly + rhs.clone())
    }
}

impl Add<PallasPoly> for &Poly {
    type Output = Poly;

    fn add(self, rhs: PallasPoly) -> Self::Output {
        Poly::from(self.poly.clone() + rhs)
    }
}

impl Add<&PallasPoly> for &Poly {
    type Output = Poly;

    fn add(self, rhs: &PallasPoly) -> Self::Output {
        Poly::from(self.poly.clone() + rhs.clone())
    }
}

impl Add<Poly> for PallasPoly {
    type Output = Self;

    fn add(self, rhs: Poly) -> Self::Output {
        self + rhs.poly
    }
}

impl Add<&Poly> for PallasPoly {
    type Output = PallasPoly;

    fn add(self, rhs: &Poly) -> Self::Output {
        self + rhs.poly.clone()
    }
}

impl Add<Poly> for &PallasPoly {
    type Output = PallasPoly;

    fn add(self, rhs: Poly) -> Self::Output {
        self.clone() + rhs.poly
    }
}

impl Add<&Poly> for &PallasPoly {
    type Output = PallasPoly;

    fn add(self, rhs: &Poly) -> Self::Output {
        self.clone() + rhs.poly.clone()
    }
}

// Sub -----------------------------------------------------

impl Sub<PallasPoly> for Poly {
    type Output = Self;

    fn sub(self, rhs: PallasPoly) -> Self::Output {
        Poly::from(self.poly - rhs)
    }
}

impl Sub<&PallasPoly> for Poly {
    type Output = Self;

    fn sub(self, rhs: &PallasPoly) -> Self::Output {
        Poly::from(self.poly - rhs.clone())
    }
}

impl Sub<PallasPoly> for &Poly {
    type Output = Poly;

    fn sub(self, rhs: PallasPoly) -> Self::Output {
        Poly::from(self.poly.clone() - rhs)
    }
}

impl Sub<&PallasPoly> for &Poly {
    type Output = Poly;

    fn sub(self, rhs: &PallasPoly) -> Self::Output {
        Poly::from(self.poly.clone() - rhs.clone())
    }
}

impl Sub<Poly> for PallasPoly {
    type Output = Self;

    fn sub(self, rhs: Poly) -> Self::Output {
        self - rhs.poly
    }
}

impl Sub<&Poly> for PallasPoly {
    type Output = PallasPoly;

    fn sub(self, rhs: &Poly) -> Self::Output {
        self - rhs.poly.clone()
    }
}

impl Sub<Poly> for &PallasPoly {
    type Output = PallasPoly;

    fn sub(self, rhs: Poly) -> Self::Output {
        self.clone() - rhs.poly
    }
}

impl Sub<&Poly> for &PallasPoly {
    type Output = PallasPoly;

    fn sub(self, rhs: &Poly) -> Self::Output {
        self.clone() - rhs.poly.clone()
    }
}

// Mul -----------------------------------------------------

impl Mul<PallasPoly> for Poly {
    type Output = Self;

    fn mul(self, rhs: PallasPoly) -> Self::Output {
        Poly::from(self.poly * rhs)
    }
}

impl Mul<&PallasPoly> for Poly {
    type Output = Self;

    fn mul(self, rhs: &PallasPoly) -> Self::Output {
        Poly::from(self.poly * rhs.clone())
    }
}

impl Mul<PallasPoly> for &Poly {
    type Output = Poly;

    fn mul(self, rhs: PallasPoly) -> Self::Output {
        Poly::from(self.poly.clone() * rhs)
    }
}

impl Mul<&PallasPoly> for &Poly {
    type Output = Poly;

    fn mul(self, rhs: &PallasPoly) -> Self::Output {
        Poly::from(self.poly.clone() * rhs.clone())
    }
}

impl Mul<Poly> for PallasPoly {
    type Output = Self;

    fn mul(self, rhs: Poly) -> Self::Output {
        self * rhs.poly
    }
}

impl Mul<&Poly> for PallasPoly {
    type Output = PallasPoly;

    fn mul(self, rhs: &Poly) -> Self::Output {
        self * rhs.poly.clone()
    }
}

impl Mul<Poly> for &PallasPoly {
    type Output = PallasPoly;

    fn mul(self, rhs: Poly) -> Self::Output {
        self.clone() * rhs.poly
    }
}

impl Mul<&Poly> for &PallasPoly {
    type Output = PallasPoly;

    fn mul(self, rhs: &Poly) -> Self::Output {
        self.clone() * rhs.poly.clone()
    }
}

// Div -----------------------------------------------------

impl Div<PallasPoly> for Poly {
    type Output = Self;

    fn div(self, rhs: PallasPoly) -> Self::Output {
        Poly::from(self.poly / rhs)
    }
}

impl Div<&PallasPoly> for Poly {
    type Output = Self;

    fn div(self, rhs: &PallasPoly) -> Self::Output {
        Poly::from(self.poly / rhs.clone())
    }
}

impl Div<PallasPoly> for &Poly {
    type Output = Poly;

    fn div(self, rhs: PallasPoly) -> Self::Output {
        Poly::from(self.poly.clone() / rhs)
    }
}

impl Div<&PallasPoly> for &Poly {
    type Output = Poly;

    fn div(self, rhs: &PallasPoly) -> Self::Output {
        Poly::from(self.poly.clone() / rhs.clone())
    }
}

impl Div<Poly> for PallasPoly {
    type Output = Self;

    fn div(self, rhs: Poly) -> Self::Output {
        self / rhs.poly
    }
}

impl Div<&Poly> for PallasPoly {
    type Output = PallasPoly;

    fn div(self, rhs: &Poly) -> Self::Output {
        self / rhs.poly.clone()
    }
}

impl Div<Poly> for &PallasPoly {
    type Output = PallasPoly;

    fn div(self, rhs: Poly) -> Self::Output {
        self.clone() / rhs.poly
    }
}

impl Div<&Poly> for &PallasPoly {
    type Output = PallasPoly;

    fn div(self, rhs: &Poly) -> Self::Output {
        self.clone() / rhs.poly.clone()
    }
}

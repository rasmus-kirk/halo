mod op_i32;
mod op_i64;
mod op_pallas_scalar;
mod op_poly;
mod op_scalar;
mod op_u32;
mod op_u64;
mod op_usize;

use ark_poly::{DenseUVPolynomial, Polynomial};
use halo_accumulation::{
    group::PallasPoly,
    pcdl::{self, EvalProof},
};
use rand::Rng;

use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::curve::{Point, Scalar};

/// f ∈ 𝔽ₚᵈ[X]
/// Polynomial f of degree d over the field 𝔽ₚ
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Poly {
    pub(crate) poly: PallasPoly,
    eval_cache: Option<Vec<Scalar>>,
}

impl Poly {
    pub fn new(poly: PallasPoly) -> Self {
        Poly {
            poly,
            eval_cache: None,
        }
    }

    pub fn new_cache(poly: PallasPoly, eval_cache: Vec<Scalar>) -> Self {
        Poly {
            poly,
            eval_cache: Some(eval_cache),
        }
    }

    pub fn cache_len(&self) -> Option<usize> {
        self.eval_cache.as_ref().map(|v| v.len())
    }

    pub fn cache(&self, i: u64) -> Option<Scalar> {
        if let Some(len) = self.cache_len() {
            if i as usize >= len {
                return None;
            }
            self.eval_cache.as_ref().map(|v| v[i as usize])
        } else {
            None
        }
    }

    pub fn degree(&self) -> u64 {
        self.poly.degree() as u64
    }

    pub fn evaluate(&self, x: &Scalar) -> Scalar {
        self.poly.evaluate(&x.into()).into()
    }

    pub fn evaluate_many<const N: usize>(fs: &[Poly; N], x: &Scalar) -> [Scalar; N] {
        let mut ys = [Scalar::ZERO; N];
        for (i, f) in fs.iter().enumerate() {
            ys[i] = f.evaluate(x);
        }
        ys
    }

    pub fn scalar_vec_to_poly(points: Vec<Scalar>) -> Poly {
        let coeffs = points.iter().map(|y| y.into()).collect();
        PallasPoly::from_coefficients_vec(coeffs).into()
    }

    pub fn commit(&self) -> Point {
        let mut d = self.degree().next_power_of_two() - 1;
        if self.degree() >= d {
            d += 2;
            d = d.next_power_of_two() - 1;
        }
        Point::new_d(d, pcdl::commit(&self.poly, d as usize, None))
    }

    pub fn commit_many<const N: usize>(fs: &[Poly; N]) -> [Point; N] {
        let mut comms = [Point::default(); N];
        for (i, f) in fs.iter().enumerate() {
            comms[i] = f.commit();
        }
        comms
    }

    pub fn open<R: Rng>(&self, rng: &mut R, commit: &Point, ch: &Scalar) -> EvalProof {
        pcdl::open(
            rng,
            self.poly.clone(),
            commit.into(),
            commit.into(),
            &ch.into(),
            None,
        )
    }
}

// Negate -----------------------------------------------------

impl Neg for Poly {
    type Output = Poly;

    fn neg(self) -> Self::Output {
        Poly::from(-self.poly)
    }
}

// Add -----------------------------------------------------

impl Add for Poly {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Poly::new(self.poly + other.poly)
    }
}
impl Add for &Poly {
    type Output = Poly;

    fn add(self, other: Self) -> Self::Output {
        Poly::new(&self.poly + &other.poly)
    }
}

impl Add<&Poly> for Poly {
    type Output = Poly;

    fn add(self, other: &Poly) -> Self::Output {
        Poly::new(self.poly + &other.poly)
    }
}

impl Add<Poly> for &Poly {
    type Output = Poly;

    fn add(self, other: Poly) -> Self::Output {
        Poly::new(&self.poly + other.poly)
    }
}

// Sub -----------------------------------------------------

impl Sub for Poly {
    type Output = Poly;

    fn sub(self, other: Poly) -> Self::Output {
        Poly::new(self.poly - other.poly)
    }
}

impl Sub for &Poly {
    type Output = Poly;

    fn sub(self, other: Self) -> Self::Output {
        Poly::new(&self.poly - &other.poly)
    }
}

impl Sub<&Poly> for Poly {
    type Output = Poly;

    fn sub(self, other: &Poly) -> Self::Output {
        Poly::new(self.poly - &other.poly)
    }
}

impl Sub<Poly> for &Poly {
    type Output = Poly;

    fn sub(self, other: Poly) -> Self::Output {
        Poly::new(&self.poly - other.poly)
    }
}

// Mul -----------------------------------------------------

impl Mul for Poly {
    type Output = Poly;

    fn mul(self, other: Poly) -> Self::Output {
        Poly::new(self.poly * other.poly)
    }
}

impl Mul for &Poly {
    type Output = Poly;

    fn mul(self, other: Self) -> Self::Output {
        Poly::new(&self.poly * &other.poly)
    }
}

impl Mul<&Poly> for Poly {
    type Output = Poly;

    fn mul(self, other: &Poly) -> Self::Output {
        Poly::new(self.poly * &other.poly)
    }
}

impl Mul<Poly> for &Poly {
    type Output = Poly;

    fn mul(self, other: Poly) -> Self::Output {
        Poly::new(&self.poly * other.poly)
    }
}

// Div -----------------------------------------------------

impl Div for Poly {
    type Output = Poly;

    fn div(self, other: Poly) -> Self::Output {
        Poly::new(self.poly / other.poly)
    }
}

impl Div for &Poly {
    type Output = Poly;

    fn div(self, other: Self) -> Self::Output {
        Poly::new(&self.poly / &other.poly)
    }
}

impl Div<&Poly> for Poly {
    type Output = Poly;

    fn div(self, other: &Poly) -> Self::Output {
        Poly::new(self.poly / &other.poly)
    }
}

impl Div<Poly> for &Poly {
    type Output = Poly;

    fn div(self, other: Poly) -> Self::Output {
        Poly::new(&self.poly / other.poly)
    }
}

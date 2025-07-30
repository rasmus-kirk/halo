use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;
use ark_ff::{BigInt, FftField, PrimeField};
use ark_poly::{EvaluationDomain, Evaluations, GeneralEvaluationDomain};
use ark_std::{One, Zero};
use rayon::prelude::*;

use crate::{PastaConfig, Poly, Scalar};

// pub type Evals<P> = Evaluations<Scalar<P>, GeneralEvaluationDomain<Scalar<P>>>;
pub type Domain<P> = GeneralEvaluationDomain<Scalar<P>>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evals<P: PastaConfig> {
    pub evals: Evaluations<Scalar<P>, Domain<P>>,
}

/// A thin wrapper for ark_poly Evaluations
impl<P: PastaConfig> Evals<P> {
    pub fn from_vec_and_domain(mut evals: Vec<Scalar<P>>, domain: Domain<P>) -> Self {
        // shift right
        let last = evals.pop().unwrap();
        evals.insert(0, last);
        let evals = Evaluations::from_vec_and_domain(evals, domain);
        Self { evals }
    }

    pub fn new(evals: Evaluations<Scalar<P>>) -> Self {
        Self { evals }
    }

    pub fn from_poly_ref(poly: &Poly<P>, domain: Domain<P>) -> Self {
        let evals = poly.evaluate_over_domain_by_ref(domain);
        Self::new(evals)
    }

    pub fn from_poly(poly: Poly<P>, domain: Domain<P>) -> Self {
        let evals = poly.evaluate_over_domain(domain);
        Self::new(evals)
    }

    pub fn one(domain: Domain<P>) -> Self {
        Evals::<P>::from_vec_and_domain(vec![Scalar::<P>::one(); domain.size()], domain)
    }

    pub fn divide_by_vanishing(mut self, vanishing_domain: Domain<P>) -> Self {
        for i in 0..self.evals.evals.len() {
            let x = self.domain().element(i);
            let z_h_x = vanishing_domain.evaluate_vanishing_polynomial(x);
            let inv = z_h_x.inverse().unwrap();
            self.evals.evals[i] *= inv;
        }
        self
    }

    pub fn extend(&self, new_domain_size: usize) -> Evals<P> {
        let mut vec = self.evals.evals.clone();
        vec.resize(new_domain_size, Scalar::<P>::zero());
        Evals::<P>::from_vec_and_domain(vec, Domain::<P>::new(new_domain_size).unwrap())
    }

    #[inline]
    pub fn scale_in_place(mut self, other: P::ScalarField) -> Evals<P> {
        self.evals.evals.par_iter_mut().for_each(|x| *x *= other);
        self
    }

    #[inline]
    pub fn scale(&self, other: P::ScalarField) -> Evals<P> {
        let mut evals = self.clone();
        evals.evals.evals.par_iter_mut().for_each(|x| *x *= other);
        evals
    }

    #[inline]
    pub fn add_scalar(mut self, other: P::ScalarField) -> Evals<P> {
        self.evals.evals.par_iter_mut().for_each(|x| *x += other);
        self
    }

    pub fn omega(&self) -> Scalar<P> {
        self.domain().element(1)
    }

    pub fn interpolate(self) -> Poly<P> {
        self.evals.interpolate()
    }

    pub fn interpolate_by_ref(&self) -> Poly<P> {
        self.evals.interpolate_by_ref()
    }

    pub fn domain(&self) -> Domain<P> {
        self.evals.domain()
    }
}

// ---------- Index ---------- //

impl<P: PastaConfig> Index<usize> for Evals<P> {
    type Output = Scalar<P>;
    fn index(&self, index: usize) -> &Scalar<P> {
        &self.evals[index]
    }
}

// ---------- Mul ---------- //

impl<P: PastaConfig> Mul<Evals<P>> for Evals<P> {
    type Output = Evals<P>;
    #[inline]
    fn mul(mut self, other: Evals<P>) -> Evals<P> {
        self *= &other;
        self
    }
}

impl<P: PastaConfig> Mul<Evals<P>> for &Evals<P> {
    type Output = Evals<P>;

    #[inline]
    fn mul(self, mut other: Evals<P>) -> Evals<P> {
        other *= self;
        other
    }
}

impl<P: PastaConfig> Mul<&Evals<P>> for Evals<P> {
    type Output = Evals<P>;

    #[inline]
    fn mul(mut self, other: &Evals<P>) -> Evals<P> {
        self *= &other;
        self
    }
}

impl<'a, 'b, P: PastaConfig> Mul<&'a Evals<P>> for &'b Evals<P> {
    type Output = Evals<P>;

    #[inline]
    fn mul(self, other: &'a Evals<P>) -> Evals<P> {
        Evals {
            evals: &self.evals * &other.evals,
        }
    }
}

// ---------- MulAssign ---------- //

impl<P: PastaConfig> MulAssign<&Evals<P>> for Evals<P> {
    #[inline]
    fn mul_assign(&mut self, other: &Evals<P>) {
        self.evals *= &other.evals;
    }
}

// ---------- Add ---------- //

impl<P: PastaConfig> Add for Evals<P> {
    type Output = Evals<P>;

    #[inline]
    fn add(mut self, other: Evals<P>) -> Evals<P> {
        self += &other;
        self
    }
}

impl<'a, P: PastaConfig> Add<&'a Evals<P>> for Evals<P> {
    type Output = Evals<P>;

    #[inline]
    fn add(mut self, other: &'a Evals<P>) -> Evals<P> {
        self += &other;
        self
    }
}

impl<'a, P: PastaConfig> Add<Evals<P>> for &'a Evals<P> {
    type Output = Evals<P>;

    #[inline]
    fn add(self, mut other: Evals<P>) -> Evals<P> {
        other += self;
        other
    }
}

impl<'a, 'b, P: PastaConfig> Add<&'a Evals<P>> for &'b Evals<P> {
    type Output = Evals<P>;

    #[inline]
    fn add(self, other: &'a Evals<P>) -> Evals<P> {
        Evals {
            evals: &self.evals + &other.evals,
        }
    }
}

// ---------- AddAssign ---------- //

impl<'a, P: PastaConfig> AddAssign<&'a Evals<P>> for Evals<P> {
    #[inline]
    fn add_assign(&mut self, other: &'a Evals<P>) {
        self.evals += &other.evals;
    }
}

impl<'a, 'b, P: PastaConfig> Sub<&'a Evals<P>> for &'b Evals<P> {
    type Output = Evals<P>;

    #[inline]
    fn sub(self, other: &'a Evals<P>) -> Evals<P> {
        Evals {
            evals: &self.evals - &other.evals,
        }
    }
}

impl<'a, P: PastaConfig> SubAssign<&'a Evals<P>> for Evals<P> {
    #[inline]
    fn sub_assign(&mut self, other: &'a Evals<P>) {
        self.evals -= &other.evals;
    }
}

impl<P: PastaConfig> Div for Evals<P> {
    type Output = Evals<P>;

    #[inline]
    fn div(mut self, other: Evals<P>) -> Evals<P> {
        self /= &other;
        self
    }
}

impl<'a, P: PastaConfig> Div<&'a Evals<P>> for Evals<P> {
    type Output = Evals<P>;

    #[inline]
    fn div(mut self, other: &'a Evals<P>) -> Evals<P> {
        self /= &other;
        self
    }
}

impl<'a, P: PastaConfig> Div<Evals<P>> for &'a Evals<P> {
    type Output = Evals<P>;

    #[inline]
    fn div(self, mut other: Evals<P>) -> Evals<P> {
        other /= self;
        other
    }
}

impl<'a, 'b, P: PastaConfig> Div<&'a Evals<P>> for &'b Evals<P> {
    type Output = Evals<P>;

    #[inline]
    fn div(self, other: &'a Evals<P>) -> Evals<P> {
        Evals {
            evals: &self.evals / &other.evals,
        }
    }
}

impl<'a, P: PastaConfig> DivAssign<&'a Evals<P>> for Evals<P> {
    #[inline]
    fn div_assign(&mut self, other: &'a Evals<P>) {
        self.evals /= &other.evals;
    }
}

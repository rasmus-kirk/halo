#![allow(type_alias_bounds)]
#![allow(dead_code)]

use ark_ec::short_weierstrass::{Affine as AffineEC, Projective, SWCurveConfig};
use ark_poly::{
    univariate::DensePolynomial, EvaluationDomain, Evaluations, GeneralEvaluationDomain,
};
use std::ops::{Index, Mul};

pub type Scalar<P: SWCurveConfig> = P::ScalarField;
pub type Poly<P: SWCurveConfig> = DensePolynomial<P::ScalarField>;
pub type Point<P: SWCurveConfig> = Projective<P>;
pub type Affine<P: SWCurveConfig> = AffineEC<P>;

#[derive(Debug, PartialEq, Eq)]
pub struct Evals<P: SWCurveConfig> {
    evals: Vec<P::ScalarField>,
    domain: GeneralEvaluationDomain<P::ScalarField>,
}

impl<P: SWCurveConfig> Clone for Evals<P> {
    fn clone(&self) -> Self {
        Self {
            evals: self.evals.clone(),
            domain: self.domain,
        }
    }
}

impl<P: SWCurveConfig> Evals<P> {
    pub fn new(
        evals: Vec<P::ScalarField>,
        domain: GeneralEvaluationDomain<P::ScalarField>,
    ) -> Self {
        Self { evals, domain }
    }

    pub fn interpolate(self) -> Poly<P> {
        Evaluations::from_vec_and_domain(self.evals, self.domain).interpolate()
    }

    pub fn vec(self) -> Vec<P::ScalarField> {
        self.evals
    }

    pub fn last(&self) -> &P::ScalarField {
        self.evals.last().unwrap()
    }
}

impl<P: SWCurveConfig> Mul for Evals<P> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let evals = self
            .domain
            .mul_polynomials_in_evaluation_domain(&self.evals, &rhs.evals);
        Self {
            evals,
            domain: self.domain,
        }
    }
}

impl<P: SWCurveConfig> Index<usize> for Evals<P> {
    type Output = P::ScalarField;

    fn index(&self, index: usize) -> &Self::Output {
        &self.evals[index]
    }
}

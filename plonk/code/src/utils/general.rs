#![allow(type_alias_bounds)]
#![allow(dead_code)]

use ark_ec::short_weierstrass::{Affine as AffineEC, Projective, SWCurveConfig};

use ark_poly::{univariate::DensePolynomial, Evaluations};

pub type Scalar<P: SWCurveConfig> = P::ScalarField;
pub type Poly<P: SWCurveConfig> = DensePolynomial<P::ScalarField>;
pub type Evals<P: SWCurveConfig> = Evaluations<P::ScalarField>;
pub type Point<P: SWCurveConfig> = Projective<P>;
pub type Affine<P: SWCurveConfig> = AffineEC<P>;

#![allow(type_alias_bounds)]
#![allow(dead_code)]
use ark_ec::short_weierstrass::{Affine as AffineEC, Projective, SWCurveConfig};

pub type Scalar<P: SWCurveConfig> = P::ScalarField;
pub type Point<P: SWCurveConfig> = Projective<P>;
pub type Affine<P: SWCurveConfig> = AffineEC<P>;

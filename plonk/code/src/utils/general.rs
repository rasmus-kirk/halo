use std::ops::{Add, Mul};

use ark_ec::short_weierstrass::{Projective, SWCurveConfig};

use ark_ff::Field;
use ark_poly::{univariate::DensePolynomial, Evaluations};

pub type Scalar<P: SWCurveConfig> = P::ScalarField;
pub type Poly<P: SWCurveConfig> = DensePolynomial<P::ScalarField>;
pub type Evals<P: SWCurveConfig> = Evaluations<P::ScalarField>;
pub type Point<P: SWCurveConfig> = Projective<P>;

// /// Y = x₀y₀ + x₁y₁ + x₂y₂ + ...
// pub fn dot<I, T, U>(xs: I, ys: I) -> U
// where
//     I: IntoIterator<Item = T>,
//     T: Mul<T, Output = U>,
//     U: Add<U, Output = U>,
// {
//     xs.into_iter()
//         .zip(ys)
//         .map(|(x, y)| x * y)
//         .reduce(|acc, x| acc + x)
//         .unwrap()
// }

/// p₀ + a₁p₁ + a₂p₂ + ...
pub fn geometric<F, T, U, I>(one: F, a: F, ps: I) -> U
where
    I: IntoIterator<Item = T>,
    F: Mul<F, Output = F> + Copy,
    U: Default + Add<U, Output = U>,
    T: Mul<F, Output = U>,
{
    ps.into_iter()
        .fold((U::default(), one), |(acc, power), p| {
            (acc + (p * power), power * a)
        })
        .0
}

pub fn flat_geometric<const M: usize, F, T, U, I>(one: F, a: F, pss: [I; M]) -> U
where
    I: IntoIterator<Item = T>,
    F: Mul<F, Output = F> + Copy,
    U: Default + Add<U, Output = U>,
    T: Mul<F, Output = U>,
{
    geometric(one, a, pss.into_iter().flat_map(|ps| ps.into_iter()))
}

pub fn geometric_fp<P: SWCurveConfig, T, U, I>(a: Scalar<P>, ps: I) -> U
where
    I: IntoIterator<Item = T>,
    U: Default + Add<U, Output = U>,
    T: Mul<Scalar<P>, Output = U>,
{
    geometric(Scalar::<P>::ONE, a, ps)
}

pub fn flat_geometric_fp<const M: usize, P: SWCurveConfig, T, U, I>(a: Scalar<P>, pss: [I; M]) -> U
where
    I: IntoIterator<Item = T>,
    U: Default + Add<U, Output = U>,
    T: Mul<Scalar<P>, Output = U>,
{
    flat_geometric(Scalar::<P>::ONE, a, pss)
}

// /// f(x₀,y₀) f(x₁,y₁) f(x₂,y₂) ...
// pub fn prod_zip_map<I, J, T, U, V>(xs: I, ys: J, f: impl Fn(T, U) -> V) -> V
// where
//     V: Mul<V, Output = V>,
//     I: IntoIterator<Item = T>,
//     J: IntoIterator<Item = U>,
// {
//     xs.into_iter()
//         .zip(ys)
//         .map(move |(t, u)| f(t, u))
//         .reduce(|acc, x| acc * x)
//         .unwrap()
// }

use super::Terms;
use crate::utils::{misc::EnumIter, Scalar};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;
use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, Mul, Sub},
};

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

/// aQₗ + bQᵣ + cQₒ + abQₘ + Q꜀ + PI
pub fn plonk_eqn<U, T, I1, I2>(ws: I1, qs: I2, pip: T) -> U
where
    I1: IntoIterator<Item = T>,
    I2: IntoIterator<Item = T>,
    U: Add<T, Output = U> + Add<U, Output = U> + Mul<T, Output = U> + Default,
    T: Mul<T, Output = U> + Mul<U, Output = U> + Debug + Copy,
{
    let [a, b, c] = ws.into_iter().collect::<Vec<T>>().try_into().unwrap();
    let [ql, qr, qo, qm, qc, _, _] = qs.into_iter().collect::<Vec<T>>().try_into().unwrap();

    (a * ql) + (b * qr) + (c * qo) + (a * b * qm) + qc + pip
}

pub fn plonk_eqn_str(terms: [String; Terms::COUNT]) -> String {
    let [a, b, c, ql, qr, qo, qm, qc, _, _, pi] = terms;
    format!(
        "{} × {} + {} × {} + {} × {} + {} × {} × {} + {} + {}",
        a, ql, b, qr, c, qo, a, b, qm, qc, pi,
    )
}

/// a + ζb + ζ²c + ζ³j
pub fn plookup_compress<F, U, T>(one: F, zeta: F, a: T, b: T, c: T, j: T) -> U
where
    F: Mul<F, Output = F> + Copy,
    U: Add<U, Output = U> + Default,
    T: Mul<F, Output = U>,
{
    geometric(one, zeta, [a, b, c, j])
}

/// aQₗ + bQᵣ + cQₒ + abQₘ + Q꜀ + PI + Qₖ(a + ζb + ζ²c + ζ³j - f)
pub fn plonkup_eqn<F, T, U, I1, I2>(one: F, zeta: F, ws: I1, qs: I2, pip: T, f: T) -> U
where
    F: Mul<F, Output = F> + Copy,
    I1: IntoIterator<Item = T> + Clone,
    I2: IntoIterator<Item = T> + Clone,
    U: Add<T, Output = U> + Add<U, Output = U> + Sub<T, Output = U> + Mul<T, Output = U> + Default,
    T: Mul<F, Output = U>
        + Mul<F, Output = U>
        + Mul<T, Output = U>
        + Mul<U, Output = U>
        + Debug
        + Copy,
{
    let eqn1 = plonk_eqn(ws.clone(), qs.clone(), pip);
    let [a, b, c] = ws.into_iter().collect::<Vec<_>>().try_into().unwrap();
    let [_, _, _, _, _, qk, j] = qs.into_iter().collect::<Vec<_>>().try_into().unwrap();
    eqn1 + qk * (plookup_compress(one, zeta, a, b, c, j) - f)
}

/// a + βb + γ
pub fn copy_constraint_term<F, T, U, Func>(into: Func, beta: F, gamma: F) -> impl Fn(T, T) -> U
where
    Func: Fn(F) -> U,
    F: Mul<F, Output = F> + Copy,
    T: Mul<F, Output = U> + Add<U, Output = U>,
    U: Add<U, Output = U>,
{
    move |a: T, b: T| a + (b * beta) + into(gamma)
}

/// ε(1 + δ) + a + δb
pub fn plookup_term<F, T, U, Func>(into: Func, e1d: F, delta: F) -> impl Fn(T, T) -> U
where
    Func: Fn(F) -> U,
    F: Add<F, Output = F> + Mul<F, Output = F> + Copy,
    T: Mul<F, Output = U>,
    U: Add<T, Output = U> + Add<U, Output = U>,
{
    move |a: T, b: T| into(e1d) + a + (b * delta)
}

/// L₁ (Z - 1)
pub fn grand_product1<F, T, U>(one: F, z: T, l1: T) -> U
where
    T: Sub<F, Output = U> + Mul<U, Output = U>,
{
    l1 * (z - one)
}

/// Z f' - g' Z'
pub fn grand_product2<T, U>(z: T, zf: T, zg: T, z_bar: T) -> U
where
    T: Mul<T, Output = U>,
    U: Sub<U, Output = U>,
{
    (z * zf) - (zg * z_bar)
}

pub struct EqnsF<P: SWCurveConfig>(PhantomData<P>);
impl<P: SWCurveConfig> EqnsF<P> {
    pub fn geometric_fp<T, U, I>(a: Scalar<P>, ps: I) -> U
    where
        I: IntoIterator<Item = T>,
        U: Default + Add<U, Output = U>,
        T: Mul<Scalar<P>, Output = U>,
    {
        geometric(Scalar::<P>::ONE, a, ps)
    }

    pub fn flat_geometric_fp<const M: usize, T, U, I>(a: Scalar<P>, pss: [I; M]) -> U
    where
        I: IntoIterator<Item = T>,
        U: Default + Add<U, Output = U>,
        T: Mul<Scalar<P>, Output = U>,
    {
        flat_geometric(Scalar::<P>::ONE, a, pss)
    }

    pub fn plookup_compress<U, T>(zeta: P::ScalarField, a: T, b: T, c: T, j: T) -> U
    where
        U: Add<U, Output = U> + Default,
        T: Mul<P::ScalarField, Output = U>,
    {
        plookup_compress(P::ScalarField::ONE, zeta, a, b, c, j)
    }

    pub fn plonkup_eqn<T, U, I1, I2>(zeta: Scalar<P>, ws: I1, qs: I2, pip: T, f: T) -> U
    where
        I1: IntoIterator<Item = T> + Clone,
        I2: IntoIterator<Item = T> + Clone,
        U: Add<T, Output = U>
            + Add<U, Output = U>
            + Sub<T, Output = U>
            + Mul<T, Output = U>
            + Default,
        T: Mul<Scalar<P>, Output = U>
            + Mul<Scalar<P>, Output = U>
            + Mul<T, Output = U>
            + Mul<U, Output = U>
            + Debug
            + Copy,
    {
        plonkup_eqn(Scalar::<P>::ONE, zeta, ws, qs, pip, f)
    }

    pub fn grand_product1<T, U>(z: T, l1: T) -> U
    where
        T: Sub<Scalar<P>, Output = U> + Mul<U, Output = U>,
    {
        grand_product1(Scalar::<P>::ONE, z, l1)
    }
}

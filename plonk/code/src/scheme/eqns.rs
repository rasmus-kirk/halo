use super::Terms;
use crate::utils::{geometric, misc::EnumIter, Scalar};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;
use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

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

pub fn plookup_compress_fp<U, T, P: SWCurveConfig>(
    zeta: P::ScalarField,
    a: T,
    b: T,
    c: T,
    j: T,
) -> U
where
    U: Add<U, Output = U> + Default,
    T: Mul<P::ScalarField, Output = U>,
{
    plookup_compress(P::ScalarField::ONE, zeta, a, b, c, j)
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

pub fn plonkup_eqn_fp<P: SWCurveConfig, T, U, I1, I2>(
    zeta: Scalar<P>,
    ws: I1,
    qs: I2,
    pip: T,
    f: T,
) -> U
where
    I1: IntoIterator<Item = T> + Clone,
    I2: IntoIterator<Item = T> + Clone,
    U: Add<T, Output = U> + Add<U, Output = U> + Sub<T, Output = U> + Mul<T, Output = U> + Default,
    T: Mul<Scalar<P>, Output = U>
        + Mul<Scalar<P>, Output = U>
        + Mul<T, Output = U>
        + Mul<U, Output = U>
        + Debug
        + Copy,
{
    plonkup_eqn(Scalar::<P>::ONE, zeta, ws, qs, pip, f)
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

pub fn grand_product1_fp<P: SWCurveConfig, T, U>(z: T, l1: T) -> U
where
    T: Sub<Scalar<P>, Output = U> + Mul<U, Output = U>,
{
    grand_product1(Scalar::<P>::ONE, z, l1)
}

/// Z f' - g' Z'
pub fn grand_product2<T, U>(z: T, zf: T, zg: T, z_bar: T) -> U
where
    T: Mul<T, Output = U>,
    U: Sub<U, Output = U>,
{
    (z * zf) - (zg * z_bar)
}

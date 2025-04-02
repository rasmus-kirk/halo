use super::Terms;
use crate::utils::{geometric, misc::EnumIter};

use ark_ff::{Field, Fp, FpConfig};
use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

/// aQₗ + bQᵣ + cQₒ + abQₘ + Q꜀ + PI
pub fn plonk_eqn<P, T, I1, I2>(ws: I1, qs: I2, pip: T) -> P
where
    I1: IntoIterator<Item = T>,
    I2: IntoIterator<Item = T>,
    P: Add<T, Output = P> + Add<P, Output = P> + Mul<T, Output = P> + Default,
    T: Mul<T, Output = P> + Mul<P, Output = P> + Debug + Copy,
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
pub fn plookup_compress<F, P, T>(one: F, zeta: F, a: T, b: T, c: T, j: T) -> P
where
    F: Mul<F, Output = F> + Copy,
    P: Add<P, Output = P> + Default,
    T: Mul<F, Output = P>,
{
    geometric(one, zeta, [a, b, c, j])
}

pub fn plookup_compress_fp<const N: usize, C, P, T>(zeta: Fp<C, N>, a: T, b: T, c: T, j: T) -> P
where
    C: FpConfig<N>,
    P: Add<P, Output = P> + Default,
    T: Mul<Fp<C, N>, Output = P>,
{
    plookup_compress(Fp::ONE, zeta, a, b, c, j)
}

/// aQₗ + bQᵣ + cQₒ + abQₘ + Q꜀ + PI + Qₖ(a + ζb + ζ²c + ζ³j - f)
pub fn plonkup_eqn<F, T, P, I1, I2>(one: F, zeta: F, ws: I1, qs: I2, pip: T, f: T) -> P
where
    F: Mul<F, Output = F> + Copy,
    I1: IntoIterator<Item = T> + Clone,
    I2: IntoIterator<Item = T> + Clone,
    P: Add<T, Output = P> + Add<P, Output = P> + Sub<T, Output = P> + Mul<T, Output = P> + Default,
    T: Mul<F, Output = P>
        + Mul<F, Output = P>
        + Mul<T, Output = P>
        + Mul<P, Output = P>
        + Debug
        + Copy,
{
    let eqn1 = plonk_eqn(ws.clone(), qs.clone(), pip);
    let [a, b, c] = ws.into_iter().collect::<Vec<_>>().try_into().unwrap();
    let [_, _, _, _, _, qk, j] = qs.into_iter().collect::<Vec<_>>().try_into().unwrap();
    eqn1 + qk * (plookup_compress(one, zeta, a, b, c, j) - f)
}

pub fn plonkup_eqn_fp<const N: usize, C: FpConfig<N>, T, P, I1, I2>(
    zeta: Fp<C, N>,
    ws: I1,
    qs: I2,
    pip: T,
    f: T,
) -> P
where
    I1: IntoIterator<Item = T> + Clone,
    I2: IntoIterator<Item = T> + Clone,
    P: Add<T, Output = P> + Add<P, Output = P> + Sub<T, Output = P> + Mul<T, Output = P> + Default,
    T: Mul<Fp<C, N>, Output = P>
        + Mul<Fp<C, N>, Output = P>
        + Mul<T, Output = P>
        + Mul<P, Output = P>
        + Debug
        + Copy,
{
    plonkup_eqn(Fp::ONE, zeta, ws, qs, pip, f)
}

/// a + βb + γ
pub fn copy_constraint_term<F, T, P, Func>(into: Func, beta: F, gamma: F) -> impl Fn(T, T) -> P
where
    Func: Fn(F) -> P,
    F: Mul<F, Output = F> + Copy,
    T: Mul<F, Output = P> + Add<P, Output = P>,
    P: Add<P, Output = P>,
{
    move |a: T, b: T| a + (b * beta) + into(gamma)
}

/// ε(1 + δ) + a + δb
pub fn plookup_term<F, T, P, Func>(one: F, into: Func, epsilon: F, delta: F) -> impl Fn(T, T) -> P
where
    Func: Fn(F) -> P,
    F: Add<F, Output = F> + Mul<F, Output = F> + Copy,
    T: Mul<F, Output = P>,
    P: Add<T, Output = P> + Add<P, Output = P>,
{
    move |a: T, b: T| into(epsilon * (one + delta)) + a + (b * delta)
}

pub fn plookup_term_fp<const N: usize, C, T, P, Func>(
    into: Func,
    epsilon: Fp<C, N>,
    delta: Fp<C, N>,
) -> impl Fn(T, T) -> P
where
    Func: Fn(Fp<C, N>) -> P,
    C: FpConfig<N>,
    T: Mul<Fp<C, N>, Output = P>,
    P: Add<T, Output = P> + Add<P, Output = P>,
{
    plookup_term(Fp::ONE, into, epsilon, delta)
}

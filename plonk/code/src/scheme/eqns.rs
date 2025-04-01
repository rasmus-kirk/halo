use super::Terms;
use crate::utils::geometric;

use ark_ff::{Field, Fp, FpConfig};
use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

/// aQₗ + bQᵣ + cQₒ + abQₘ + Q꜀ + PI
pub fn plonk_eqn<const N: usize, C: FpConfig<N>, P, T, I1, I2>(ws: I1, qs: I2, pip: T) -> P
where
    I1: IntoIterator<Item = T>,
    I2: IntoIterator<Item = T>,
    P: Default + Add<P, Output = P> + Add<T, Output = P> + Mul<T, Output = P>,
    T: Mul<Fp<C, N>, Output = P> + Mul<T, Output = P> + Mul<P, Output = P> + Debug + Copy,
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
pub fn plookup_compress<const N: usize, C, P, T>(zeta: Fp<C, N>, a: T, b: T, c: T, j: T) -> P
where
    C: FpConfig<N>,
    P: Default + Add<P, Output = P>,
    T: Mul<Fp<C, N>, Output = P>,
{
    geometric(zeta, [a, b, c, j])
}

/// aQₗ + bQᵣ + cQₒ + abQₘ + Q꜀ + PI + Qₖ(a + ζb + ζ²c + ζ³j - f)
pub fn plonkup_eqn<const N: usize, C: FpConfig<N>, T, P, I1, I2>(
    zeta: Fp<C, N>,
    ws: I1,
    qs: I2,
    pip: T,
    f: T,
) -> P
where
    I1: IntoIterator<Item = T> + Clone,
    I2: IntoIterator<Item = T> + Clone,
    P: Default + Add<P, Output = P> + Add<T, Output = P> + Mul<T, Output = P> + Sub<T, Output = P>,
    T: Mul<Fp<C, N>, Output = P>
        + Mul<Fp<C, N>, Output = P>
        + Mul<T, Output = P>
        + Mul<P, Output = P>
        + Debug
        + Copy,
{
    let eqn1 = plonk_eqn(ws.clone(), qs.clone(), pip);
    let [a, b, c] = ws.into_iter().collect::<Vec<_>>().try_into().unwrap();
    let [_, _, _, _, _, qk, j] = qs.into_iter().collect::<Vec<_>>().try_into().unwrap();
    eqn1 + qk * (plookup_compress(zeta, a, b, c, j) - f)
}

/// a + βb + γ
pub fn copy_constraint_term<const N: usize, C, T, P, F>(
    into: F,
    beta: Fp<C, N>,
    gamma: Fp<C, N>,
) -> impl Fn(T, T) -> P
where
    F: Fn(Fp<C, N>) -> P,
    C: FpConfig<N>,
    T: Mul<Fp<C, N>, Output = P> + Add<P, Output = P>,
    P: Add<P, Output = P>,
{
    move |a: T, b: T| a + (b * beta) + into(gamma)
}

/// ε(1 + δ) + a + δb
pub fn plookup_term<const N: usize, C, T, P, F>(
    into: F,
    epsilon: Fp<C, N>,
    delta: Fp<C, N>,
) -> impl Fn(T, T) -> P
where
    F: Fn(Fp<C, N>) -> P,
    C: FpConfig<N>,
    T: Mul<Fp<C, N>, Output = P>,
    P: Add<T, Output = P> + Add<P, Output = P>,
{
    move |a: T, b: T| into(epsilon * (Fp::ONE + delta)) + a + (b * delta)
}

use super::Terms;
use crate::utils::{dot, geometric};

use ark_ff::{Fp, FpConfig};
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

    dot([a, b, c], [ql, qr, qo]) + (a * b * qm) + qc + pip
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

// Qₖ(a + ζb + ζ²c + ζ³j - f)
pub fn plookup_eqn<const N: usize, C, P, T, I1, I2>(zeta: Fp<C, N>, ws: I1, qs: I2, f: T) -> P
where
    I1: IntoIterator<Item = T>,
    I2: IntoIterator<Item = T>,
    C: FpConfig<N>,
    P: Default + Add<P, Output = P> + Sub<T, Output = P>,
    T: Mul<Fp<C, N>, Output = P> + Mul<P, Output = P> + Debug,
{
    let [a, b, c] = ws.into_iter().collect::<Vec<_>>().try_into().unwrap();
    let [_, _, _, _, _, qk, j] = qs.into_iter().collect::<Vec<_>>().try_into().unwrap();
    qk * (plookup_compress(zeta, a, b, c, j) - f)
}

/// aQₗ + bQᵣ + cQₒ + abQₘ + Q꜀ + PI(X) + Qₖ(a + ζb + ζ²c + ζ³j - f)
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
    plonk_eqn(ws.clone(), qs.clone(), pip) + plookup_eqn(zeta, ws, qs, f)
}

// TODO copy constraint term, plookup constraint term, grand product f, grand product g

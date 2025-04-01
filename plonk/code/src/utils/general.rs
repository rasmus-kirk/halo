use std::ops::{Add, Mul};

use ark_ff::{Field, Fp, FpConfig};

// /// Y = x₀y₀ + x₁y₁ + x₂y₂ + ...
// pub fn dot<I, T, P>(xs: I, ys: I) -> P
// where
//     I: IntoIterator<Item = T>,
//     T: Mul<T, Output = P>,
//     P: Add<P, Output = P>,
// {
//     xs.into_iter()
//         .zip(ys)
//         .map(|(x, y)| x * y)
//         .reduce(|acc, x| acc + x)
//         .unwrap()
// }

/// p₀ + a₁p₁ + a₂p₂ + ...
pub fn geometric<F, T, P, I>(one: F, a: F, ps: I) -> P
where
    I: IntoIterator<Item = T>,
    F: Mul<F, Output = F> + Copy,
    P: Default + Add<P, Output = P>,
    T: Mul<F, Output = P>,
{
    ps.into_iter()
        .fold((P::default(), one), |(acc, power), p| {
            (acc + (p * power), power * a)
        })
        .0
}

pub fn flat_geometric<const M: usize, F, T, P, I>(one: F, a: F, pss: [I; M]) -> P
where
    I: IntoIterator<Item = T>,
    F: Mul<F, Output = F> + Copy,
    P: Default + Add<P, Output = P>,
    T: Mul<F, Output = P>,
{
    geometric(one, a, pss.into_iter().flat_map(|ps| ps.into_iter()))
}

pub fn geometric_fp<const N: usize, C, T, P, I>(a: Fp<C, N>, ps: I) -> P
where
    I: IntoIterator<Item = T>,
    C: FpConfig<N>,
    P: Default + Add<P, Output = P>,
    T: Mul<Fp<C, N>, Output = P>,
{
    geometric(Fp::ONE, a, ps)
}

pub fn flat_geometric_fp<const N: usize, const M: usize, C, T, P, I>(a: Fp<C, N>, pss: [I; M]) -> P
where
    I: IntoIterator<Item = T>,
    C: FpConfig<N>,
    P: Default + Add<P, Output = P>,
    T: Mul<Fp<C, N>, Output = P>,
{
    flat_geometric(Fp::ONE, a, pss)
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

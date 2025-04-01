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
pub fn geometric<const N: usize, C, T, P, I>(a: Fp<C, N>, ps: I) -> P
where
    I: IntoIterator<Item = T>,
    C: FpConfig<N>,
    P: Default + Add<P, Output = P>,
    T: Mul<Fp<C, N>, Output = P>,
{
    ps.into_iter()
        .fold((P::default(), Fp::ONE), |(acc, power), p| {
            (acc + (p * power), power * a)
        })
        .0
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

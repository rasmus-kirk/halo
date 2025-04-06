use std::marker::PhantomData;

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;

use crate::{
    utils::{Evals, Scalar},
    Coset,
};

pub struct GrandProduct<P: SWCurveConfig>(PhantomData<P>);

impl<P: SWCurveConfig> GrandProduct<P> {
    // Z(1) = 1
    // Z(ω) = 1
    // Z(ωⁱ) = Z(ωᶦ⁻¹) f'(ωᶦ⁻¹) / g'(ωᶦ⁻¹)
    pub fn evals<F, G>(h: &Coset<P>, f: F, g: G) -> Evals<P>
    where
        F: Fn(usize) -> Scalar<P>,
        G: Fn(usize) -> Scalar<P>,
    {
        let zf_vals: Vec<_> = h.iter_usize().map(|i| f(i)).collect();
        let mut zg_vals: Vec<_> = h.iter_usize().map(|i| g(i)).collect();
        Self::batch_inverse(&mut zg_vals);
        let ratios = zf_vals
            .into_iter()
            .zip(zg_vals)
            .map(|(num, den_inv)| num * den_inv);
        let points_rest = ratios
            .scan(Scalar::<P>::ONE, |acc, x| {
                *acc *= x;
                Some(*acc)
            })
            .collect::<Vec<_>>();
        let points = [Scalar::<P>::ONE; 2]
            .into_iter()
            .chain(points_rest.into_iter())
            .collect();
        Evals::<P>::from_vec_and_domain(points, h.domain)
    }

    // bᵢ = 1 / aᵢ
    // A = a₁ a₂ ...
    // bᵢ = Π(i ≠ j, aⱼ) A⁻¹
    fn batch_inverse(values: &mut [Scalar<P>]) {
        let n = values.len();
        let mut prod = vec![Scalar::<P>::ONE; n + 1];

        // Π(<= 1, aᵢ), Π(<= 2, aᵢ), ...
        for i in 1..n + 1 {
            prod[i] = prod[i - 1] * values[i - 1];
        }

        // A⁻¹
        let mut inv = prod.last().unwrap().inverse().unwrap();

        // bᵢ
        for i in (0..n).rev() {
            let tmp = values[i];
            values[i] = inv * prod[i];
            inv *= tmp;
        }
    }
}

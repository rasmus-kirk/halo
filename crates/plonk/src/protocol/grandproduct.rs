use crate::{
    utils::{Evals, Scalar},
    Coset,
};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;

use std::marker::PhantomData;

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
        let zf_vals: Vec<_> = h.iter_usize().map(f).collect();
        let mut zg_vals: Vec<_> = h.iter_usize().map(g).collect();
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
        let mut points: Vec<Scalar<P>> = [Scalar::<P>::ONE; 2]
            .into_iter()
            .chain(points_rest)
            .collect();
        let last = points.pop().unwrap();
        points[0] = last;
        Evals::<P>::new(points)
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

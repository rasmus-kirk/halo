#![allow(non_snake_case)]

use ark_ec::VariableBaseMSM;

use halo_group::{Affine, PastaConfig, Point, PublicParams, Scalar};

pub fn commit<P: PastaConfig>(
    w: Option<&Scalar<P>>,
    Gs: &[Affine<P>],
    ms: &[Scalar<P>],
) -> Point<P> {
    let pp = PublicParams::get_pp();

    assert!(
        Gs.len() >= ms.len(),
        "ms must be larger than Gs: (Gs: {}), (ms: {})",
        Gs.len(),
        ms.len()
    );

    let acc = Point::msm_unchecked(Gs, ms);
    if let Some(w) = w {
        pp.S * w + acc
    } else {
        acc
    }
}

#[cfg(test)]
mod tests {
    use ark_pallas::PallasConfig;
    use ark_std::UniformRand;
    use halo_group::{PallasScalar, PublicParams};
    use rand::Rng;

    use super::*;

    fn test_single_homomorphism<R: Rng>(rng: &mut R, l: usize) {
        let pp = PublicParams::new(l);

        // Generate random commit keys
        let Gs = &pp.Gs[0..l];

        // Create random message vectors
        let ms1: Vec<PallasScalar> = (0..l).map(|_| PallasScalar::rand(rng)).collect();
        let ms2: Vec<PallasScalar> = (0..l).map(|_| PallasScalar::rand(rng)).collect();
        let ms_sum: Vec<PallasScalar> =
            ms1.iter().zip(ms2.iter()).map(|(m1, m2)| m1 + m2).collect();

        // Create random hiding factors
        let w1 = PallasScalar::rand(rng);
        let w2 = PallasScalar::rand(rng);

        let inner_sum = commit::<PallasConfig>(Some(&(w1 + w2)), Gs, &ms_sum);
        let outer_sum = commit::<PallasConfig>(Some(&w1), Gs, &ms1)
            + commit::<PallasConfig>(Some(&w2), Gs, &ms2);

        // Check if homomorphism property holds
        println!("{:?}, {}", &pp.Gs[0..l], l);
        println!("{:?}", &pp.Gs[0..l][0].is_on_curve());
        assert!(
            inner_sum == outer_sum,
            "The homomorphism property does not hold."
        );
    }

    #[test]
    fn test_homomorphism_property() {
        let mut rng = ark_std::test_rng();
        let ms_len = 2;
        let tests = 10;

        for _ in 0..tests {
            test_single_homomorphism(&mut rng, ms_len);
        }
    }
}

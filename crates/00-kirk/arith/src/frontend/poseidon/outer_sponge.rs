use halo_group::PastaConfig;
use halo_poseidon::Protocols;

use crate::frontend::{
    poseidon::inner_sponge::InnerSponge,
    primitives::{WireAffine, WireScalar},
};

pub struct OuterSponge<P: PastaConfig> {
    sponge: InnerSponge<P::OtherCurve>,
}

impl<P: PastaConfig> OuterSponge<P> {
    pub fn new(label: Protocols) -> Self {
        let mut inner_sponge = InnerSponge::new();
        let field_label =
            WireScalar::<P::OtherCurve>::constant(P::OtherCurve::scalar_from_u64(label as u64));
        inner_sponge.absorb(&[field_label]);

        OuterSponge {
            sponge: inner_sponge,
        }
    }

    pub fn absorb_g(&mut self, gs: &[WireAffine<P>]) {
        for g in gs {
            self.sponge.absorb(&[g.x, g.y]);
        }
    }

    pub fn absorb_fq(&mut self, x: &[WireScalar<P::OtherCurve>]) {
        for fe in x {
            self.sponge.absorb(&[*fe])
        }
    }

    pub fn absorb_fp(&mut self, x: &[WireScalar<P>]) {
        x.iter().for_each(|x| {
            if P::SCALAR_MODULUS < P::BASE_MODULUS {
                let v = x.fq_message_pass();
                self.sponge.absorb(&[v]);
            } else {
                let (h, l) = x.fp_message_pass();
                self.sponge.absorb(&[h]);
                self.sponge.absorb(&[l]);
            }
        });
    }

    pub fn challenge(&mut self) -> WireScalar<P> {
        let x: WireScalar<P::OtherCurve> = self.sponge.squeeze();
        if P::SCALAR_MODULUS < P::BASE_MODULUS {
            let (h, _) = x.fp_message_pass();
            h
        } else {
            x.fq_message_pass()
        }
    }

    pub fn reset(&mut self) {
        self.sponge.reset()
    }
}

#[cfg(test)]
mod tests {
    use std::array;

    use anyhow::Result;
    use halo_group::{
        Affine, Fp, Fq, PallasConfig, VestaConfig, ark_ff::UniformRand, ark_std::test_rng,
    };
    use halo_poseidon::Protocols;

    use crate::{
        frontend::{
            Call,
            poseidon::outer_sponge::OuterSponge,
            primitives::{WireAffine, WireScalar},
        },
        plonk::PlonkProof,
    };

    #[test]
    fn absorb_squeeze_pallas_scalar() -> Result<()> {
        let rng = &mut test_rng();
        let mut sponge = OuterSponge::<PallasConfig>::new(Protocols::PCDL);

        let witnesses: [WireScalar<PallasConfig>; 10] = array::from_fn(|_| WireScalar::witness());
        sponge.absorb_fp(&witnesses);
        let challenge = sponge.challenge();
        challenge.output();

        let mut call = Call::new();

        let values = [Fp::rand(rng); 10];
        for (w, v) in witnesses.iter().zip(values) {
            call.witness(*w, v)?
        }
        let (fp_trace, fq_trace) = call.trace()?;
        let output = fp_trace.outputs[0];

        let mut sponge = halo_poseidon::Sponge::<PallasConfig>::new(Protocols::PCDL);
        sponge.absorb_fr(&values);
        let expected_output = sponge.challenge();
        assert_eq!(output, expected_output);

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }

    #[test]
    fn absorb_squeeze_pallas_basefield() -> Result<()> {
        let rng = &mut test_rng();
        let mut sponge = OuterSponge::<PallasConfig>::new(Protocols::PCDL);

        let witnesses: [WireScalar<VestaConfig>; 10] = array::from_fn(|_| WireScalar::witness());
        sponge.absorb_fq(&witnesses);
        let challenge = sponge.challenge();
        challenge.output();

        let mut call = Call::new();

        let values = [Fq::rand(rng); 10];
        for (w, v) in witnesses.iter().zip(values) {
            call.witness(*w, v)?
        }
        let (fp_trace, fq_trace) = call.trace()?;
        let output = fp_trace.outputs[0];

        let mut sponge = halo_poseidon::Sponge::<PallasConfig>::new(Protocols::PCDL);
        sponge.absorb_fq(&values);
        let expected_output = sponge.challenge();
        assert_eq!(output, expected_output);

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }

    #[test]
    fn absorb_squeeze_pallas_affine() -> Result<()> {
        let rng = &mut test_rng();
        let mut sponge = OuterSponge::<PallasConfig>::new(Protocols::PCDL);

        let witnesses: [WireAffine<PallasConfig>; 10] = array::from_fn(|_| WireAffine::witness());
        sponge.absorb_g(&witnesses);
        let challenge = sponge.challenge();
        challenge.output();

        let mut call = Call::new();

        let values = [Affine::rand(rng); 10];
        for (w, v) in witnesses.iter().zip(values) {
            call.witness_affine(*w, v)?
        }
        let (fp_trace, fq_trace) = call.trace()?;
        let output = fp_trace.outputs[0];

        let mut sponge = halo_poseidon::Sponge::<PallasConfig>::new(Protocols::PCDL);
        sponge.absorb_g_affine(&values);
        let expected_output = sponge.challenge();
        assert_eq!(output, expected_output);

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }
}

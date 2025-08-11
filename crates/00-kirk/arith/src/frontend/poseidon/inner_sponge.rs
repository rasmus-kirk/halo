use halo_group::PastaConfig;
use halo_poseidon::{SPONGE_RATE, STATE_SIZE};

use crate::frontend::{FRONTEND, primitives::WireScalar};

#[derive(Clone, Debug)]
enum SpongeState {
    Absorbed(usize),
    Squeezed(usize),
}

#[derive(Clone, Debug)]
pub struct InnerSponge<P: PastaConfig> {
    state: [WireScalar<P>; STATE_SIZE],
    sponge_state: SpongeState,
}
impl<P: PastaConfig> InnerSponge<P> {
    pub(crate) fn poseidon_block_cipher(state: &mut [WireScalar<P>; STATE_SIZE]) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let mut wire_state = state.map(|x| x.wire);
            for i in 0..11 {
                wire_state = frontend.circuit.poseidon(i, wire_state);
            }
            wire_state = frontend.circuit.poseidon_finish(wire_state);
            *state = wire_state.map(WireScalar::new)
        })
    }

    pub(crate) fn new() -> Self {
        Self {
            state: [WireScalar::<P>::zero(); STATE_SIZE],
            sponge_state: SpongeState::Absorbed(0),
        }
    }

    pub(crate) fn absorb(&mut self, x: &[WireScalar<P>]) {
        for x in x.iter() {
            match self.sponge_state {
                SpongeState::Absorbed(n) if n < SPONGE_RATE => {
                    self.sponge_state = SpongeState::Absorbed(n + 1);
                    self.state[n] += *x;
                }
                SpongeState::Absorbed(SPONGE_RATE) => {
                    Self::poseidon_block_cipher(&mut self.state);
                    self.sponge_state = SpongeState::Absorbed(1);
                    self.state[0] += *x;
                }
                SpongeState::Squeezed(_n) => {
                    self.sponge_state = SpongeState::Absorbed(1);
                    self.state[0] += *x;
                }
                _ => panic!("Impossible case found"),
            }
        }
    }

    pub(crate) fn squeeze(&mut self) -> WireScalar<P> {
        match self.sponge_state {
            SpongeState::Squeezed(n) if n < SPONGE_RATE => {
                self.sponge_state = SpongeState::Squeezed(n + 1);
                self.state[n]
            }
            SpongeState::Squeezed(SPONGE_RATE) | SpongeState::Absorbed(_) => {
                Self::poseidon_block_cipher(&mut self.state);
                self.sponge_state = SpongeState::Squeezed(1);
                self.state[0]
            }
            _ => panic!("Impossible case found"),
        }
    }

    pub(crate) fn reset(&mut self) {
        self.state = [WireScalar::zero(); STATE_SIZE];
        self.sponge_state = SpongeState::Absorbed(0);
    }
}

#[cfg(test)]
mod tests {
    use std::array;

    use anyhow::Result;
    use halo_group::{Fp, PallasConfig, VestaConfig, ark_ff::UniformRand, ark_std::test_rng};
    use halo_poseidon::STATE_SIZE;

    use crate::{
        frontend::{Call, poseidon::inner_sponge::InnerSponge, primitives::WireScalar},
        plonk::PlonkProof,
    };

    #[test]
    fn permutation() -> Result<()> {
        let rng = &mut test_rng();
        let s0 = WireScalar::<PallasConfig>::witness();
        let s1 = WireScalar::<PallasConfig>::witness();
        let s2 = WireScalar::<PallasConfig>::witness();
        let mut state = [s0.clone(), s1.clone(), s2.clone()];
        InnerSponge::<PallasConfig>::poseidon_block_cipher(&mut state);
        state[0].output();
        state[1].output();
        state[2].output();

        let mut call = Call::new();

        let s0_v = Fp::rand(rng);
        let s1_v = Fp::rand(rng);
        let s2_v = Fp::rand(rng);

        call.witness(s0, s0_v)?;
        call.witness(s1, s1_v)?;
        call.witness(s2, s2_v)?;

        let (fp_trace, fq_trace) = call.trace(None)?;

        let outputs: [_; STATE_SIZE] = array::from_fn(|i| fp_trace.outputs[i]);
        let mut expected_state = [s0_v, s1_v, s2_v];
        halo_poseidon::inner_sponge::poseidon_block_cipher::<VestaConfig>(&mut expected_state);
        assert_eq!(outputs, expected_state);

        let (plonk_public_input, plonk_witness) = fp_trace.consume();
        PlonkProof::naive_prover(rng, plonk_witness).verify(plonk_public_input)?;
        let (plonk_public_input, plonk_witness) = fq_trace.consume();
        PlonkProof::naive_prover(rng, plonk_witness).verify(plonk_public_input)?;

        Ok(())
    }

    #[test]
    fn absorb_squeeze() -> Result<()> {
        let rng = &mut test_rng();
        let mut sponge = InnerSponge::<PallasConfig>::new();

        let witnesses: [WireScalar<PallasConfig>; 10] = array::from_fn(|_| WireScalar::witness());
        sponge.absorb(&witnesses);
        sponge.squeeze().output();

        let mut call = Call::new();

        let values = [Fp::rand(rng); 10];
        for (w, v) in witnesses.iter().zip(values) {
            call.witness(*w, v)?
        }
        let (fp_trace, fq_trace) = call.trace(None)?;
        let output = fp_trace.outputs[0];

        let mut sponge = halo_poseidon::inner_sponge::PoseidonSponge::<VestaConfig>::new();
        sponge.absorb(&values);
        let expected_output = sponge.squeeze();
        assert_eq!(output, expected_output);

        let (plonk_public_input, plonk_witness) = fp_trace.consume();
        PlonkProof::naive_prover(rng, plonk_witness).verify(plonk_public_input)?;
        let (plonk_public_input, plonk_witness) = fq_trace.consume();
        PlonkProof::naive_prover(rng, plonk_witness).verify(plonk_public_input)?;

        Ok(())
    }
}

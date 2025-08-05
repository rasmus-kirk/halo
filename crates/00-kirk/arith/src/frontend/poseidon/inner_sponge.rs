use halo_poseidon::{SPONGE_RATE, STATE_SIZE};

use crate::frontend::{FRONTEND, field::Fp};

#[derive(Clone, Debug)]
enum SpongeState {
    Absorbed(usize),
    Squeezed(usize),
}

#[derive(Clone, Debug)]
pub struct InnerSponge {
    state: [Fp; STATE_SIZE],
    sponge_state: SpongeState,
}
impl InnerSponge {
    pub(crate) fn poseidon_block_cipher(state: &mut [Fp; STATE_SIZE]) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let mut wire_state = state.map(|x| x.wire);
            for i in 0..11 {
                wire_state = frontend.fp_circuit.poseidon_gate(i, wire_state);
            }
            wire_state = frontend.fp_circuit.poseidon_gate_finish(wire_state);
            *state = wire_state.map(Fp::new)
        })
    }

    pub(crate) fn new() -> Self {
        Self {
            state: [Fp::zero(); STATE_SIZE],
            sponge_state: SpongeState::Absorbed(0),
        }
    }

    pub(crate) fn absorb(&mut self, x: &[Fp]) {
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

    pub(crate) fn squeeze(&mut self) -> Fp {
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
        self.state = [Fp::zero(); STATE_SIZE];
        self.sponge_state = SpongeState::Absorbed(0);
    }
}

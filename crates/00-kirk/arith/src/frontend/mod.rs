use anyhow::Result;
use halo_poseidon::{SPONGE_RATE, STATE_SIZE};
use std::{
    cell::RefCell,
    ops::{Add, AddAssign, Mul},
};

use halo_group::{
    PallasConfig, Scalar, VestaConfig,
    ark_std::One,
    ark_std::Zero,
    ark_std::rand::{RngCore, thread_rng},
};
// use halo_poseidon::{SPONGE_RATE, STATE_SIZE};

use crate::circuit::{CircuitSpec, Trace, TraceBuilder, Wire};

// Thread-local Frontend instance
thread_local! {
    static FRONTEND: RefCell<Frontend> = RefCell::new(Frontend::new());
}

#[derive(Clone, Copy, Debug)]
struct Fp {
    wire: Wire,
}

impl Fp {
    // Create a new wire using the thread-local frontend
    fn witness() -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let wire = frontend.fp_circuit.witness_gate();
            Fp { wire }
        })
    }

    fn constant(c: Scalar<PallasConfig>) -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let wire = frontend.fp_circuit.constant_gate(c);
            Fp { wire }
        })
    }

    fn zero() -> Self {
        Self::constant(Scalar::<PallasConfig>::zero())
    }

    fn one() -> Self {
        Self::constant(Scalar::<PallasConfig>::one())
    }

    fn from_wire(wire: Wire) -> Self {
        Fp { wire }
    }

    fn output(self) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            frontend.fp_circuit.output_gate(self.wire);
            println!("{:?}", frontend.fp_circuit);
        })
    }
}

impl Mul for Fp {
    type Output = Fp;

    fn mul(self, other: Fp) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let wire = frontend.fp_circuit.mul_gate(self.wire, other.wire);
            Fp { wire }
        })
    }
}

impl Add for Fp {
    type Output = Fp;

    fn add(self, other: Fp) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let wire = frontend.fp_circuit.add_gate(self.wire, other.wire);
            Fp { wire }
        })
    }
}

impl AddAssign for Fp {
    fn add_assign(&mut self, rhs: Self) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let wire = frontend.fp_circuit.add_gate(self.wire, rhs.wire);
            *self = Fp { wire }
        })
    }
}

#[derive(Clone, Debug)]
enum SpongeState {
    Absorbed(usize),
    Squeezed(usize),
}

#[derive(Clone, Debug)]
struct InnerSponge {
    state: [Fp; STATE_SIZE],
    sponge_state: SpongeState,
}
impl InnerSponge {
    fn poseidon_block_cipher(state: &mut [Fp; STATE_SIZE]) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let mut wire_state = state.map(|x| x.wire);
            for i in 0..11 {
                wire_state = frontend.fp_circuit.poseidon_gate(i, wire_state);
            }
            wire_state = frontend.fp_circuit.poseidon_gate_finish(wire_state);
            *state = wire_state.map(Fp::from_wire)
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

#[derive(Clone)]
struct Frontend {
    fp_circuit: CircuitSpec<PallasConfig>,
    fq_circuit: CircuitSpec<VestaConfig>,
}

impl Frontend {
    pub fn new() -> Self {
        Self {
            fp_circuit: CircuitSpec::new(),
            fq_circuit: CircuitSpec::new(),
        }
    }
}

struct Call {
    fp_trace_builder: TraceBuilder<PallasConfig>,
    fq_trace_builder: TraceBuilder<VestaConfig>,
}
impl Call {
    fn new() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            let fp_trace_builder = TraceBuilder::new(frontend.fp_circuit.clone());
            let fq_trace_builder = TraceBuilder::new(frontend.fq_circuit.clone());
            Call {
                fp_trace_builder,
                fq_trace_builder,
            }
        })
    }
    fn fp_witness(&mut self, fp: Fp, scalar: Scalar<PallasConfig>) -> Result<()> {
        self.fp_trace_builder.witness(fp.wire, scalar)
    }
    fn fp_public_input(&mut self, fp: Fp, scalar: Scalar<PallasConfig>) -> Result<()> {
        self.fp_trace_builder.public_input(fp.wire, scalar)
    }
    fn trace(self) -> Result<(Trace<PallasConfig>, Trace<VestaConfig>)> {
        Ok((
            self.fp_trace_builder.trace()?,
            self.fq_trace_builder.trace()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::array;

    use anyhow::Result;
    use halo_group::{
        PallasConfig, PastaConfig, Scalar, VestaConfig,
        ark_std::{
            rand::{Rng, RngCore},
            test_rng,
        },
    };
    use halo_poseidon::STATE_SIZE;

    use crate::{
        frontend::{Call, FRONTEND, Fp, InnerSponge},
        plonk::PlonkProof,
    };

    fn random_scalar<R: Rng>(rng: &mut R) -> Scalar<PallasConfig> {
        PallasConfig::scalar_from_u64(rng.next_u64())
    }

    #[test]
    fn test_mul() -> Result<()> {
        let rng = &mut test_rng();
        let a = Fp::witness();
        let b = Fp::witness();
        let c = a.clone() * b.clone();
        let d = a.clone() + c.clone();
        d.output();

        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow_mut();
            println!("{:?}", frontend.fp_circuit);
        });

        let mut call = Call::new();

        call.fp_witness(a, random_scalar(rng))?;
        call.fp_witness(b, random_scalar(rng))?;

        let (fp_trace, fq_trace) = call.trace()?;
        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }

    #[test]
    fn test_poseidon_permutation() -> Result<()> {
        let rng = &mut test_rng();
        let s0 = Fp::witness();
        let s1 = Fp::witness();
        let s2 = Fp::witness();
        let mut state = [s0.clone(), s1.clone(), s2.clone()];
        InnerSponge::poseidon_block_cipher(&mut state);
        state[0].clone().output();
        state[1].clone().output();
        state[2].clone().output();

        let mut call = Call::new();

        let s0_v = random_scalar(rng);
        let s1_v = random_scalar(rng);
        let s2_v = random_scalar(rng);

        call.fp_witness(s0, s0_v)?;
        call.fp_witness(s1, s1_v)?;
        call.fp_witness(s2, s2_v)?;

        let (fp_trace, fq_trace) = call.trace()?;

        let outputs: [_; STATE_SIZE] = array::from_fn(|i| fp_trace.outputs[i]);
        let mut expected_state = [s0_v, s1_v, s2_v];
        halo_poseidon::inner_sponge::poseidon_block_cipher::<VestaConfig>(&mut expected_state);
        assert_eq!(outputs, expected_state);

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }

    #[test]
    fn test_poseidon_absorb_squeeze() -> Result<()> {
        let rng = &mut test_rng();
        let mut sponge = InnerSponge::new();

        let witnesses: [Fp; 10] = array::from_fn(|_| Fp::witness());
        sponge.absorb(&witnesses);
        sponge.squeeze().output();

        let mut call = Call::new();

        println!("ws: {:?}", witnesses);
        let values = [random_scalar(rng); 10];
        for (w, v) in witnesses.iter().zip(values) {
            call.fp_witness(*w, v)?
        }
        let (fp_trace, fq_trace) = call.trace()?;
        let output = fp_trace.outputs[0];

        let mut sponge = halo_poseidon::inner_sponge::PoseidonSponge::<VestaConfig>::new();
        sponge.absorb(&values);
        let expected_output = sponge.squeeze();
        assert_eq!(output, expected_output);

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }
}

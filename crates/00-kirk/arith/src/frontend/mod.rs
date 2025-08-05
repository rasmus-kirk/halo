use anyhow::Result;
use std::cell::RefCell;

use halo_group::{
    Affine, PallasConfig, Scalar, VestaConfig,
    ark_std::{One, Zero},
};

use crate::{
    circuit::{CircuitSpec, Trace, TraceBuilder},
    frontend::{
        curve::CurvePoint,
        field::{Fp, Fq},
    },
};

pub mod curve;
pub mod field;
pub mod poseidon;

thread_local! {
    static FRONTEND: RefCell<Frontend> = RefCell::new(Frontend::new());
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

pub struct Call {
    fp_trace_builder: TraceBuilder<PallasConfig>,
    fq_trace_builder: TraceBuilder<VestaConfig>,
}
impl Call {
    pub fn new() -> Self {
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
    pub fn fp_witness(&mut self, fp: Fp, scalar: Scalar<PallasConfig>) -> Result<()> {
        self.fp_trace_builder.witness(fp.wire, scalar)
    }
    pub fn fq_witness(&mut self, fq: Fq, scalar: Scalar<VestaConfig>) -> Result<()> {
        self.fq_trace_builder.witness(fq.wire, scalar)
    }
    pub fn curve_witness(&mut self, p: CurvePoint, affine: Affine<PallasConfig>) -> Result<()> {
        assert!(affine.is_on_curve());
        self.fq_trace_builder.witness(p.x.wire, affine.x)?;
        self.fq_trace_builder.witness(p.y.wire, affine.y)?;
        Ok(())
    }
    pub fn fp_public_input(&mut self, fp: Fp, scalar: Scalar<PallasConfig>) -> Result<()> {
        self.fp_trace_builder.public_input(fp.wire, scalar)
    }
    pub fn trace(self) -> Result<(Trace<PallasConfig>, Trace<VestaConfig>)> {
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
        ark_std::{rand::Rng, test_rng},
    };
    use halo_poseidon::STATE_SIZE;

    use crate::{
        frontend::{Call, FRONTEND, field::Fp, poseidon::inner_sponge::InnerSponge},
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

use std::ops::{Add, AddAssign, Mul, Neg};

use halo_group::{
    PallasConfig, Scalar, VestaConfig,
    ark_std::{One, Zero},
};

use crate::{circuit::Wire, frontend::FRONTEND};

#[derive(Clone, Copy, Debug)]
pub struct Fp {
    pub(crate) wire: Wire,
}

impl Fp {
    pub(crate) fn new(wire: Wire) -> Self {
        Self { wire }
    }

    // Create a new wire using the thread-local frontend
    pub fn witness() -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Fp::new(frontend.fp_circuit.witness_gate())
        })
    }

    pub fn constant(c: Scalar<PallasConfig>) -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Fp::new(frontend.fp_circuit.constant_gate(c))
        })
    }

    pub fn inv(&self) -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Fp::new(frontend.fp_circuit.inv_gate(self.wire))
        })
    }

    pub fn zero() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            Fp::new(frontend.fq_circuit.zero)
        })
    }

    pub fn one() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            Fp::new(frontend.fq_circuit.one)
        })
    }

    pub fn from_wire(wire: Wire) -> Self {
        Fp { wire }
    }

    pub fn output(self) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            frontend.fp_circuit.output_gate(self.wire)
        })
    }
}

impl Mul for Fp {
    type Output = Fp;

    fn mul(self, other: Fp) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Fp::new(frontend.fp_circuit.mul_gate(self.wire, other.wire))
        })
    }
}

impl Add for Fp {
    type Output = Fp;

    fn add(self, other: Fp) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Fp::new(frontend.fp_circuit.add_gate(self.wire, other.wire))
        })
    }
}

impl AddAssign for Fp {
    fn add_assign(&mut self, rhs: Self) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            *self = Fp::new(frontend.fp_circuit.add_gate(self.wire, rhs.wire))
        })
    }
}

impl Neg for Fp {
    type Output = Fp;

    fn neg(self) -> Fp {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Fp::new(frontend.fp_circuit.neg_gate(self.wire))
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Fq {
    pub(crate) wire: Wire,
}

impl Fq {
    pub(crate) fn new(wire: Wire) -> Self {
        Self { wire }
    }

    // Create a new wire using the thread-local frontend
    pub fn witness() -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Fq::new(frontend.fq_circuit.witness_gate())
        })
    }

    pub fn constant(c: Scalar<VestaConfig>) -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Fq::new(frontend.fq_circuit.constant_gate(c))
        })
    }

    pub fn inv(&self) -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Fq::new(frontend.fq_circuit.inv_gate(self.wire))
        })
    }

    pub fn zero() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            Fq::new(frontend.fq_circuit.zero)
        })
    }

    pub fn one() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            Fq::new(frontend.fq_circuit.one)
        })
    }

    pub fn from_wire(wire: Wire) -> Self {
        Fq { wire }
    }

    pub fn output(self) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            frontend.fq_circuit.output_gate(self.wire)
        })
    }
}

impl Mul for Fq {
    type Output = Fq;

    fn mul(self, other: Fq) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Fq::new(frontend.fq_circuit.mul_gate(self.wire, other.wire))
        })
    }
}

impl Add for Fq {
    type Output = Fq;

    fn add(self, other: Fq) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Fq::new(frontend.fq_circuit.add_gate(self.wire, other.wire))
        })
    }
}

impl AddAssign for Fq {
    fn add_assign(&mut self, rhs: Self) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            *self = Fq::new(frontend.fq_circuit.add_gate(self.wire, rhs.wire))
        })
    }
}

impl Neg for Fq {
    type Output = Fq;

    fn neg(self) -> Fq {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Fq::new(frontend.fq_circuit.neg_gate(self.wire))
        })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use halo_group::{
        PallasConfig, PastaConfig, Scalar, VestaConfig,
        ark_ff::Field,
        ark_std::{rand::Rng, test_rng},
    };

    use crate::{
        frontend::{
            Call, FRONTEND,
            field::{Fp, Fq},
        },
        plonk::PlonkProof,
    };

    fn random_fp_scalar<R: Rng>(rng: &mut R) -> Scalar<PallasConfig> {
        PallasConfig::scalar_from_u64(rng.next_u64())
    }

    fn random_fq_scalar<R: Rng>(rng: &mut R) -> Scalar<VestaConfig> {
        VestaConfig::scalar_from_u64(rng.next_u64())
    }

    #[test]
    fn fp_add_mul() -> Result<()> {
        let rng = &mut test_rng();
        let x_v = random_fp_scalar(rng);
        let y_v = random_fp_scalar(rng);
        let z_v = random_fp_scalar(rng);
        let x = Fp::witness();
        let y = Fp::witness();
        let z = Fp::constant(z_v);
        let c = x * y;
        let d = c + z;
        d.output();

        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow_mut();
            println!("{:?}", frontend.fp_circuit);
        });

        let mut call = Call::new();

        call.fp_witness(x, x_v)?;
        call.fp_witness(y, y_v)?;

        let (fp_trace, fq_trace) = call.trace()?;

        let output = fp_trace.outputs[0];
        let expected_output = x_v * y_v + z_v;
        assert_eq!(fp_trace.outputs.len(), 1);
        assert_eq!(output, expected_output);

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }

    #[test]
    fn fp_inv_neg() -> Result<()> {
        let rng = &mut test_rng();
        let x_v = random_fp_scalar(rng);
        let y_v = random_fp_scalar(rng);
        let z_v = random_fp_scalar(rng);
        let x = Fp::witness();
        let y = Fp::witness();
        let z = Fp::witness();
        let x_inv = x.inv();
        let y_neg = -y;
        let _ = z.inv();
        x_inv.output();
        y_neg.output();

        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow_mut();
            println!("{:?}", frontend.fp_circuit);
        });

        let mut call = Call::new();

        call.fp_witness(x, x_v)?;
        call.fp_witness(y, y_v)?;
        call.fp_witness(z, z_v)?;

        let (fp_trace, fq_trace) = call.trace()?;

        assert_eq!(fp_trace.outputs.len(), 2);
        assert_eq!(fp_trace.outputs, [x_v.inverse().unwrap(), -y_v]);

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }

    #[test]
    fn fq_add_mul() -> Result<()> {
        let rng = &mut test_rng();
        let x_v = random_fq_scalar(rng);
        let y_v = random_fq_scalar(rng);
        let z_v = random_fq_scalar(rng);
        let x = Fq::witness();
        let y = Fq::witness();
        let z = Fq::constant(z_v);
        let c = x * y;
        let d = c + z;
        d.output();

        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow_mut();
            println!("{:?}", frontend.fp_circuit);
        });

        let mut call = Call::new();

        call.fq_witness(x, x_v)?;
        call.fq_witness(y, y_v)?;

        let (fp_trace, fq_trace) = call.trace()?;
        println!("----- FP TRACE -----");
        println!("{:?}", fp_trace);

        println!("----- FQ TRACE -----");
        println!("{:?}", fq_trace);

        let output = fq_trace.outputs[0];
        let expected_output = x_v * y_v + z_v;
        assert_eq!(fq_trace.outputs.len(), 1);
        assert_eq!(output, expected_output);

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }
}

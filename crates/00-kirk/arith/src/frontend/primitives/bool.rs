use std::{
    marker::PhantomData,
    ops::{BitAnd, Not},
};

use halo_group::PastaConfig;

use crate::{
    circuit::Wire,
    frontend::{FRONTEND, curve::WireAffine, field::WireScalar},
};

#[derive(Clone, Copy, Debug)]
pub struct WireBool<P: PastaConfig> {
    pub(crate) wire: Wire,
    _p: PhantomData<P>,
}

impl<P: PastaConfig> WireBool<P> {
    pub(crate) fn new(wire: Wire) -> Self {
        Self {
            wire,
            _p: PhantomData::default(),
        }
    }

    pub fn witness() -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Self::new(frontend.circuit.witness_bool(P::SFID))
        })
    }

    pub fn constant(b: bool) -> Self {
        if b { Self::t() } else { Self::f() }
    }

    pub fn f() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            WireBool::new(frontend.circuit.zero[P::SFID as usize])
        })
    }

    pub fn t() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            WireBool::new(frontend.circuit.one[P::SFID as usize])
        })
    }

    pub fn assert_eq(&self, other: Self) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            frontend.circuit.assert_eq_gate(self.wire, other.wire);
        })
    }

    pub fn from_wire(wire: Wire) -> Self {
        WireBool {
            wire,
            _p: PhantomData::default(),
        }
    }

    pub fn scalar_cmp(a: WireScalar<P>, b: WireScalar<P>) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Self::new(frontend.circuit.eq_gate(a.wire, b.wire));
        })
    }

    pub fn scalar_ite(self, true_case: WireScalar<P>, false_case: WireScalar<P>) -> WireScalar<P> {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let cond_times_true_case = frontend.circuit.mul_gate(self.wire, true_case.wire);
            let one = frontend.circuit.one[P::SFID as usize];
            let minus_cond = frontend.circuit.neg_gate(self.wire);
            let one_minus_cond = frontend.circuit.add_gate(one, minus_cond);
            let one_minus_cond_times_false_case =
                frontend.circuit.mul_gate(one_minus_cond, false_case.wire);
            let out = frontend
                .circuit
                .add_gate(cond_times_true_case, one_minus_cond_times_false_case);
            WireScalar::new(out)
        })
    }

    pub fn affine_ite(
        cond: Self,
        true_case: WireAffine<P::OtherCurve>,
        false_case: WireAffine<P::OtherCurve>,
    ) -> WireAffine<P::OtherCurve> {
        let x = cond.scalar_ite(true_case.x, false_case.x);
        let y = cond.scalar_ite(true_case.y, false_case.y);
        WireAffine::new(x.wire, y.wire)
    }

    pub fn output(self) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            frontend.circuit.output_gate(self.wire)
        })
    }
}
impl<P: PastaConfig> Not for WireBool<P> {
    type Output = Self;

    fn not(self) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let one = frontend.circuit.one[P::SFID as usize];
            frontend.circuit.add_gate(one, self.wire);
            WireBool::new(frontend.circuit.add_gate(one, self.wire))
        })
    }
}
impl<P: PastaConfig> BitAnd for WireBool<P> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            WireBool::new(frontend.circuit.mul_gate(self.wire, rhs.wire))
        })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use halo_group::{
        Fp, PallasConfig,
        ark_ff::{AdditiveGroup, Field, UniformRand},
        ark_std::{rand::Rng, test_rng},
    };

    use crate::{
        frontend::{Call, FRONTEND, field::WireScalar, primitives::bool::WireBool},
        plonk::PlonkProof,
    };

    #[test]
    fn bool_fp_and_neg() -> Result<()> {
        let rng = &mut test_rng();
        let x_v = rng.gen_bool(0.5);
        let y_v = rng.gen_bool(0.5);
        let x = WireBool::<PallasConfig>::witness();
        let y = WireBool::<PallasConfig>::witness();
        let c = x & y;
        let d = !x;
        c.output();
        d.output();

        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow_mut();
            println!("{:?}", frontend.circuit);
        });

        let mut call = Call::new();

        call.witness_scalar_bool(x, x_v)?;
        call.witness_scalar_bool(y, y_v)?;

        let (fp_trace, fq_trace) = call.trace()?;

        let c_out = fp_trace.outputs[0];
        let d_out = fp_trace.outputs[1];
        let expected_c_out = x_v && y_v;
        let expected_d_out = !x_v;
        assert_eq!(fp_trace.outputs.len(), 2);
        assert_eq!(c_out, expected_c_out.into());
        assert_eq!(d_out, expected_d_out.into());

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }

    #[test]
    fn bool_fp_neq() -> Result<()> {
        let rng = &mut test_rng();
        let x_v = Fp::rand(rng);
        let y_v = Fp::rand(rng);
        let x = WireScalar::<PallasConfig>::witness();
        let y = WireScalar::<PallasConfig>::witness();
        let neq = x.equals(y);
        neq.output();

        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow_mut();
            println!("{:?}", frontend.circuit);
        });

        let mut call = Call::new();

        call.witness(x, x_v)?;
        call.witness(y, y_v)?;

        let (fp_trace, fq_trace) = call.trace()?;
        println!("{:?}", fp_trace);

        let neq_out = fp_trace.outputs[0];
        let expected_neq_out = Fp::ZERO;
        assert_eq!(neq_out, expected_neq_out.into());

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }

    #[test]
    fn bool_fp_eq() -> Result<()> {
        let rng = &mut test_rng();
        let x_v = Fp::rand(rng);
        let y_v = x_v;
        let x = WireScalar::<PallasConfig>::witness();
        let y = WireScalar::<PallasConfig>::witness();
        let eq = x.equals(y);
        eq.output();

        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow_mut();
            println!("{:?}", frontend.circuit);
        });

        let mut call = Call::new();

        call.witness(x, x_v)?;
        call.witness(y, y_v)?;

        let (fp_trace, fq_trace) = call.trace()?;
        println!("{:?}", fp_trace);

        let eq_out = fp_trace.outputs[0];
        let expected_eq_out = Fp::ONE;
        assert_eq!(fp_trace.outputs.len(), 1);
        assert_eq!(eq_out, expected_eq_out);

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }

    #[test]
    fn bool_fp_scalar_ite() -> Result<()> {
        let rng = &mut test_rng();
        let b_v = rng.gen_bool(0.5);
        let x_v = Fp::from(5u64);
        let y_v = Fp::from(9u64);
        let b = WireBool::<PallasConfig>::witness();
        let x = WireScalar::<PallasConfig>::witness();
        let y = WireScalar::<PallasConfig>::witness();
        let c = b.scalar_ite(x, y);
        c.output();

        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow_mut();
            println!("{:?}", frontend.circuit);
        });

        let mut call = Call::new();

        call.witness_scalar_bool(b, b_v)?;
        call.witness(x, x_v)?;
        call.witness(y, y_v)?;

        let (fp_trace, fq_trace) = call.trace()?;
        println!("{:?}", fp_trace);

        let c_out = fp_trace.outputs[0];
        let expected_c_out = if b_v { x_v } else { y_v };
        assert_eq!(fp_trace.outputs.len(), 1);
        assert_eq!(c_out, expected_c_out);

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }
}

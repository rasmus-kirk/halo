use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Mul, Neg},
};

use halo_group::{
    PallasConfig, PastaConfig, PastaFE, PastaFieldId, Scalar, VestaConfig,
    ark_ec::CurveConfig,
    ark_std::{One, Zero},
};

use crate::{circuit::Wire, frontend::FRONTEND};

#[derive(Clone, Copy, Debug)]
pub struct WireScalar<P: PastaConfig> {
    pub(crate) wire: Wire,
    _p: PhantomData<P>,
}

impl<P: PastaConfig> WireScalar<P> {
    pub(crate) fn new(wire: Wire) -> Self {
        Self {
            wire,
            _p: PhantomData::default(),
        }
    }

    // Create a new wire using the thread-local frontend
    pub fn witness() -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Self::new(frontend.circuit.witness(P::SFID))
        })
    }

    pub fn constant(c: Scalar<P>) -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            WireScalar::new(frontend.circuit.constant(PastaFE::from_scalar::<P>(c)))
        })
    }

    pub fn inv(&self) -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            WireScalar::new(frontend.circuit.inv(self.wire))
        })
    }

    pub fn zero() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            WireScalar::new(frontend.circuit.zero[P::SFID as usize])
        })
    }

    pub fn one() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            WireScalar::new(frontend.circuit.one[P::SFID as usize])
        })
    }

    pub fn assert_eq(&self, other: Self) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            frontend.circuit.assert_eq_gate(self.wire, other.wire);
        })
    }

    pub(crate) fn message_pass(self) -> (WireScalar<P::OtherCurve>, WireScalar<P::OtherCurve>) {
        if P::IS_PALLAS {
            FRONTEND.with(|frontend| {
                let mut frontend = frontend.borrow_mut();
                let (h, l) = frontend.circuit.fp_message_pass(self.wire);
                (WireScalar::new(h), WireScalar::new(l))
            })
        } else {
            panic!("Not Pallas!")
        }
    }

    pub fn from_wire(wire: Wire) -> Self {
        WireScalar {
            wire,
            _p: PhantomData::default(),
        }
    }

    pub fn output(self) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            frontend.circuit.output_gate(self.wire)
        })
    }
}

impl<P: PastaConfig> Mul for WireScalar<P> {
    type Output = WireScalar<P>;

    fn mul(self, other: WireScalar<P>) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            WireScalar::new(frontend.circuit.mul(self.wire, other.wire))
        })
    }
}

impl<P: PastaConfig> Add for WireScalar<P> {
    type Output = WireScalar<P>;

    fn add(self, other: WireScalar<P>) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            WireScalar::new(frontend.circuit.add(self.wire, other.wire))
        })
    }
}

impl<P: PastaConfig> AddAssign for WireScalar<P> {
    fn add_assign(&mut self, rhs: Self) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            *self = WireScalar::new(frontend.circuit.add(self.wire, rhs.wire))
        })
    }
}

impl<P: PastaConfig> Neg for WireScalar<P> {
    type Output = WireScalar<P>;

    fn neg(self) -> WireScalar<P> {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            WireScalar::new(frontend.circuit.neg_gate(self.wire))
        })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use halo_group::{
        Fp, Fq, PallasConfig, PastaConfig, PastaFE, Scalar, VestaConfig,
        ark_ff::{BigInt, Field, PrimeField},
        ark_std::{
            rand::{Rng, RngCore},
            test_rng,
        },
    };

    use crate::{
        frontend::{Call, FRONTEND, field::WireScalar},
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
        let x = WireScalar::<PallasConfig>::witness();
        let y = WireScalar::<PallasConfig>::witness();
        let z = WireScalar::<PallasConfig>::constant(z_v);
        let c = x * y;
        let d = c + z;
        d.output();

        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow_mut();
            println!("{:?}", frontend.circuit);
        });

        let mut call = Call::new();

        call.witness(x, x_v)?;
        call.witness(y, y_v)?;

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
        let x = WireScalar::<PallasConfig>::witness();
        let y = WireScalar::<PallasConfig>::witness();
        let z = WireScalar::<PallasConfig>::witness();
        let x_inv = x.inv();
        let y_neg = -y;
        let _ = z.inv();
        x_inv.output();
        y_neg.output();

        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow_mut();
            println!("{:?}", frontend.circuit);
        });

        let mut call = Call::new();

        call.witness(x, x_v)?;
        call.witness(y, y_v)?;
        call.witness(z, z_v)?;

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
        let x_v = Fq::from(3u64);
        let y_v = Fq::from(5u64);
        let z_v = Fq::from(7u64);
        println!("{x_v}");
        // let x_v = random_fq_scalar(rng);
        // let y_v = random_fq_scalar(rng);
        // let z_v = random_fq_scalar(rng);
        let x = WireScalar::<VestaConfig>::witness();
        let y = WireScalar::<VestaConfig>::witness();
        let z = WireScalar::constant(z_v);
        let c = x * y;
        let d = c + z;
        d.output();

        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow_mut();
            println!("{:?}", frontend.circuit);
        });

        let mut call = Call::new();

        call.witness(x, x_v)?;
        call.witness(y, y_v)?;

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

    #[test]
    fn pasta_field_inversion() -> Result<()> {
        let rng = &mut test_rng();
        let x_u64 = rng.next_u64();
        let x_fp = Fp::from(x_u64);
        let x_fq = Fq::from(x_u64);
        let x_fp_pfe = PastaFE::from(x_fp);
        println!("{:?}", x_u64);
        println!("{:?}", x_fp);
        println!("{:?}", x_fq);
        println!("{:?}", x_fp_pfe);
        assert_eq!(x_fq, x_fp_pfe.into());

        Ok(())
    }
}

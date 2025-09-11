use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use halo_group::{
    PastaConfig, PastaFE, PastaFieldId, Scalar,
    ark_ff::{One, Zero},
};

use crate::{
    circuit::Wire,
    frontend::{FRONTEND, primitives::bool::WireBool},
};

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

    pub fn public_input() -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            Self::new(frontend.circuit.public_input(P::SFID))
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

    // [1, self, self^2, self^3, ...]
    pub fn geometric_series(&self, n: usize) -> Vec<Self> {
        let mut result = Vec::with_capacity(n);
        let mut current = WireScalar::<P>::one();
        for _ in 0..n {
            result.push(current);
            current *= *self;
        }
        result
    }

    pub fn square(&self) -> Self {
        *self * *self
    }

    pub fn double(&self) -> Self {
        *self + *self
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

    pub fn equals(self, b: Self) -> WireBool<P> {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            WireBool::new(frontend.circuit.eq_gate(self.wire, b.wire))
        })
    }

    pub(crate) fn fp_message_pass(self) -> (WireScalar<P::OtherCurve>, WireScalar<P::OtherCurve>) {
        assert!(self.wire.fid == PastaFieldId::Fp);
        assert!(P::IS_PALLAS);
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let (h, l) = frontend.circuit.fp_message_pass(self.wire);
            (WireScalar::new(h), WireScalar::new(l))
        })
    }

    pub(crate) fn fq_message_pass(self) -> WireScalar<P::OtherCurve> {
        assert!(self.wire.fid == PastaFieldId::Fq);
        assert!(!P::IS_PALLAS);
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let v = frontend.circuit.fq_message_pass(self.wire);
            WireScalar::new(v)
        })
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

    pub fn print(self, label: &'static str) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            frontend.circuit.print(self.wire, label, "")
        })
    }
}

impl<P: PastaConfig> Zero for WireScalar<P> {
    fn zero() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            WireScalar::new(frontend.circuit.zero[P::SFID as usize])
        })
    }

    fn is_zero(&self) -> bool {
        self.wire == Self::zero().wire
    }
}

impl<P: PastaConfig> One for WireScalar<P> {
    fn one() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            WireScalar::new(frontend.circuit.one[P::SFID as usize])
        })
    }
}

impl<P: PastaConfig> Add for WireScalar<P> {
    type Output = WireScalar<P>;

    fn add(self, other: WireScalar<P>) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            WireScalar::new(frontend.circuit.add_gate(self.wire, other.wire))
        })
    }
}

impl<P: PastaConfig> AddAssign for WireScalar<P> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<P: PastaConfig> Sub for WireScalar<P> {
    type Output = WireScalar<P>;

    fn sub(self, other: WireScalar<P>) -> Self::Output {
        self + (-other)
    }
}

impl<P: PastaConfig> SubAssign for WireScalar<P> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<P: PastaConfig> Mul for WireScalar<P> {
    type Output = WireScalar<P>;

    fn mul(self, other: WireScalar<P>) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            WireScalar::new(frontend.circuit.mul_gate(self.wire, other.wire))
        })
    }
}

impl<P: PastaConfig> MulAssign for WireScalar<P> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<P: PastaConfig> Div for WireScalar<P> {
    type Output = WireScalar<P>;

    fn div(self, other: WireScalar<P>) -> Self::Output {
        self * other.inv()
    }
}

impl<P: PastaConfig> DivAssign for WireScalar<P> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
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
        ark_ff::{BigInt, BigInteger, Field, PrimeField},
        ark_std::{
            rand::{Rng, RngCore},
            test_rng,
        },
    };

    use crate::{
        frontend::{Call, FRONTEND, primitives::WireScalar},
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

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

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

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

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

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

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

    #[test]
    fn message_pass_fq() -> Result<()> {
        let rng = &mut test_rng();
        let x_u64 = 9u64;
        let x_fp = Fp::from(x_u64);
        let x_fq = Fq::from(x_u64);

        let x = WireScalar::<VestaConfig>::witness();
        let y = (x * x).fq_message_pass();
        y.output();

        let mut call = Call::new();

        call.witness(x, x_fq)?;

        let (fp_trace, fq_trace) = call.trace()?;
        let y_out = fp_trace.outputs[0];

        println!("{:?}", fp_trace);
        println!("{:?}", fq_trace);
        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        println!("{:?}", x_u64);
        println!("{:?}", x_fp);
        println!("{:?}", x_fq);
        assert_eq!(y_out, x_fp * x_fp);

        Ok(())
    }

    #[test]
    fn message_pass_fp() -> Result<()> {
        let rng = &mut test_rng();
        let x_u64 = ((1 << 17) - 1) as u64;
        let x_fp = Fp::from(x_u64);
        let x_fq = Fq::from(x_u64);

        let x = WireScalar::<PallasConfig>::witness();
        let (h, l) = x.fp_message_pass();
        h.output();
        l.output();

        let mut call = Call::new();

        call.witness(x, x_fp)?;

        let (fp_trace, fq_trace) = call.trace()?;
        let h_out = fq_trace.outputs[0];
        let l_out = fq_trace.outputs[1];

        println!("{:?}", fq_trace);
        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        let x_bits: Vec<u64> = x_fp
            .into_bigint()
            .to_bits_le()
            .into_iter()
            .map(|x| if x { 1 } else { 0 })
            .collect();
        let mut y_bits = h_out.into_bigint().to_bits_le();
        y_bits.insert(0, *l_out.into_bigint().to_bits_le().first().unwrap());
        y_bits.pop();
        let y_bits: Vec<u64> = y_bits.into_iter().map(|x| if x { 1 } else { 0 }).collect();
        assert_eq!(y_bits, x_bits);

        Ok(())
    }
}

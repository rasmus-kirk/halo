use std::{
    ops::{Add, AddAssign, Mul, Neg},
    str::FromStr,
};

use halo_group::{Affine, Fp, Fq, PastaConfig, PastaFE};

use crate::{
    circuit::Wire,
    frontend::{
        FRONTEND,
        primitives::{WireBool, WireScalar},
    },
};

#[derive(Clone, Copy, Debug)]
pub struct WireAffine<P: PastaConfig> {
    pub(crate) x: WireScalar<P::OtherCurve>,
    pub(crate) y: WireScalar<P::OtherCurve>,
}
impl<P: PastaConfig> WireAffine<P> {
    pub(crate) fn new(x: Wire, y: Wire) -> Self {
        Self {
            x: WireScalar::<P::OtherCurve>::new(x),
            y: WireScalar::<P::OtherCurve>::new(y),
        }
    }

    // TODO: Need to constrain
    pub fn witness() -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let x_wire = frontend.circuit.witness(P::BFID);
            let y_wire = frontend.circuit.witness(P::BFID);
            Self::new(x_wire, y_wire)
        })
    }

    pub fn public_input() -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let x_wire = frontend.circuit.public_input(P::BFID);
            let y_wire = frontend.circuit.public_input(P::BFID);
            Self::new(x_wire, y_wire)
        })
    }

    pub fn assert_eq(self, other: Self) {
        self.x.assert_eq(other.x);
        self.y.assert_eq(other.y);
    }

    pub fn equals(self, other: Self) -> WireBool<P::OtherCurve> {
        let x_eq = self.x.equals(other.x);
        let y_eq = self.y.equals(other.y);
        x_eq & y_eq
    }

    pub fn constant(point: Affine<P>) -> Self {
        assert!(point.is_on_curve());
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            if P::IS_PALLAS {
                let x_fe = PastaFE::new(
                    P::basefield_into_bigint(point.x),
                    Some(halo_group::PastaFieldId::Fq),
                );
                let y_fe = PastaFE::new(
                    P::basefield_into_bigint(point.y),
                    Some(halo_group::PastaFieldId::Fq),
                );
                let x_wire = frontend.circuit.constant(x_fe);
                let y_wire = frontend.circuit.constant(y_fe);
                Self::new(x_wire, y_wire)
            } else {
                let x_fe = PastaFE::new(
                    P::basefield_into_bigint(point.x),
                    Some(halo_group::PastaFieldId::Fp),
                );
                let y_fe = PastaFE::new(
                    P::basefield_into_bigint(point.y),
                    Some(halo_group::PastaFieldId::Fp),
                );
                let x_wire = frontend.circuit.constant(x_fe);
                let y_wire = frontend.circuit.constant(y_fe);
                Self::new(x_wire, y_wire)
            }
        })
    }

    pub fn identity() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            let x_wire = frontend.circuit.zero[P::BFID as usize];
            let y_wire = frontend.circuit.zero[P::BFID as usize];
            Self::new(x_wire, y_wire)
        })
    }

    pub fn generator() -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();

            if P::IS_PALLAS {
                let x = Fq::from_str(
                    "28948022309329048855892746252171976963363056481941560715954676764349967630336",
                )
                .unwrap();
                let y = Fq::from(2u64);
                let x_wire = frontend.circuit.constant(x.into());
                let y_wire = frontend.circuit.constant(y.into());
                Self::new(x_wire, y_wire)
            } else {
                let x = Fp::from_str(
                    "28948022309329048855892746252171976963363056481941647379679742748393362948096",
                )
                .unwrap();
                let y = Fp::from(2u64);
                let x_wire = frontend.circuit.constant(x.into());
                let y_wire = frontend.circuit.constant(y.into());
                Self::new(x_wire, y_wire)
            }
        })
    }

    pub fn output(self) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            frontend.circuit.output_gate(self.x.wire);
            frontend.circuit.output_gate(self.y.wire);
        })
    }

    pub fn print(self, label: &'static str) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            frontend.circuit.print(self.x.wire, label, " (x)");
            frontend.circuit.print(self.y.wire, label, " (y)");
        })
    }
}

impl<P: PastaConfig> Add for WireAffine<P> {
    type Output = WireAffine<P>;

    fn add(self, other: WireAffine<P>) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let (x_wire, y_wire) = frontend
                .circuit
                .add_points((self.x.wire, self.y.wire), (other.x.wire, other.y.wire));
            WireAffine::new(x_wire, y_wire)
        })
    }
}

impl<P: PastaConfig> Mul<WireScalar<P>> for WireAffine<P> {
    type Output = WireAffine<P>;

    fn mul(self, other: WireScalar<P>) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();

            if P::IS_PALLAS {
                let (h, l) = frontend.circuit.fp_message_pass(other.wire);
                let (xs_wire, ys_wire) = frontend
                    .circuit
                    .scalar_mul_pallas((h, l), (self.x.wire, self.y.wire));
                WireAffine::new(xs_wire, ys_wire)
            } else {
                let v = frontend.circuit.fq_message_pass(other.wire);
                let (xs_wire, ys_wire) = frontend
                    .circuit
                    .scalar_mul_vesta(v, (self.x.wire, self.y.wire));
                WireAffine::new(xs_wire, ys_wire)
            }
        })
    }
}

impl<P: PastaConfig> AddAssign for WireAffine<P> {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

impl<P: PastaConfig> Neg for WireAffine<P> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let y_neg = frontend.circuit.neg_gate(self.y.wire);
            WireAffine::new(self.x.wire, y_neg)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anyhow::Result;
    use halo_group::PastaFieldId;
    use halo_group::ark_ec::AffineRepr;
    use halo_group::ark_ff::{BigInt, Zero};
    use halo_group::{
        Affine, Fp, Fq, PallasConfig, PastaAffine, PastaConfig, Point, Scalar, VestaConfig,
        ark_ec::{AdditiveGroup, CurveGroup},
        ark_ff::{BigInteger, Field, PrimeField, UniformRand},
        ark_std::{rand::Rng, test_rng},
    };

    use crate::{
        frontend::{
            Call,
            primitives::{WireAffine, WireScalar},
        },
        plonk::PlonkProof,
    };

    fn random_point<R: Rng>(rng: &mut R) -> Affine<PallasConfig> {
        Affine::<PallasConfig>::rand(rng)
    }

    #[test]
    fn add_random_points() -> Result<()> {
        let rng = &mut test_rng();

        let p_v = random_point(rng);
        let q_v = random_point(rng);

        let p = WireAffine::<PallasConfig>::constant(p_v);
        let q = WireAffine::<PallasConfig>::constant(q_v);
        let r = p.clone() + q.clone();
        r.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let rx = fq_trace.outputs[0];
        let ry = fq_trace.outputs[1];

        let expected_output = (p_v + q_v).into_affine();
        assert_eq!((rx, ry), (expected_output.x, expected_output.y));

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }

    #[test]
    fn generator() -> Result<()> {
        let rng = &mut test_rng();

        let p_v = Affine::<PallasConfig>::generator();
        let q_v = Affine::<VestaConfig>::generator();

        let p = WireAffine::<PallasConfig>::generator();
        let q = WireAffine::<VestaConfig>::generator();
        assert_eq!(p.x.wire.fid, PastaFieldId::Fq);
        assert_eq!(p.y.wire.fid, PastaFieldId::Fq);
        assert_eq!(q.x.wire.fid, PastaFieldId::Fp);
        assert_eq!(q.y.wire.fid, PastaFieldId::Fp);
        p.output();
        q.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;

        println!("{:?}", fp_trace.outputs);
        println!("{:?}", fq_trace.outputs);

        let px = fq_trace.outputs[0];
        let py = fq_trace.outputs[1];
        let qx = fp_trace.outputs[0];
        let qy = fp_trace.outputs[1];

        assert_eq!((px, py), (p_v.x, p_v.y));
        assert_eq!((qx, qy), (q_v.x, q_v.y));

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }

    #[test]
    fn add_left_identity() -> Result<()> {
        let rng = &mut test_rng();

        let p_v = random_point(rng);

        let p = WireAffine::<PallasConfig>::constant(p_v);
        let q = WireAffine::<PallasConfig>::identity();
        let r = p + q;
        r.output();

        let p_v_2 = PastaAffine::from(p_v);
        println!("{p_v:?}, {p_v_2:?}");

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let rx = fq_trace.outputs[0];
        let ry = fq_trace.outputs[1];

        println!("----- FP TRACE -----");
        println!("{:?}", fp_trace);

        println!("----- FQ TRACE -----");
        println!("{:?}", fq_trace);

        assert_eq!((rx, ry), (p_v.x, p_v.y));

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }

    #[test]
    fn add_right_identity() -> Result<()> {
        let rng = &mut test_rng();

        let q_v = random_point(rng);

        let p = WireAffine::<PallasConfig>::identity();
        let q = WireAffine::<PallasConfig>::constant(q_v);
        let r = p + q;
        r.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let rx = fq_trace.outputs[0];
        let ry = fq_trace.outputs[1];

        assert_eq!((rx, ry), (q_v.x, q_v.y));

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }

    #[test]
    fn add_double() -> Result<()> {
        let rng = &mut test_rng();

        let p_v = random_point(rng);

        let p = WireAffine::<PallasConfig>::constant(p_v);
        let r = p.clone() + p.clone();
        r.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let rx = fq_trace.outputs[0];
        let ry = fq_trace.outputs[1];

        let expected_point = (p_v + p_v).into_affine();
        assert_eq!((rx, ry), (expected_point.x, expected_point.y));

        println!("trace: {:?}", fp_trace);
        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }

    #[test]
    fn add_identity_identity() -> Result<()> {
        let rng = &mut test_rng();

        let p = WireAffine::<PallasConfig>::identity();
        let q = WireAffine::<PallasConfig>::identity();
        let r = p + q;
        r.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let rx = fq_trace.outputs[0];
        let ry = fq_trace.outputs[1];

        let expected_output = Affine::<PallasConfig>::identity();
        assert_eq!((rx, ry), (expected_output.x, expected_output.y));

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }

    #[test]
    fn add_neg() -> Result<()> {
        let rng = &mut test_rng();

        let p_v = random_point(rng);

        let p = WireAffine::<PallasConfig>::constant(p_v);
        let q = -p;
        let r = p + q;
        r.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let rx = fq_trace.outputs[0];
        let ry = fq_trace.outputs[1];

        let expected_output = Affine::<PallasConfig>::identity();
        assert_eq!((rx, ry), (expected_output.x, expected_output.y));

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }

    #[test]
    fn mul() -> Result<()> {
        let rng = &mut test_rng();

        let p_v = random_point(rng);
        // let x_v = Fq::rand(rng);
        let x_v = Fp::from(1048575u64);
        // let x_v_fp = Fp::from_bigint(x_v.into_bigint()).unwrap();
        let s_v = scalar_multiply_2(&x_v, p_v);
        println!("s_v: {:?}", s_v);

        let p = WireAffine::<PallasConfig>::constant(p_v);
        let s_expected = WireAffine::<PallasConfig>::constant(s_v);
        let x = WireScalar::constant(x_v);
        let s = p * x;
        s.assert_eq(s_expected);
        s.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let x_out: Fq = fq_trace.outputs[0];
        let y_out: Fq = fq_trace.outputs[1];
        println!("outs: {:?}", fq_trace.outputs);

        assert_eq!(x_out, s_v.x);
        assert_eq!(y_out, s_v.y);

        println!("{:?}", fp_trace);
        println!("{:?}", fq_trace);

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }

    #[test]
    fn mul_2() -> Result<()> {
        let rng = &mut test_rng();

        let x_v = Fp::from_str(
            "6867074246676488306851463630808536728354415714211655136906833663722499829998",
        )
        .unwrap();
        let p_x = Fq::from_str(
            "16752387948686341610547730880506948762988022158064797870253267427491271468441",
        )
        .unwrap();
        let p_y = Fq::from_str(
            "24103962019005494757060435852850040604014379072860896298542399601100599714602",
        )
        .unwrap();
        let p_v = Affine::new(p_x, p_y);

        // let x_v_fp = Fp::from_bigint(x_v.into_bigint()).unwrap();
        let s_v = scalar_multiply_2(&x_v, p_v);
        println!("s_v: {:?}", s_v);

        let p = WireAffine::<PallasConfig>::constant(p_v);
        let s_expected = WireAffine::<PallasConfig>::constant(s_v);
        let x = WireScalar::constant(x_v);
        let s = p * x;
        s.assert_eq(s_expected);
        s.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let x_out: Fq = fq_trace.outputs[0];
        let y_out: Fq = fq_trace.outputs[1];
        println!("outs: {:?}", fq_trace.outputs);

        assert_eq!(x_out, s_v.x);
        assert_eq!(y_out, s_v.y);

        println!("{:?}", fp_trace);
        println!("{:?}", fq_trace);

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }

    #[test]
    fn mul_fq() -> Result<()> {
        let rng = &mut test_rng();

        let p_v = Affine::<VestaConfig>::rand(rng);
        // let x_v = Fq::rand(rng);
        let x_v = Fq::rand(rng);
        // let x_v_fp = Fp::from_bigint(x_v.into_bigint()).unwrap();
        let s_v = scalar_multiply_2(&x_v, p_v);
        println!("s_v: {:?}", s_v);

        let p = WireAffine::<VestaConfig>::constant(p_v);
        let s_expected = WireAffine::<VestaConfig>::constant(s_v);
        let x = WireScalar::constant(x_v);
        let s = p * x;
        s.assert_eq(s_expected);
        s.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let x_out: Fp = fp_trace.outputs[0];
        let y_out: Fp = fp_trace.outputs[1];
        println!("outs: {:?}", fq_trace.outputs);

        assert_eq!(x_out, s_v.x);
        assert_eq!(y_out, s_v.y);

        println!("{:?}", fp_trace);
        println!("{:?}", fq_trace);

        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }

    fn scalar_multiply_2<P: PastaConfig>(x: &Scalar<P>, g: Affine<P>) -> Affine<P> {
        let mut acc: Point<P> = Affine::<P>::identity().into();
        let bits: Vec<bool> = (*x)
            .into_bigint()
            .to_bits_le()
            .iter()
            .rev()
            .copied()
            .collect();
        let is = (0..bits.len()).rev();
        for (bit, i) in bits.into_iter().zip(is) {
            // println!("i: {i} {bit}");
            let q = acc + acc;
            let r = q + g;
            let s = if bit { r } else { q };
            if i < 8 {
                println!("i: {i} {bit}");
                println!("acc_i: {acc}");
                println!("q: {q}");
                println!("r: {r}\n");
            }
            acc = s;
        }
        println!("acc: {acc}");
        println!("Done \n");
        acc.into_affine()
    }

    #[test]
    fn affine_scalar_params() {
        let rng = &mut test_rng();
        let p = (Point::<VestaConfig>::rand(rng)).into_affine();
        // let p = Affine::<VestaConfig>::identity();
        let xp = p.x;
        let yp = p.y;
        let b_k = rng.gen_bool(0.5);

        let inv0 = |x: Fp| {
            if x.is_zero() { Fp::ZERO } else { Fp::ONE / x }
        };

        let lambda = if !yp.is_zero() {
            (Fp::from(3) * xp.square()) / (Fp::from(2) * yp)
        } else {
            Fp::ZERO
        };

        let xq = lambda.square() - (Fp::from(2) * xp);
        let yq = lambda * (xp - xq) - yp;

        let q = (p + p).into_affine();
        let xq_expected = q.x;
        let yq_expected = q.y;

        assert_eq!(xq, xq_expected);
        assert_eq!(yq, yq_expected);

        let beta = inv0(xp);
        assert_eq!((Fp::ONE - xp * beta) * xq, Fp::zero());
        assert_eq!((Fp::ONE - xp * beta) * yq, Fp::zero());
        assert_eq!(
            Fp::from(2) * yp * lambda - Fp::from(3) * xp.square(),
            Fp::ZERO
        );
        assert_eq!(lambda.square() - (Fp::from(2) * xp) - xq, Fp::ZERO);
        assert_eq!(lambda * (xp - xq) - yp - yq, Fp::ZERO);

        let r = (p + q).into_affine();
        let xr = r.x;
        let yr = r.y;

        // let xr_calculated = ((yp - yq) / (xp - xq)).square() - xq - xp;
        // let yr_calculated = ((yp - yq) / (xp - xq)) * (xq - xr) - yq;
        // assert_eq!(xr, xr_calculated);
        // assert_eq!(yr, yr_calculated);

        let beta = inv0(xp);
        assert_eq!((Fp::ONE - xp * beta) * xr, Fp::zero());
        assert_eq!((Fp::ONE - xp * beta) * yr, Fp::zero());
        let c1 = (xr + xq + xp) * (xp - xq).square() - (yp - yq).square();
        let c2 = (yr + yq) * (xp - xq) - (yp - yq) * (xq - xr);
        assert_eq!(c1, Fp::ZERO);
        assert_eq!(c2, Fp::ZERO);

        let g = if b_k { r } else { q };

        let xg = g.x;
        let yg = g.y;

        let b = if b_k { Fp::ONE } else { Fp::ZERO };
        assert_eq!(xg - (b * xr + (Fp::ONE - b) * xq), Fp::ZERO);
        assert_eq!(yg - (b * yr + (Fp::ONE - b) * yq), Fp::ZERO);
    }

    #[test]
    fn affine_scalar_multiply_rust() {
        let rng = &mut test_rng();

        let g = Affine::<PallasConfig>::rand(rng);
        let x = Scalar::<PallasConfig>::rand(rng);
        let xg = scalar_multiply_2(&x, g);
        let expected_xg = g * x;
        assert_eq!(xg, expected_xg);
    }

    #[test]
    fn field_five_is_not_square() {
        let five = Fq::from(5u64);
        assert!(five.sqrt().is_none(), "5 should not be a square in Fq");

        let five = Fp::from(5u64);
        assert!(five.sqrt().is_none(), "5 should not be a square in Fq");
    }
}

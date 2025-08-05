use std::ops::{Add, AddAssign, Mul, Neg};

use halo_group::{Affine, PallasConfig, Point, ark_ff::UniformRand};

use crate::{
    circuit::Wire,
    frontend::{FRONTEND, field::Fq},
};

#[derive(Clone, Copy, Debug)]
pub struct CurvePoint {
    pub(crate) x: Fq,
    pub(crate) y: Fq,
}
impl CurvePoint {
    pub(crate) fn new(x: Wire, y: Wire) -> Self {
        Self {
            x: Fq::new(x),
            y: Fq::new(y),
        }
    }

    // TODO: Need to constrain
    pub fn witness() -> Self {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let x_wire = frontend.fq_circuit.witness_gate();
            let y_wire = frontend.fq_circuit.witness_gate();
            Self::new(x_wire, y_wire)
        })
    }

    pub fn constant(point: Affine<PallasConfig>) -> Self {
        assert!(point.is_on_curve());
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let x_wire = frontend.fq_circuit.constant_gate(point.x);
            let y_wire = frontend.fq_circuit.constant_gate(point.y);
            Self::new(x_wire, y_wire)
        })
    }

    pub fn identity() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            let x_wire = frontend.fq_circuit.zero;
            let y_wire = frontend.fq_circuit.zero;
            Self::new(x_wire, y_wire)
        })
    }

    pub fn output(self) {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            frontend.fq_circuit.output_gate(self.x.wire);
            frontend.fq_circuit.output_gate(self.y.wire);
        })
    }
}

impl Add for CurvePoint {
    type Output = CurvePoint;

    fn add(self, other: CurvePoint) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let (x_wire, y_wire) = frontend
                .fq_circuit
                .add_points((self.x.wire, self.y.wire), (other.x.wire, other.y.wire));
            CurvePoint::new(x_wire, y_wire)
        })
    }
}

impl AddAssign for CurvePoint {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs
    }
}

impl Neg for CurvePoint {
    type Output = Self;
    fn neg(self) -> Self::Output {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            let y_neg = frontend.fq_circuit.neg_gate(self.y.wire);
            CurvePoint::new(self.x.wire, y_neg)
        })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use halo_group::{
        Affine, PallasConfig,
        ark_ec::CurveGroup,
        ark_ff::UniformRand,
        ark_std::{rand::Rng, test_rng},
    };

    use crate::{
        frontend::{Call, curve::CurvePoint},
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

        let p = CurvePoint::constant(p_v);
        let q = CurvePoint::constant(q_v);
        let r = p.clone() + q.clone();
        r.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let rx = fq_trace.outputs[0];
        let ry = fq_trace.outputs[1];

        let expected_output = (p_v + q_v).into_affine();
        assert_eq!((rx, ry), (expected_output.x, expected_output.y));

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }

    #[test]
    fn add_left_identity() -> Result<()> {
        let rng = &mut test_rng();

        let p_v = random_point(rng);

        let p = CurvePoint::constant(p_v);
        let q = CurvePoint::identity();
        let r = p + q;
        r.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let rx = fq_trace.outputs[0];
        let ry = fq_trace.outputs[1];

        assert_eq!((rx, ry), (p_v.x, p_v.y));

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }

    #[test]
    fn add_right_identity() -> Result<()> {
        let rng = &mut test_rng();

        let q_v = random_point(rng);

        let p = CurvePoint::identity();
        let q = CurvePoint::constant(q_v);
        let r = p + q;
        r.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let rx = fq_trace.outputs[0];
        let ry = fq_trace.outputs[1];

        assert_eq!((rx, ry), (q_v.x, q_v.y));

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }

    #[test]
    fn add_double() -> Result<()> {
        let rng = &mut test_rng();

        let p_v = random_point(rng);

        let p = CurvePoint::constant(p_v);
        let r = p.clone() + p.clone();
        r.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let rx = fq_trace.outputs[0];
        let ry = fq_trace.outputs[1];

        let expected_point = (p_v + p_v).into_affine();
        assert_eq!((rx, ry), (expected_point.x, expected_point.y));

        println!("trace: {:?}", fp_trace);
        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }

    #[test]
    fn add_identity_identity() -> Result<()> {
        let rng = &mut test_rng();

        let p = CurvePoint::identity();
        let q = CurvePoint::identity();
        let r = p + q;
        r.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let rx = fq_trace.outputs[0];
        let ry = fq_trace.outputs[1];

        let expected_output = Affine::<PallasConfig>::identity();
        assert_eq!((rx, ry), (expected_output.x, expected_output.y));

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }

    #[test]
    fn add_neg() -> Result<()> {
        let rng = &mut test_rng();

        let p_v = random_point(rng);

        let p = CurvePoint::constant(p_v);
        let q = -p;
        let r = p + q;
        r.output();

        let call = Call::new();

        let (fp_trace, fq_trace) = call.trace()?;
        let rx = fq_trace.outputs[0];
        let ry = fq_trace.outputs[1];

        let expected_output = Affine::<PallasConfig>::identity();
        assert_eq!((rx, ry), (expected_output.x, expected_output.y));

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }
}

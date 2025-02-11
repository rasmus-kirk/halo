#![allow(non_snake_case)]

use bincode::{Decode, Encode};

use crate::{
    acc::{Accumulator, AccumulatorHiding},
    archive::{WrappedPoint, WrappedPoly, WrappedScalar},
    group::PallasPoint,
    pcdl::{EvalProof, Instance},
};

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct WrappedAccumulator {
    q: WrappedInstance,
    pi_V: WrappedAccHiding,
}

impl From<Accumulator> for WrappedAccumulator {
    fn from(value: Accumulator) -> Self {
        Self {
            q: value.q.into(),
            pi_V: value.pi_V.into(),
        }
    }
}

impl From<WrappedAccumulator> for Accumulator {
    fn from(value: WrappedAccumulator) -> Self {
        Self {
            q: value.q.into(),
            pi_V: value.pi_V.into(),
        }
    }
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct WrappedAccHiding {
    h: WrappedPoly,
    U: WrappedPoint,
    w: WrappedScalar,
}

impl From<AccumulatorHiding> for WrappedAccHiding {
    fn from(value: AccumulatorHiding) -> Self {
        Self {
            h: value.h.into(),
            U: value.U.into(),
            w: value.w.into(),
        }
    }
}

impl From<WrappedAccHiding> for AccumulatorHiding {
    fn from(value: WrappedAccHiding) -> Self {
        Self {
            h: value.h.into(),
            U: value.U.into(),
            w: value.w.into(),
        }
    }
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct WrappedInstance {
    c: WrappedPoint,
    d: u64,
    z: WrappedScalar,
    v: WrappedScalar,
    pi: WrappedEvalProof,
}

impl From<Instance> for WrappedInstance {
    fn from(value: Instance) -> Self {
        WrappedInstance {
            c: value.C.into(),
            d: (value.d as u64).into(),
            z: value.z.into(),
            v: value.v.into(),
            pi: value.pi.into(),
        }
    }
}

impl From<WrappedInstance> for Instance {
    fn from(value: WrappedInstance) -> Self {
        Instance {
            C: value.c.into(),
            d: value.d.try_into().unwrap(),
            z: value.z.into(),
            v: value.v.into(),
            pi: value.pi.into(),
        }
    }
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct WrappedEvalProof {
    ls: Vec<WrappedPoint>,
    rs: Vec<WrappedPoint>,
    u: WrappedPoint,
    c: WrappedScalar,
    c_bar: Option<WrappedPoint>,
    w_prime: Option<WrappedScalar>,
}

impl From<EvalProof> for WrappedEvalProof {
    fn from(value: EvalProof) -> Self {
        let (c_bar, w_prime) = match (value.C_bar, value.w_prime) {
            (Some(c), Some(w)) => (Some(c.into()), Some(w.into())),
            (None, None) => (None, None),
            (_, _) => panic!("invalid eval proof"),
        };
        let ls: Vec<WrappedPoint> = value.Ls.into_iter().map(|x| x.into()).collect();
        let rs: Vec<WrappedPoint> = value.Rs.into_iter().map(|x| x.into()).collect();
        WrappedEvalProof {
            ls,
            rs,
            u: value.U.into(),
            c: value.c.into(),
            c_bar,
            w_prime,
        }
    }
}

impl From<WrappedEvalProof> for EvalProof {
    fn from(value: WrappedEvalProof) -> Self {
        let (C_bar, w_prime) = match (value.c_bar, value.w_prime) {
            (Some(c), Some(w)) => (Some(c.into()), Some(w.into())),
            (None, None) => (None, None),
            (_, _) => panic!("invalid eval proof"),
        };
        let Ls: Vec<PallasPoint> = value.ls.into_iter().map(|x| x.into()).collect();
        let Rs: Vec<PallasPoint> = value.rs.into_iter().map(|x| x.into()).collect();
        EvalProof {
            Ls,
            Rs,
            U: value.u.into(),
            c: value.c.into(),
            C_bar,
            w_prime,
        }
    }
}

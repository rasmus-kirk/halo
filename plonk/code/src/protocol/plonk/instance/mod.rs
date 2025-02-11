pub mod many;

use crate::curve::{Point, Poly, Scalar};

use halo_accumulation::pcdl::{self, EvalProof};

use rand::rngs::ThreadRng;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    pub comm: Point,
    pub pi: EvalProof,
    pub ev: Option<Scalar>,
}

impl Instance {
    pub fn new(rng: &mut ThreadRng, poly: &Poly, ch: &Scalar, has_ev: bool) -> Self {
        let commit = &poly.commit();
        let ev = if has_ev {
            Some(poly.evaluate(ch))
        } else {
            None
        };
        Self {
            comm: *commit,
            pi: poly.open(rng, commit, ch),
            ev,
        }
    }

    pub fn new_from_comm(
        rng: &mut ThreadRng,
        poly: &Poly,
        ch: &Scalar,
        comm: &Point,
        has_ev: bool,
    ) -> Self {
        let ev = if has_ev {
            Some(poly.evaluate(ch))
        } else {
            None
        };
        Self {
            comm: *comm,
            pi: poly.open(rng, comm, ch),
            ev,
        }
    }

    pub fn check(&self, ch: &Scalar, ev: Option<&Scalar>) -> bool {
        let ev_val = match (ev, &self.ev) {
            (Some(ev), _) => ev,
            (_, Some(ev_self)) => ev_self,
            _ => return false,
        }
        .into();
        pcdl::check(
            &self.comm.into(),
            self.comm.into(),
            &ch.into(),
            &ev_val,
            self.pi.clone(),
        )
        .is_ok()
    }

    pub fn set_ev(&self, ev: &Scalar) -> Instance {
        Instance {
            comm: self.comm,
            pi: self.pi.clone(),
            ev: Some(*ev),
        }
    }
}

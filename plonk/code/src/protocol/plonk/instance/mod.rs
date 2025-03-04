pub mod many;

use crate::curve::{Point, Poly, Scalar};

use halo_accumulation::pcdl::{self, EvalProof};

use rand::Rng;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance<const EV: bool> {
    pub comm: Point,
    pub pi: EvalProof,
    pub ev: Option<Scalar>,
}

impl<const EV: bool> Instance<EV> {
    pub fn new<R: Rng>(rng: &mut R, poly: &Poly, ch: &Scalar) -> Self {
        let commit = &poly.commit();
        Self {
            comm: *commit,
            pi: poly.open(rng, commit, ch),
            ev: if EV { Some(poly.evaluate(ch)) } else { None },
        }
    }

    pub fn new_from_comm<R: Rng>(rng: &mut R, poly: &Poly, ch: &Scalar, comm: &Point) -> Self {
        Self {
            comm: *comm,
            pi: poly.open(rng, comm, ch),
            ev: if EV { Some(poly.evaluate(ch)) } else { None },
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

    pub fn set_ev(&self, ev: &Scalar) -> Instance<true> {
        Instance {
            comm: self.comm,
            pi: self.pi.clone(),
            ev: Some(*ev),
        }
    }
}

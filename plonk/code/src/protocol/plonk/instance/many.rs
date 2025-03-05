use super::Instance;
use crate::{
    curve::{Point, Poly, Scalar},
    util,
};

use rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instances<const N: usize>([Instance; N]);

impl<const N: usize> From<[Instance; N]> for Instances<N> {
    fn from(proofs: [Instance; N]) -> Self {
        Instances(proofs)
    }
}

impl<const N: usize> Instances<N> {
    pub fn new(rng: &mut ThreadRng, polys: &[Poly; N], ch: &Scalar, has_ev: bool) -> Self {
        util::map_fix(polys, |poly| Instance::new(rng, poly, ch, has_ev)).into()
    }

    pub fn unwrap(&self) -> &[Instance; N] {
        &self.0
    }

    pub fn new_from_comm<R: Rng>(
        rng: &mut R,
        polys: &[Poly; N],
        comm: &[Point; N],
        ch: &Scalar,
        has_ev: bool,
    ) -> Self {
        let xs = &util::zip_fix(polys, comm);
        util::map_fix(xs, |(poly, comm)| {
            Instance::new_from_comm(rng, poly, ch, comm, has_ev)
        })
        .into()
    }

    pub fn check(&self, ch: &Scalar) -> bool {
        self.0.iter().all(|p| p.check(ch, None))
    }

    pub fn set_ev_many(&self, ev: &Scalar) -> Self {
        util::map_fix(&self.0, |f| f.set_ev(ev)).into()
    }

    pub fn get_evs(&self) -> Option<[Scalar; N]> {
        let slices = util::map_fix(&self.0, |p| p.ev);
        if slices.iter().all(|x| x.is_some()) {
            Some(util::map_fix(&slices, |x| x.unwrap()))
        } else {
            None
        }
    }

    pub fn get_comms(&self) -> [Point; N] {
        util::map_fix(&self.0, |p| p.comm)
    }
}

use super::Instance;
use crate::{
    curve::{Point, Poly, Scalar},
    util,
};

use rand::rngs::ThreadRng;

pub struct Instances<const N: usize, const EV: bool>([Instance<EV>; N]);

impl<const N: usize, const EV: bool> From<[Instance<EV>; N]> for Instances<N, EV> {
    fn from(proofs: [Instance<EV>; N]) -> Self {
        Instances(proofs)
    }
}

impl<const N: usize, const EV: bool> Instances<N, EV> {
    pub fn new(rng: &mut ThreadRng, polys: &[Poly; N], ch: &Scalar) -> Self {
        util::map_fix(polys, |poly| Instance::new(rng, poly, ch)).into()
    }

    pub fn unwrap(&self) -> &[Instance<EV>; N] {
        &self.0
    }

    pub fn new_from_comm(
        rng: &mut ThreadRng,
        polys: &[Poly; N],
        comm: &[Point; N],
        ch: &Scalar,
    ) -> Self {
        let xs = &util::zip_fix(polys, comm);
        util::map_fix(xs, |(poly, comm)| {
            Instance::new_from_comm(rng, poly, ch, comm)
        })
        .into()
    }

    pub fn check(&self, ch: &Scalar) -> bool {
        self.0.iter().all(|p| p.check(ch, None))
    }

    pub fn set_ev_many(&self, ev: &Scalar) -> Instances<N, true> {
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

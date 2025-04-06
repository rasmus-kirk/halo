use super::{ConstraintID, Pos, Trace};
use crate::{arithmetizer::WireID, pcs::PCS, utils::misc::pair_app};

use ark_ec::short_weierstrass::SWCurveConfig;
use bimap::BiMap;
use std::collections::HashMap;

impl<P: SWCurveConfig, PCST: PCS<P>> Trace<P, PCST> {
    fn eq_constraints(&self, other: &Self, enforced_map: &mut BiMap<WireID, WireID>) -> bool {
        if other.constraints.len() != self.constraints.len() {
            return false;
        }
        for (lhs, rhs) in self.constraints.iter().zip(other.constraints.iter()) {
            if !lhs.structural_eq(rhs, enforced_map) {
                return false;
            }
        }
        true
    }

    fn get_mapped_permutation(&self, enforced_map: &BiMap<WireID, WireID>) -> HashMap<Pos, Pos> {
        HashMap::<Pos, Pos>::from_iter(self.permutation.iter().map(pair_app(|pos: &Pos| {
            let new_id = enforced_map.get_by_left(&(pos.id()));
            let id = *new_id.unwrap_or(&(pos.id())) as ConstraintID;
            Pos::new(pos.slot, id)
        })))
    }

    fn eq_permutation(&self, other: &Self, enforced_map: &BiMap<WireID, WireID>) -> bool {
        let mapped_permutation = self.get_mapped_permutation(enforced_map);
        for (lhs, rhs) in other.permutation.iter() {
            match mapped_permutation.get(lhs) {
                Some(mapped) if mapped == rhs => continue,
                None if lhs == rhs => continue,
                _ => return false,
            }
        }
        true
    }
}

impl<P: SWCurveConfig, PCST: PCS<P>> PartialEq for Trace<P, PCST> {
    fn eq(&self, other: &Self) -> bool {
        let enforced_map = &mut BiMap::new();
        self.eq_constraints(other, enforced_map)
            && self.eq_permutation(other, enforced_map)
            && self.d == other.d
    }
}

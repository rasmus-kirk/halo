use super::{ConstraintID, Pos, Trace};
use crate::{arithmetizer::WireID, utils::misc::pair_app};

use bimap::BiMap;
use std::collections::HashMap;

impl Trace {
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
            let new_id = enforced_map.get_by_left(&(pos.id as usize));
            let id = *new_id.unwrap_or(&(pos.id as usize)) as ConstraintID;
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

impl PartialEq for Trace {
    fn eq(&self, other: &Self) -> bool {
        let enforced_map = &mut BiMap::new();
        self.eq_constraints(other, enforced_map)
            && self.eq_permutation(other, enforced_map)
            && self.d == other.d
    }
}

impl Eq for Trace {}

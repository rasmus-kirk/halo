use super::{value::Value, ConstraintID, Constraints, Pos, Trace};
use crate::{
    arithmetizer::{
        plookup::{opsets::EmptyOpSet, TableRegistry},
        PlookupEvsThunk,
    },
    circuit::{Circuit, CircuitPrivate, CircuitPublic},
    pcs::PCS,
    scheme::{Selectors, Slots},
    utils::{batch_fft, batch_p, misc::EnumIter},
};

use ark_ec::short_weierstrass::SWCurveConfig;

use std::collections::HashMap;

pub type TraceDeconstructed<P> = (
    usize,
    Vec<Constraints<P>>,
    [Vec<Pos>; Slots::COUNT],
    TableRegistry<P>,
);

impl<P: SWCurveConfig> Trace<P> {
    pub fn reconstruct((d, constraints, permutation_vals, table): TraceDeconstructed<P>) -> Self {
        let mut permutation = HashMap::new();
        for (slot_i, perms) in permutation_vals.iter().enumerate() {
            let slot = Slots::un_id(slot_i);
            for (i_, pos) in perms.iter().enumerate() {
                let i = (i_ + 1) as ConstraintID;
                permutation.insert(Pos::new(slot, i), *pos);
            }
        }
        Self {
            d,
            h: Default::default(),
            evals: Default::default(),
            constraints,
            permutation,
            table,
        }
    }

    pub fn to_circuit<PCST: PCS<P>>(self) -> Circuit<P> {
        let d = self.d;
        let mut ts = batch_fft(&self.gate_evals());
        let pip = ts.drain(Slots::COUNT + Selectors::COUNT..).next().unwrap();
        let qs = ts.drain(Slots::COUNT..).collect::<Vec<_>>();
        let ws = ts;
        let ps = batch_fft(&self.permutation_evals());
        let is = batch_fft(&self.identity_evals());

        let pip_com = PCST::commit(&pip.p, d, None);
        let qs_coms = PCST::batch_commit(batch_p(&qs), d, None);
        let ps_coms = PCST::batch_commit(batch_p(&ps), d, None);
        let ws_coms = PCST::batch_commit(batch_p(&ws), d, None);

        let plookup = PlookupEvsThunk::new(self.constraints, self.table);
        let x = CircuitPublic::new(d, self.h, qs, qs_coms, pip, pip_com, is, ps, ps_coms);
        let w = CircuitPrivate::new(ws, ws_coms, plookup);
        (x, w)
    }

    pub fn from_circuit((x, w): Circuit<P>) -> Self {
        let h = &x.h;
        let (expected_constraints, m) = h
            .iter()
            .try_fold((vec![], h.n()), |(mut acc, m), i| {
                let c = Constraints::new(
                    w.ws()
                        .chain(x.qs())
                        .chain(std::iter::once(&x.pip))
                        .map(|p| Value::AnonWire(p[i as usize]))
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                );
                if c == Constraints::default() {
                    Err((acc, i))
                } else {
                    acc.push(c);
                    Ok((acc, m))
                }
            })
            .unwrap_or_else(|res| res);

        let mut expected_permutation: [Vec<Pos>; Slots::COUNT] = [vec![], vec![], vec![]];
        (1..m).for_each(|i| {
            x.ps().zip(Slots::iter()).for_each(|(poly, slot)| {
                if let Some(pos) = Pos::from_scalar(poly[i as usize], h) {
                    expected_permutation[slot.id()].push(pos);
                }
            });
        });

        // TODO use IVC table eventually
        let table = TableRegistry::new::<EmptyOpSet>();
        Trace::reconstruct((x.d, expected_constraints, expected_permutation, table))
    }
}

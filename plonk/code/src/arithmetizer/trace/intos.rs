use super::{value::Value, ConstraintID, Constraints, Pos, Trace};
use crate::{
    arithmetizer::{plookup::TableRegistry, PlookupEvsThunk},
    circuit::{Circuit, CircuitPrivate, CircuitPublic},
    scheme::Slots,
    utils::poly::batch_interpolate,
};

use halo_accumulation::{
    group::{PallasPoint, PallasPoly},
    pcdl,
};

use ark_poly::Polynomial;
use std::collections::HashMap;

type Poly = PallasPoly;

impl
    From<(
        usize,
        Vec<Constraints>,
        [Vec<Pos>; Slots::COUNT],
        TableRegistry,
    )> for Trace
{
    fn from(
        (d, constraints, permutation_vals, table): (
            usize,
            Vec<Constraints>,
            [Vec<Pos>; Slots::COUNT],
            TableRegistry,
        ),
    ) -> Self {
        let mut permutation = HashMap::new();
        for (slot_i, perms) in permutation_vals.iter().enumerate() {
            let slot = Slots::from(slot_i);
            for (i_, pos) in perms.iter().enumerate() {
                let i = (i_ + 1) as ConstraintID;
                permutation.insert(Pos::new(slot, i), *pos);
            }
        }
        Trace {
            d,
            h: Default::default(),
            evals: Default::default(),
            constraints,
            permutation,
            table,
        }
    }
}

impl From<Trace> for Circuit {
    fn from(eval: Trace) -> Self {
        let d = eval.d;
        let (_ws, _qs, _pip) = eval.gate_polys();
        let (_is, _ps) = eval.copy_constraints();
        let ws = batch_interpolate(_ws.clone());
        let qs = batch_interpolate(_qs);
        let pip = _pip.interpolate();
        let is = batch_interpolate(_is.clone());
        let ps = batch_interpolate(_ps.clone());

        let pip_com = pcdl::commit(&pip, d, None);
        let qs_coms: Vec<PallasPoint> = qs.iter().map(|q| pcdl::commit(q, eval.d, None)).collect();
        let ps_coms: Vec<PallasPoint> = (0..Slots::COUNT)
            .map(|i| pcdl::commit(&ps[i], eval.d, None))
            .collect();

        ws.iter()
            .chain(qs.iter())
            .chain(ps.iter())
            .for_each(|p: &Poly| assert!(p.degree() <= d));

        let x = CircuitPublic {
            d: eval.d,
            h: eval.h,
            pip_com,
            qs_coms,
            ps_coms,
            pip,
            qs,
            is,
            _is,
            ps,
            _ps,
        };
        let w = CircuitPrivate {
            ws,
            _ws,
            plookup: PlookupEvsThunk::new(eval.constraints, eval.table),
        };
        (x, w)
    }
}

impl From<Circuit> for Trace {
    fn from((x, w): Circuit) -> Self {
        let h = &x.h;
        let mut m = h.n();
        let mut expected_constraints: Vec<Constraints> = vec![];
        for i in 1..m {
            let wi = &h.w(i);
            let polys =
                w.ws.iter()
                    .chain(x.qs.iter())
                    .chain(std::iter::once(&x.pip));
            let vs = polys
                .map(|p| Value::AnonWire(p.evaluate(wi)))
                .collect::<Vec<Value>>()
                .try_into()
                .unwrap();
            let c = Constraints::new(vs);
            if c == Constraints::default() {
                m = i;
                break;
            }
            expected_constraints.push(Constraints::new(vs));
            // construct Constraints
        }

        let mut expected_permutation: [Vec<Pos>; Slots::COUNT] = [vec![], vec![], vec![]];
        for i in 1..m {
            let wi = &h.w(i);
            for slot in Slots::iter() {
                let y = x.ps[slot as usize].evaluate(wi);
                if let Some(pos) = Pos::from_scalar(y, h) {
                    expected_permutation[slot as usize].push(pos);
                }
            }
            // if not exceeded then the permutation evaluations are valid
        }

        let table = TableRegistry::new();
        (x.d, expected_constraints, expected_permutation, table).into()
    }
}

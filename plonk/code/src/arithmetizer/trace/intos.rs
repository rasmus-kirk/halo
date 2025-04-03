use super::{value::Value, ConstraintID, Constraints, Pos, Trace};
use crate::{
    arithmetizer::{
        plookup::{EmptyOpSet, TableRegistry},
        PlookupEvsThunk,
    },
    circuit::{Circuit, CircuitPrivate, CircuitPublic},
    scheme::{Selectors, Slots, Terms},
    utils::{misc::EnumIter, poly::batch_interpolate, Poly},
};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::FpConfig;
use halo_accumulation::{group::PallasPoint, pcdl};

use ark_poly::Polynomial;
use std::collections::HashMap;

pub type TraceDeconstructed<const N: usize, C: FpConfig<N>> = (
    usize,
    Vec<Constraints<N, C>>,
    [Vec<Pos>; Slots::COUNT],
    TableRegistry<N, C>,
);

impl<const N: usize, C: FpConfig<N>, P: SWCurveConfig> From<TraceDeconstructed<N, C>>
    for Trace<N, C, P>
{
    fn from((d, constraints, permutation_vals, table): TraceDeconstructed<N, C>) -> Self {
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
            _marker: std::marker::PhantomData,
        }
    }
}

impl<const N: usize, C: FpConfig<N>, P: SWCurveConfig> From<Trace<N, C, P>> for Circuit<N, C, P> {
    fn from(eval: Trace<N, C, P>) -> Self {
        let d = eval.d;
        let _ts = eval.gate_polys();
        let (_is, _ps) = eval.copy_constraints();
        let ts = batch_interpolate(_ts.clone());
        let is = batch_interpolate(_is.clone());
        let ps = batch_interpolate(_ps.clone());

        let pip_com = pcdl::commit(&ts[Terms::PublicInputs.id()], d, None);
        let qs_com: Vec<PallasPoint> = ts[Slots::COUNT..Slots::COUNT + Selectors::COUNT]
            .iter()
            .map(|q| pcdl::commit(q, eval.d, None))
            .collect();
        let ps_com: Vec<PallasPoint> = (0..Slots::COUNT)
            .map(|i| pcdl::commit(&ps[i], eval.d, None))
            .collect();

        ts.iter()
            .chain(ps.iter())
            .chain(is.iter())
            .for_each(|p: &Poly<N, C>| assert!(p.degree() <= d));

        let pip = ts[Terms::PublicInputs.id()].clone();
        let ws = ts[..Slots::COUNT].to_vec();
        let _ws = _ts[..Slots::COUNT].to_vec();
        let qs = ts[Slots::COUNT..Slots::COUNT + Selectors::COUNT].to_vec();
        let x = CircuitPublic {
            d: eval.d,
            h: eval.h,
            qs,
            pip,
            is,
            _is,
            ps,
            _ps,
            pip_com,
            qs_com,
            ps_com,
        };
        let w = CircuitPrivate {
            ws,
            _ws,
            plookup: PlookupEvsThunk::new(eval.constraints, eval.table),
        };
        (x, w)
    }
}

impl<const N: usize, C: FpConfig<N>, P: SWCurveConfig> From<Circuit<N, C, P>> for Trace<N, C, P> {
    fn from((x, w): Circuit<N, C, P>) -> Self {
        let h = &x.h;
        let (expected_constraints, m) = h
            .iter()
            .try_fold((vec![], h.n()), |(mut acc, m), i| {
                let c = Constraints::new(
                    w.ws.iter()
                        .chain(x.qs.iter())
                        .chain(std::iter::once(&x.pip))
                        .map(|p| Value::AnonWire(p.evaluate(&h.w(i))))
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                );
                if c == Constraints::<N, C>::default() {
                    Err((acc, i))
                } else {
                    acc.push(c);
                    Ok((acc, m))
                }
            })
            .unwrap_or_else(|res| res);

        let mut expected_permutation: [Vec<Pos>; Slots::COUNT] = [vec![], vec![], vec![]];
        (1..m).for_each(|i| {
            let wi = &h.w(i);
            Slots::iter().for_each(|slot| {
                if let Some(pos) = Pos::from_scalar(x.ps[slot.id()].evaluate(wi), h) {
                    expected_permutation[slot.id()].push(pos);
                }
            });
        });

        // TODO use IVC table eventually
        let table = TableRegistry::new::<EmptyOpSet>();
        (x.d, expected_constraints, expected_permutation, table).into()
    }
}

mod constraints;
mod display;
mod eq;
mod errors;
mod intos;
mod pos;
mod value;

pub use constraints::Constraints;
pub use errors::TraceError;
pub use pos::Pos;
use value::Value;

use super::{
    arith_wire::ArithWire,
    cache::ArithWireCache,
    plookup::{PlookupOps, TableRegistry},
    WireID,
};
use crate::{
    scheme::{Slots, Terms, MAX_BLIND_TERMS},
    utils::{misc::EnumIter, Evals, Scalar},
    Coset,
};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::AdditiveGroup;

use educe::Educe;
use log::info;
use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::collections::HashMap;

/// A unique identifier for a constraint in the circuit.
type ConstraintID = u64;

/// Given a circuit arithmetization, output wires, and input wire values,
/// computes the circuit polynomials and permutation polynomials.
#[derive(Educe)]
#[educe(Default, Debug, Clone)]
pub struct Trace<P: SWCurveConfig> {
    h: Coset<P>,
    d: usize,
    evals: HashMap<WireID, Value<P>>,
    permutation: HashMap<Pos, Pos>,
    constraints: Vec<Constraints<P>>,
    table: TableRegistry<P>,
}

impl<P: SWCurveConfig> Trace<P> {
    pub fn new<R: Rng, Op: PlookupOps>(
        rng: &mut R,
        d: Option<usize>,
        wires: &ArithWireCache<Op, P>,
        input_values: Vec<Scalar<P>>,
        output_wires: Vec<WireID>,
    ) -> Result<Self, TraceError<Op, P>>
    where
        Standard: Distribution<Scalar<P>>,
    {
        let mut eval: Self = Trace {
            table: TableRegistry::new::<Op>(),
            ..Default::default()
        };

        for (wire, value) in input_values.into_iter().enumerate() {
            let value = &Value::new_wire(wire, value).set_bit_type(wires);
            eval.evals.insert(wire, *value);
            eval.bool_constraint(wires, wire, *value)?;
            eval.public_constraint(wires, wire, *value)?;
        }
        // TODO remember to predicate check for input wires too
        // fix input wire values

        for w in output_wires {
            eval.resolve(wires, w)?;
        }
        // compute wire values

        eval.compute_pos_permutation();
        // compute copy constraint values

        let n = eval.table.len() as u64;
        let m = eval.constraints.len() as u64;
        let lub = std::cmp::max(n, m) + MAX_BLIND_TERMS;
        eval.h = Coset::new(rng, lub, Slots::COUNT).ok_or(TraceError::FailedToMakeCoset(m))?;
        eval.d = d.unwrap_or(eval.h.n() as usize - 1);
        // compute coset

        Ok(eval)
    }

    // Evaluation computation -------------------------------------------------

    /// Look up for the wire's evaluation, otherwise start the evaluating.
    fn resolve<Op: PlookupOps>(
        &mut self,
        wires: &ArithWireCache<Op, P>,
        wire: WireID,
    ) -> Result<Value<P>, TraceError<Op, P>> {
        match self.evals.get(&wire) {
            Some(val) => Ok(*val),
            None => self.eval(wires, wire),
        }
    }

    /// Compute the values and constraints for the wire and all its dependencies.
    fn eval<Op: PlookupOps>(
        &mut self,
        wires: &ArithWireCache<Op, P>,
        wire: WireID,
    ) -> Result<Value<P>, TraceError<Op, P>> {
        let mut stack = vec![wire];
        while let Some(wire) = stack.pop() {
            if self.evals.contains_key(&wire) {
                continue;
            }
            let arith_wire = wires
                .to_arith(wire)
                .ok_or(TraceError::WireNotInCache(wire))?;
            if let Some((opt_constraint, value)) =
                self.eval_helper(&mut stack, wires, wire, arith_wire)?
            {
                if let Some(constraint) = opt_constraint {
                    if !constraint.is_satisfied() {
                        return Err(TraceError::constraint_not_satisfied(&constraint));
                    }
                    self.constraints.push(constraint);
                }
                self.bool_constraint(wires, wire, value)?;
                self.public_constraint(wires, wire, value)?;
                self.evals.insert(wire, value);
            } else {
                continue;
            }
        }
        self.get(wire).ok_or(TraceError::WireNotInCache(wire))
    }

    // Compute constraint and value for a wire, and update the stack used in eval.
    #[allow(clippy::type_complexity)]
    fn eval_helper<Op: PlookupOps>(
        &self,
        stack: &mut Vec<WireID>,
        wires: &ArithWireCache<Op, P>,
        wire: WireID,
        arith_wire: ArithWire<Op, P>,
    ) -> Result<Option<(Option<Constraints<P>>, Value<P>)>, TraceError<Op, P>> {
        match arith_wire {
            ArithWire::Input(id) => Err(TraceError::InputNotSet(id)),
            ArithWire::Constant(scalar, private) => {
                let id = wires
                    .lookup_const_id(scalar, private)
                    .ok_or(TraceError::ConstNotInCache(scalar))?;
                let val = Value::new_wire(id, scalar);
                let constraint = if !private {
                    Some(Constraints::constant(val))
                } else {
                    None
                };
                Ok(Some((constraint, val)))
            }
            ArithWire::AddGate(_, _)
            | ArithWire::MulGate(_, _)
            | ArithWire::Lookup(_, _, _)
            | ArithWire::Inv(_) => {
                let mut vals = Vec::new();
                for inp in arith_wire.inputs() {
                    match self.evals.get(&inp) {
                        Some(val) => vals.push(*val),
                        None => {
                            stack.push(wire);
                            stack.push(inp);
                            return Ok(None);
                        }
                    }
                }
                let out_val = self
                    .compute_output(&arith_wire, &vals)?
                    .set_id(wire)
                    .set_bit_type(wires);
                let constraint = Some(Self::compute_constraint(&arith_wire, vals, out_val));
                Ok(Some((constraint, out_val)))
            }
        }
    }

    /// Compute the output value of a gate operation.
    fn compute_output<Op: PlookupOps>(
        &self,
        arith_wire: &ArithWire<Op, P>,
        vals: &[Value<P>],
    ) -> Result<Value<P>, TraceError<Op, P>> {
        match arith_wire {
            ArithWire::AddGate(_, _) => Ok(vals[0] + vals[1]),
            ArithWire::MulGate(_, _) => Ok(vals[0] * vals[1]),
            &ArithWire::Lookup(op, _, _) => Ok(self.lookup_value(op, vals[0], vals[1]).unwrap()),
            ArithWire::Inv(id) => vals[0].inv().ok_or(TraceError::InverseZero(*id)),
            _ => unreachable!(),
        }
    }

    /// Compute the constraint for a gate operation.
    fn compute_constraint<Op: PlookupOps>(
        arith_wire: &ArithWire<Op, P>,
        vals: Vec<Value<P>>,
        out_val: Value<P>,
    ) -> Constraints<P> {
        match arith_wire {
            ArithWire::AddGate(_, _) => Constraints::add(vals[0], vals[1], out_val),
            ArithWire::MulGate(_, _) => Constraints::mul(vals[0], vals[1], out_val),
            ArithWire::Lookup(op, _, _) => Constraints::lookup(*op, vals[0], vals[1], out_val),
            ArithWire::Inv(_) => Constraints::mul_inv(vals[0], out_val),
            _ => unreachable!(),
        }
    }

    /// Get the value of a wire if it is computed
    pub fn get(&self, wire: WireID) -> Option<Value<P>> {
        self.evals.get(&wire).cloned()
    }

    /// Check and construct if the wire has a boolean constraint.
    fn bool_constraint<Op: PlookupOps>(
        &mut self,
        wires: &ArithWireCache<Op, P>,
        wire: WireID,
        value: Value<P>,
    ) -> Result<(), TraceError<Op, P>> {
        if value.is_bit() && wires.is_bool_constraint(wire) {
            let bool_constraint = Constraints::boolean(value);
            if !bool_constraint.is_satisfied() {
                return Err(TraceError::constraint_not_satisfied(&bool_constraint));
            }
            self.constraints.push(bool_constraint);
        }
        Ok(())
    }

    /// Check and construct if the wire has a public input constraint.
    fn public_constraint<Op: PlookupOps>(
        &mut self,
        wires: &ArithWireCache<Op, P>,
        wire: WireID,
        value: Value<P>,
    ) -> Result<(), TraceError<Op, P>> {
        if wires.is_public(wire) {
            let pub_constraint = Constraints::public_input(value);
            if !pub_constraint.is_satisfied() {
                return Err(TraceError::constraint_not_satisfied(&pub_constraint));
            }
            self.constraints.push(pub_constraint);
        }
        Ok(())
    }

    /// Look up the output value of the gate in the table.
    pub fn lookup_value<Op: PlookupOps>(
        &self,
        op: Op,
        a: Value<P>,
        b: Value<P>,
    ) -> Result<Value<P>, TraceError<Op, P>> {
        let a = a.to_fp();
        let b = b.to_fp();
        if let Some(c) = self.table.lookup(op, a, b) {
            Ok(Value::AnonWire(c))
        } else {
            Err(TraceError::LookupFailed(op, a, b))
        }
    }

    /// Compute the permutation of slot positions as per copy constraints.
    fn compute_pos_permutation(&mut self) {
        self.constraints
            .iter()
            .enumerate()
            .fold(
                HashMap::new(),
                |mut acc: HashMap<WireID, Vec<Pos>>, (i_, eqn)| {
                    let i = (i_ + 1) as ConstraintID;
                    Slots::iter().for_each(|slot| {
                        if let Value::Wire(wire, _, _) = eqn[Terms::F(slot)] {
                            let pos = Pos::new(slot, i);
                            acc.entry(wire).or_default().push(pos);
                        }
                    });
                    acc
                },
            )
            // compute equivalence class of wire indices
            .into_values()
            .for_each(|set| {
                let last_index = set.len() - 1;
                (0..last_index).for_each(|i| {
                    self.permutation.insert(set[i], set[i + 1]);
                });
                self.permutation.insert(set[last_index], set[0]);
            })
        // compute cyclic permutation per equivalence class
    }

    // Poly construction -------------------------------------------------------

    /// Compute the circuit polynomial evaluations.
    fn gate_evals(&self) -> Vec<Evals<P>> {
        let extend = self.h.n() as usize - self.constraints.len();
        Terms::iter()
            .map(|term| {
                Evals::<P>::new_sr(
                    self.constraints
                        .iter()
                        .map(|eqn| eqn[term].to_fp())
                        .chain(vec![Scalar::<P>::ZERO; extend])
                        .collect(),
                )
            })
            .collect()
    }

    /// Compute the permutation polynomial evaluations.
    fn permutation_evals(&self) -> Vec<Evals<P>> {
        Slots::iter()
            .map(|slot| {
                Evals::<P>::new_sr(
                    self.h
                        .iter()
                        .map(|id| {
                            let pos = Pos::new(slot, id);
                            self.permutation
                                .get(&pos)
                                .unwrap_or(&pos)
                                .to_scalar(&self.h)
                        })
                        .collect(),
                )
            })
            .collect()
    }

    /// Compute the identity permutation polynomial evaluations.
    fn identity_evals(&self) -> Vec<Evals<P>> {
        Slots::iter()
            .map(|slot| Evals::<P>::new_sr(self.h.vec_k(slot)))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use ark_ff::Field;
    use ark_pallas::PallasConfig;
    use halo_group::PallasScalar;

    use crate::{
        arithmetizer::{plookup::opsets::EmptyOpSet, Arithmetizer, Wire},
        pcs::PCSPallas,
    };

    use ark_ff::Field;
    use ark_pallas::PallasConfig;
    use halo_accumulation::group::PallasScalar;

    #[test]
    fn evaluator_values() {
        let rng = &mut rand::thread_rng();
        let [x, y] = Arithmetizer::<EmptyOpSet, PallasConfig>::build::<2>();
        let input_values = vec![1, 2];
        let output_wires = &[(x.clone() * x) * 3 + (y * 5) - 47];
        // build circuit

        let circuit = output_wires[0].arith().borrow();
        let input_scalars = input_values.into_iter().map(PallasScalar::from).collect();
        let output_ids = output_wires.iter().map(Wire::id).collect();
        let d = (1 << 10) - 1;
        let eval_res = Trace::new(rng, Some(d), &circuit.wires, input_scalars, output_ids);
        assert!(eval_res.is_ok());
        // construct evaluator

        let eval = eval_res.unwrap();
        let expected_constraints = vec![
            Constraints::mul(
                Value::new_wire(0, PallasScalar::ONE),
                Value::new_wire(0, PallasScalar::ONE),
                Value::new_wire(2, PallasScalar::ONE),
            ),
            Constraints::constant(Value::new_wire(3, 3.into())),
            Constraints::mul(
                Value::new_wire(2, PallasScalar::ONE),
                Value::new_wire(3, 3.into()),
                Value::new_wire(4, 3.into()),
            ),
            Constraints::constant(Value::new_wire(5, 5.into())),
            Constraints::mul(
                Value::new_wire(1, 2.into()),
                Value::new_wire(5, 5.into()),
                Value::new_wire(6, 10.into()),
            ),
            Constraints::add(
                Value::new_wire(4, 3.into()),
                Value::new_wire(6, 10.into()),
                Value::new_wire(7, 13.into()),
            ),
            Constraints::constant(Value::new_wire(8, (-47).into())),
            Constraints::add(
                Value::new_wire(7, 13.into()),
                Value::new_wire(8, (-47).into()),
                Value::new_wire(9, (-34).into()),
            ),
        ];
        let expected_permutation = [
            vec![
                Pos::new(Slots::B, 1),
                Pos::new(Slots::B, 3),
                Pos::new(Slots::C, 1),
                Pos::new(Slots::B, 5),
                Pos::new(Slots::A, 5),
                Pos::new(Slots::C, 3),
                Pos::new(Slots::B, 8),
                Pos::new(Slots::C, 6),
            ],
            vec![
                Pos::new(Slots::A, 1),
                Pos::new(Slots::B, 2),
                Pos::new(Slots::A, 2),
                Pos::new(Slots::B, 4),
                Pos::new(Slots::A, 4),
                Pos::new(Slots::C, 5),
                Pos::new(Slots::B, 7),
                Pos::new(Slots::A, 7),
            ],
            vec![
                Pos::new(Slots::A, 3),
                Pos::new(Slots::C, 2),
                Pos::new(Slots::A, 6),
                Pos::new(Slots::C, 4),
                Pos::new(Slots::B, 6),
                Pos::new(Slots::A, 8),
                Pos::new(Slots::C, 7),
                Pos::new(Slots::C, 8),
            ],
        ];
        let expected_eval = Trace::reconstruct((
            d,
            expected_constraints.clone(),
            expected_permutation.clone(),
            TableRegistry::new::<EmptyOpSet>(),
        ));
        assert!(eval == expected_eval);
        // structural equality

        let c = eval.to_circuit::<PCSPallas>();
        let eval2 = Trace::from_circuit(c);
        assert!(eval2 == expected_eval);
        // plonk structural equality
    }
}

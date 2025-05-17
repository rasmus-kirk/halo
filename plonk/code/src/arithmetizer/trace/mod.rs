mod constraints;
mod display;
mod eq;
mod errors;
mod intos;
mod pos;
mod value;

pub use constraints::Constraints;
use educe::Educe;
pub use errors::TraceError;
pub use pos::Pos;

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
use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::collections::HashMap;
use value::Value;

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
            if let Some((constraint, value)) =
                self.eval_helper(&mut stack, wires, wire, arith_wire)?
            {
                if !constraint.is_satisfied() {
                    return Err(TraceError::constraint_not_satisfied(&constraint));
                }
                self.constraints.push(constraint);
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
    ) -> Result<Option<(Constraints<P>, Value<P>)>, TraceError<Op, P>> {
        match arith_wire {
            ArithWire::Input(id) => Err(TraceError::InputNotSet(id)),
            ArithWire::Constant(scalar) => {
                let id = wires
                    .lookup_const_id(scalar)
                    .ok_or(TraceError::ConstNotInCache(scalar))?;
                let val = Value::new_wire(id, scalar);
                Ok(Some((Constraints::constant(val), val)))
            }
            ArithWire::AddGate(lhs, rhs)
            | ArithWire::MulGate(lhs, rhs)
            | ArithWire::Lookup(_, lhs, rhs) => {
                let mut vals = [Value::ZERO; 2];
                for (i, &inp) in [lhs, rhs].iter().enumerate() {
                    vals[i] = match self.evals.get(&inp) {
                        Some(val) => *val,
                        None => {
                            stack.push(wire);
                            stack.push(inp);
                            return Ok(None);
                        }
                    };
                }
                let [lhs_val, rhs_val] = vals;
                let out_val = self
                    .compute_output(&arith_wire, lhs_val, rhs_val)
                    .set_id(wire)
                    .set_bit_type(wires);
                let constraint = Self::compute_constraint(&arith_wire, lhs_val, rhs_val, out_val);
                Ok(Some((constraint, out_val)))
            }
        }
    }

    /// Compute the output value of a gate operation.
    fn compute_output<Op: PlookupOps>(
        &self,
        arith_wire: &ArithWire<Op, P>,
        lhs_val: Value<P>,
        rhs_val: Value<P>,
    ) -> Value<P> {
        match arith_wire {
            ArithWire::AddGate(_, _) => lhs_val + rhs_val,
            ArithWire::MulGate(_, _) => lhs_val * rhs_val,
            &ArithWire::Lookup(op, _, _) => self.lookup_value(op, lhs_val, rhs_val).unwrap(),
            _ => unreachable!(),
        }
    }

    /// Compute the constraint for a gate operation.
    fn compute_constraint<Op: PlookupOps>(
        arith_wire: &ArithWire<Op, P>,
        lhs_val: Value<P>,
        rhs_val: Value<P>,
        out_val: Value<P>,
    ) -> Constraints<P> {
        match arith_wire {
            ArithWire::AddGate(_, _) => Constraints::add(lhs_val, rhs_val, out_val),
            ArithWire::MulGate(_, _) => Constraints::mul(lhs_val, rhs_val, out_val),
            ArithWire::Lookup(op, _, _) => Constraints::lookup(*op, lhs_val, rhs_val, out_val),
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

    /// Compute the circuit polynomials.
    fn gate_polys(&self) -> Vec<Evals<P>> {
        let extend = self.h.n() as usize - self.constraints.len();
        Terms::iter()
            .map(|term| {
                let mut evals = self
                    .constraints
                    .iter()
                    .map(|eqn| eqn[term].to_fp())
                    .collect::<Vec<Scalar<P>>>();
                evals.insert(0, Scalar::<P>::ZERO);
                evals.extend(vec![Scalar::<P>::ZERO; extend]);
                Evals::<P>::from_vec_and_domain(evals, self.h.domain)
            })
            .collect()
    }

    /// Compute the permutation and identity permutation polynomials.
    fn copy_constraints(&self) -> Vec<Evals<P>> {
        Slots::iter()
            .map(|slot| {
                let mut evals: Vec<Scalar<P>> = self
                    .h
                    .iter()
                    .map(|id| {
                        let pos = Pos::new(slot, id);
                        self.permutation
                            .get(&pos)
                            .unwrap_or(&pos)
                            .to_scalar(&self.h)
                    })
                    .collect();
                evals = [self.h.k(slot)]
                    .into_iter()
                    .chain(evals)
                    .collect::<Vec<Scalar<P>>>();
                Evals::<P>::from_vec_and_domain(evals, self.h.domain)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use ark_ff::Field;
    use ark_pallas::PallasConfig;
    use halo_accumulation::group::PallasScalar;

    use crate::{
        arithmetizer::{plookup::opsets::EmptyOpSet, Arithmetizer, Wire},
        pcs::PCSPallas,
    };

    use super::*;

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

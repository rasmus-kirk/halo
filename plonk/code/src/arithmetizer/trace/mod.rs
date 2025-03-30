mod constraints;
mod display;
mod eq;
mod errors;
mod intos;
mod pos;
mod value;

use ark_poly::Evaluations;
pub use constraints::Constraints;
pub use errors::TraceError;
use halo_accumulation::group::PallasScalar;
pub use pos::Pos;

use super::{
    arith_wire::ArithWire,
    cache::ArithWireCache,
    plookup::{PlookupOps, TableRegistry},
    WireID,
};

use crate::{
    scheme::{Selectors, Slots, Terms, MAX_BLIND_TERMS},
    utils::scalar::batch_compute_evals,
    Coset,
};

use ark_ff::{AdditiveGroup, Field};
use log::info;
use rand::Rng;
use std::collections::HashMap;
use value::Value;

type Scalar = PallasScalar;

/// A unique identifier for a constraint in the circuit.
type ConstraintID = u64;

/// Given a circuit arithmetization, output wires, and input wire values,
/// computes the circuit polynomials and permutation polynomials.
#[derive(Debug, Clone)]
pub struct Trace {
    h: Coset,
    d: usize,
    evals: HashMap<WireID, Value>,
    permutation: HashMap<Pos, Pos>,
    constraints: Vec<Constraints>,
    table: TableRegistry,
}

impl Trace {
    pub fn new<R: Rng>(
        rng: &mut R,
        d: Option<usize>,
        wires: &ArithWireCache,
        input_values: Vec<Scalar>,
        output_wires: Vec<WireID>,
    ) -> Result<Self, TraceError> {
        let mut eval = Self {
            h: Default::default(),
            evals: HashMap::new(),
            permutation: HashMap::new(),
            constraints: vec![],
            table: TableRegistry::new(),
            d: 0,
        };
        info!("[A]: Remaining stack - {:?}", stacker::remaining_stack());

        for (wire, value) in input_values.into_iter().enumerate() {
            let value = Value::new_wire(wire, value).set_bit_type(wires);
            eval.evals.insert(wire, value);
            eval.bool_constraint(wires, wire, value)?;
            eval.public_constraint(wires, wire, value)?;
        }
        // fix input wire values
        info!("[B]: Remaining stack - {:?}", stacker::remaining_stack());

        for w in output_wires {
            eval.resolve(wires, w)?;
        }
        // compute wire values
        info!("[C]: Remaining stack - {:?}", stacker::remaining_stack());

        eval.compute_pos_permutation()?;
        // compute copy constraint values
        info!("[D]: Remaining stack - {:?}", stacker::remaining_stack());

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
    fn resolve(&mut self, wires: &ArithWireCache, wire: WireID) -> Result<Value, TraceError> {
        match self.evals.get(&wire) {
            Some(val) => Ok(*val),
            None => self.eval(wires, wire),
        }
    }

    /// Compute the values and constraints for the wire and all its dependencies.
    fn eval(&mut self, wires: &ArithWireCache, wire: WireID) -> Result<Value, TraceError> {
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
    fn eval_helper(
        &self,
        stack: &mut Vec<WireID>,
        wires: &ArithWireCache,
        wire: WireID,
        arith_wire: ArithWire,
    ) -> Result<Option<(Constraints, Value)>, TraceError> {
        match arith_wire {
            ArithWire::Input(id) => Err(TraceError::InputNotSet(id)),
            ArithWire::Constant(scalar) => {
                let id = wires
                    .lookup_const_id(scalar)
                    .ok_or(TraceError::ConstNotInCache(scalar))?;
                let val = Value::new_wire(id, scalar);
                Ok(Some((Constraints::constant(&val), val)))
            }
            ArithWire::AddGate(lhs, rhs)
            | ArithWire::MulGate(lhs, rhs)
            | ArithWire::Lookup(_, lhs, rhs) => {
                let mut vals: [Value; 2] = [Value::ZERO; 2];
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
                let [lhs_val, rhs_val] = &vals;
                let out_val = self
                    .compute_output(&arith_wire, lhs_val, rhs_val)
                    .set_id(wire)
                    .set_bit_type(wires);
                let constraint = Self::compute_constraint(&arith_wire, lhs_val, rhs_val, &out_val);
                Ok(Some((constraint, out_val)))
            }
        }
    }

    /// Compute the output value of a gate operation.
    fn compute_output(&self, arith_wire: &ArithWire, lhs_val: &Value, rhs_val: &Value) -> Value {
        match arith_wire {
            ArithWire::AddGate(_, _) => lhs_val + rhs_val,
            ArithWire::MulGate(_, _) => lhs_val * rhs_val,
            ArithWire::Lookup(op, _, _) => self.lookup_value(*op, lhs_val, rhs_val).unwrap(),
            _ => unreachable!(),
        }
    }

    /// Compute the constraint for a gate operation.
    fn compute_constraint(
        arith_wire: &ArithWire,
        lhs_val: &Value,
        rhs_val: &Value,
        out_val: &Value,
    ) -> Constraints {
        match arith_wire {
            ArithWire::AddGate(_, _) => Constraints::add(lhs_val, rhs_val, out_val),
            ArithWire::MulGate(_, _) => Constraints::mul(lhs_val, rhs_val, out_val),
            ArithWire::Lookup(op, _, _) => Constraints::lookup(*op, lhs_val, rhs_val, out_val),
            _ => unreachable!(),
        }
    }

    /// Get the value of a wire if it is computed
    pub fn get(&self, wire: WireID) -> Option<Value> {
        self.evals.get(&wire).cloned()
    }

    /// Check and construct if the wire has a boolean constraint.
    fn bool_constraint(
        &mut self,
        wires: &ArithWireCache,
        wire: WireID,
        value: Value,
    ) -> Result<(), TraceError> {
        if value.is_bit() && wires.is_bool_constraint(wire) {
            let bool_constraint = Constraints::boolean(&value);
            if !bool_constraint.is_satisfied() {
                return Err(TraceError::constraint_not_satisfied(&bool_constraint));
            }
            self.constraints.push(bool_constraint);
        }
        Ok(())
    }

    /// Check and construct if the wire has a public input constraint.
    fn public_constraint(
        &mut self,
        wires: &ArithWireCache,
        wire: WireID,
        value: Value,
    ) -> Result<(), TraceError> {
        if wires.is_public(wire) {
            let pub_constraint = Constraints::public_input(&value);
            if !pub_constraint.is_satisfied() {
                return Err(TraceError::constraint_not_satisfied(&pub_constraint));
            }
            self.constraints.push(pub_constraint);
        }
        Ok(())
    }

    /// Look up the output value of the gate in the table.
    pub fn lookup_value(&self, op: PlookupOps, a: &Value, b: &Value) -> Result<Value, TraceError> {
        let a = a.into();
        let b = b.into();
        if let Some(c) = self.table.lookup(op, &a, &b) {
            Ok(Value::AnonWire(c))
        } else {
            Err(TraceError::LookupFailed(op, a, b))
        }
    }

    /// Compute the permutation of slot positions as per copy constraints.
    fn compute_pos_permutation(&mut self) -> Result<(), TraceError> {
        let mut pos_sets: HashMap<WireID, Vec<Pos>> = HashMap::new();
        for (i_, eqn) in self.constraints.iter().enumerate() {
            let i = (i_ + 1) as ConstraintID;
            for slot in Slots::iter() {
                let pos = Pos::new(slot, i);
                if let Value::Wire(wire, _, _) = eqn[Terms::F(slot)] {
                    pos_sets.entry(wire).or_default().push(pos);
                }
            }
        }
        // compute set of positions per wire

        for set in pos_sets.values() {
            let last_index = set.len() - 1;
            for i in 0..last_index {
                self.permutation.insert(set[i], set[i + 1]);
            }
            self.permutation.insert(set[last_index], set[0]);
        }
        Ok(())
    }

    // Poly construction -------------------------------------------------------

    /// Compute the circuit polynomials.
    fn gate_polys(
        &self,
    ) -> (
        Vec<Evaluations<Scalar>>,
        Vec<Evaluations<Scalar>>,
        Evaluations<Scalar>,
    ) {
        let mut ws_evs: Vec<Vec<Scalar>> = vec![vec![Scalar::ZERO]; Slots::COUNT];
        let mut qs_evs: Vec<Vec<Scalar>> = vec![vec![Scalar::ZERO]; Selectors::COUNT];
        let mut pip_evs: Vec<Scalar> = vec![];
        for eqn in self.constraints.iter() {
            for slot in Slots::iter() {
                ws_evs[slot as usize].push(eqn[Terms::F(slot)].into());
            }
            for selector in Selectors::iter() {
                qs_evs[selector as usize].push(eqn[Terms::Q(selector)].into());
            }
            pip_evs.push(eqn[Terms::PublicInputs].into());
        }
        let extend = self.h.n() as usize - self.constraints.len();
        ws_evs
            .iter_mut()
            .chain(qs_evs.iter_mut())
            .chain(std::iter::once(&mut pip_evs))
            .for_each(|evs| {
                evs.extend(vec![Scalar::ZERO; extend]);
            });
        (
            batch_compute_evals(&self.h, ws_evs),
            batch_compute_evals(&self.h, qs_evs),
            Evaluations::from_vec_and_domain(pip_evs, self.h.domain),
        )
    }

    /// Compute the permutation and identity permutation polynomials.
    fn copy_constraints(&self) -> (Vec<Evaluations<Scalar>>, Vec<Evaluations<Scalar>>) {
        let mut sids_evs: Vec<Vec<Scalar>> = vec![vec![Scalar::ONE]; Slots::COUNT];
        let mut ss_evs: Vec<Vec<Scalar>> = vec![vec![Scalar::ONE]; Slots::COUNT];
        for i_ in 0..self.h.n() - 1 {
            let id = i_ + 1;
            for slot in Slots::iter() {
                let pos = Pos::new(slot, id);
                let pos_scalar = pos.to_scalar(&self.h);
                sids_evs[slot as usize].push(pos_scalar);
                ss_evs[slot as usize].push(match self.permutation.get(&pos) {
                    Some(y_pos) => y_pos.to_scalar(&self.h),
                    None => pos_scalar,
                });
            }
        }
        (
            batch_compute_evals(&self.h, sids_evs),
            batch_compute_evals(&self.h, ss_evs),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        arithmetizer::{Arithmetizer, Wire},
        circuit::Circuit,
    };

    use super::*;

    #[test]
    fn evaluator_values() {
        let rng = &mut rand::thread_rng();
        let [x, y] = Arithmetizer::build::<2>();
        let input_values = vec![1, 2];
        let output_wires: &[Wire; 1] = &[3 * (x.clone() * x) + (y * 5) - 47];
        // build circuit

        let circuit = output_wires[0].arith().borrow();
        let input_scalars = input_values.iter().map(|&v| v.into()).collect();
        let output_ids = output_wires.iter().map(Wire::id).collect();
        let d = (1 << 10) - 1;
        let eval_res = Trace::new(rng, Some(d), &circuit.wires, input_scalars, output_ids);
        assert!(eval_res.is_ok());
        // construct evaluator

        let eval = eval_res.unwrap();
        let expected_constraints = vec![
            Constraints::mul(
                &Value::new_wire(0, Scalar::ONE),
                &Value::new_wire(0, Scalar::ONE),
                &Value::new_wire(2, Scalar::ONE),
            ),
            Constraints::constant(&Value::new_wire(3, 3.into())),
            Constraints::mul(
                &Value::new_wire(2, Scalar::ONE),
                &Value::new_wire(3, 3.into()),
                &Value::new_wire(4, 3.into()),
            ),
            Constraints::constant(&Value::new_wire(5, 5.into())),
            Constraints::mul(
                &Value::new_wire(1, 2.into()),
                &Value::new_wire(5, 5.into()),
                &Value::new_wire(6, 10.into()),
            ),
            Constraints::add(
                &Value::new_wire(4, 3.into()),
                &Value::new_wire(6, 10.into()),
                &Value::new_wire(7, 13.into()),
            ),
            Constraints::constant(&Value::new_wire(8, (-47).into())),
            Constraints::add(
                &Value::new_wire(7, 13.into()),
                &Value::new_wire(8, (-47).into()),
                &Value::new_wire(9, (-34).into()),
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
        let expected_eval = (
            d,
            expected_constraints.clone(),
            expected_permutation.clone(),
            TableRegistry::new(),
        )
            .into();
        assert!(eval == expected_eval);
        // structural equality

        let c: Circuit = eval.into();
        let eval2: Trace = c.into();
        assert!(eval2 == expected_eval);
        // plonk structural equality
    }
}

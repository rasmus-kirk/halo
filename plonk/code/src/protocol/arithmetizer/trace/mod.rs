mod constraints;
mod display;
mod eq;
mod errors;
mod pos;
mod value;

use super::{
    arith_wire::ArithWire,
    cache::ArithWireCache,
    plonkup::{PlonkupOps, TableRegistry},
    PlonkupVecCompute, WireID,
};
use crate::{
    curve::{Coset, Poly, Scalar},
    protocol::{
        circuit::{Circuit, CircuitPrivate, CircuitPublic},
        scheme::{Slots, Terms, MAX_BLIND_TERMS},
    },
};
pub use constraints::Constraints;
pub use errors::TraceError;
pub use pos::Pos;
use value::Value;

use rand::Rng;
use std::collections::HashMap;

/// A unique identifier for a constraint in the circuit.
type ConstraintID = u64;

/// Given a circuit arithmetization, output wires, and input wire values,
/// computes the circuit polynomials and permutation polynomials.
#[derive(Debug, Clone)]
pub struct Trace {
    h: Coset,
    evals: HashMap<WireID, Value>,
    permutation: HashMap<Pos, Pos>,
    constraints: Vec<Constraints>,
    table: TableRegistry,
}

impl Trace {
    pub fn new<R: Rng>(
        rng: &mut R,
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
        };
        for (wire, value) in input_values.into_iter().enumerate() {
            let value = Value::new_wire(wire, value).set_bit_type(wires);
            eval.evals.insert(wire, value);
            eval.bool_constraint(wires, wire, value)?;
            eval.public_constraint(wires, wire, value)?;
        }
        // fix input wire values
        for w in output_wires {
            eval.resolve(wires, w)?;
        }
        // compute wire values

        eval.compute_pos_permutation()?;
        // compute copy constraint values

        let n = eval.table.len() as u64;
        let m = eval.constraints.len() as u64;
        eval.h = Coset::new(rng, std::cmp::max(n, m) + MAX_BLIND_TERMS, Slots::COUNT)
            .ok_or(TraceError::FailedToMakeCoset(m))?;
        // compute coset

        Ok(eval)
    }

    /// Compute the values and constraints for the wire and all its dependencies.
    fn eval(&mut self, wires: &ArithWireCache, wire: WireID) -> Result<Value, TraceError> {
        let arith_wire = match wires.to_arith(wire) {
            Some(wire) => wire,
            None => return Err(TraceError::WireNotInCache(wire)),
        };
        let (constraint, value) = match arith_wire {
            ArithWire::Input(id) => return Err(TraceError::InputNotSet(id)),
            ArithWire::Constant(scalar) => match wires.lookup_const_id(scalar) {
                Some(id) => {
                    let val = Value::new_wire(id, scalar);
                    (Constraints::constant(&val), val)
                }
                None => return Err(TraceError::ConstNotInCache(scalar)),
            },
            ArithWire::AddGate(lhs_, rhs_) => {
                let lhs = &self.resolve(wires, lhs_)?;
                let rhs = &self.resolve(wires, rhs_)?;
                let out = &(lhs + rhs).set_id(wire).set_bit_type(wires);
                (Constraints::add(lhs, rhs, out), *out)
            }
            ArithWire::MulGate(lhs_, rhs_) => {
                let lhs = &self.resolve(wires, lhs_)?;
                let rhs = &self.resolve(wires, rhs_)?;
                let out = &(lhs * rhs).set_id(wire).set_bit_type(wires);
                (Constraints::mul(lhs, rhs, out), *out)
            }
            ArithWire::Lookup(op, lhs_, rhs_) => {
                let lhs = &self.resolve(wires, lhs_)?;
                let rhs = &self.resolve(wires, rhs_)?;
                let out_ = self.lookup_value(op, lhs, rhs)?;
                let out = out_.set_id(wire).set_bit_type(wires);
                (Constraints::lookup(op, lhs, rhs, &out), out)
            }
        };
        self.constraints.push(constraint);
        if !constraint.is_satisfied() {
            return Err(TraceError::constraint_not_satisfied(&constraint));
        }
        self.bool_constraint(wires, wire, value)?;
        self.public_constraint(wires, wire, value)?;
        self.evals.insert(wire, value);
        Ok(value)
    }

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

    pub fn lookup_value(&self, op: PlonkupOps, a: &Value, b: &Value) -> Result<Value, TraceError> {
        let a = a.into();
        let b = b.into();
        if let Some(c) = self.table.lookup(op, &a, &b) {
            Ok(Value::AnonWire(c))
        } else {
            Err(TraceError::LookupFailed(op, a, b))
        }
    }

    /// Look up for the wire's evaluation, otherwise start the evaluating.
    fn resolve(&mut self, wires: &ArithWireCache, wire: WireID) -> Result<Value, TraceError> {
        match self.evals.get(&wire) {
            Some(val) => Ok(*val),
            None => self.eval(wires, wire),
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

    /// Compute the circuit polynomials.
    fn gate_polys(&self) -> [Poly; Terms::COUNT] {
        let mut points: [Vec<Scalar>; Terms::COUNT] = Default::default();
        for eqn in self.constraints.iter() {
            for term in Terms::iter() {
                points[Into::<usize>::into(term)].push(eqn[term].into());
            }
        }
        points.map(|ps| self.h.interpolate_zf(ps))
    }

    /// Compute the permutation and identity permutation polynomials.
    fn copy_constraints(&self) -> [Poly; Slots::COUNT * 2] {
        let mut points: [Vec<Scalar>; Slots::COUNT * 2] = Default::default();
        for ps in points.iter_mut() {
            ps.push(Scalar::ONE);
        }
        for i_ in 0..self.h.n() - 1 {
            let id = i_ + 1;
            for slot in Slots::iter() {
                let pos = Pos::new(slot, id);
                let pos_scalar = pos.to_scalar(&self.h);
                points[Slots::COUNT + slot as usize].push(pos_scalar);
                points[slot as usize].push(match self.permutation.get(&pos) {
                    Some(y_pos) => y_pos.to_scalar(&self.h),
                    None => pos_scalar,
                });
            }
        }
        points.map(|ps| self.h.interpolate(ps))
    }
}

impl From<(Vec<Constraints>, [Vec<Pos>; Slots::COUNT], TableRegistry)> for Trace {
    fn from(
        (constraints, permutation_vals, table): (
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
        let [a, b, c, ql, qr, qo, qm, qc, qk, j, pi] = eval.gate_polys();
        let [sa, sb, sc, sida, sidb, sidc] = eval.copy_constraints();
        // TODO: check d
        let x = CircuitPublic {
            h: eval.h.clone(),
            ql,
            qr,
            qo,
            qm,
            qc,
            pl_qk: qk,
            pl_j: j,
            pip: pi,
            sida,
            sidb,
            sidc,
            sa,
            sb,
            sc,
        };
        let w = CircuitPrivate {
            a,
            b,
            c,
            plonkup: PlonkupVecCompute::new(eval.h, eval.constraints, eval.table),
        };
        (x, w)
    }
}

impl From<Trace> for (Circuit, Trace) {
    fn from(eval: Trace) -> Self {
        (eval.clone().into(), eval)
    }
}

impl From<Circuit> for Trace {
    fn from((x, w): Circuit) -> Self {
        let h = &x.h;
        let mut m = h.n();
        let mut expected_constraints: Vec<Constraints> = vec![];
        for i in 1..m {
            let wi = &h.w(i);
            let polys = vec![
                &w.a, &w.b, &w.c, &x.ql, &x.qr, &x.qo, &x.qm, &x.qc, &x.pl_qk, &x.pl_j,
            ];
            let vs = polys
                .into_iter()
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
            let p = vec![&x.sa, &x.sb, &x.sc];
            for slot in Slots::iter() {
                let y = p[slot as usize].evaluate(wi);
                if let Some(pos) = Pos::from_scalar(y, h) {
                    expected_permutation[slot as usize].push(pos);
                }
            }
            // if not exceeded then the permutation evaluations are valid
        }

        let table = TableRegistry::new();
        (expected_constraints, expected_permutation, table).into()
    }
}

impl From<(&CircuitPublic, &CircuitPrivate)> for Trace {
    fn from((x, w): (&CircuitPublic, &CircuitPrivate)) -> Self {
        (x.clone(), w.clone()).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::protocol::{arithmetizer::Arithmetizer, arithmetizer::Wire};

    use super::*;

    #[test]
    fn evaluator_values() {
        let rng = &mut rand::thread_rng();
        let [x, y] = &Arithmetizer::build::<2>();
        let input_values = vec![1, 2];
        let output_wires: &[Wire; 1] = &[3 * (x * x) + (y * 5) - 47];
        // build circuit

        let circuit = output_wires[0].arith().borrow();
        let input_scalars = input_values.iter().map(|&v| v.into()).collect();
        let output_ids = output_wires.iter().map(Wire::id).collect();
        let eval_res = Trace::new(rng, &circuit.wires, input_scalars, output_ids);
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
        assert!(
            eval == (
                expected_constraints.clone(),
                expected_permutation.clone(),
                TableRegistry::new(),
            )
                .into()
        );
        // structural equality

        let c: Circuit = eval.into();
        let eval2: Trace = c.into();
        assert!(
            eval2
                == (
                    expected_constraints,
                    expected_permutation,
                    TableRegistry::new()
                )
                    .into()
        );
        // plonk structural equality
    }
}

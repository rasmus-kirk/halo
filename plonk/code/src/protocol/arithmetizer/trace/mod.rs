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
        scheme::{Selectors, Slots, Terms, MAX_BLIND_TERMS},
    },
};
pub use constraints::Constraints;
pub use errors::TraceError;
use halo_accumulation::{group::PallasPoint, pcdl};
use log::trace;
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
    d: usize,
    evals: HashMap<WireID, Value>,
    permutation: HashMap<Pos, Pos>,
    constraints: Vec<Constraints>,
    table: TableRegistry,
}

impl Trace {
    pub fn new<R: Rng>(
        rng: &mut R,
        d: usize,
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
            d,
        };
        trace!("[A]: Remaining stack - {:?}", stacker::remaining_stack());
        for (wire, value) in input_values.into_iter().enumerate() {
            let value = Value::new_wire(wire, value).set_bit_type(wires);
            eval.evals.insert(wire, value);
            eval.bool_constraint(wires, wire, value)?;
            eval.public_constraint(wires, wire, value)?;
        }
        trace!("[B]: Remaining stack - {:?}", stacker::remaining_stack());
        // fix input wire values
        for w in output_wires {
            eval.resolve(wires, w)?;
        }
        // compute wire values

        trace!("[C]: Remaining stack - {:?}", stacker::remaining_stack());
        eval.compute_pos_permutation()?;
        // compute copy constraint values

        trace!("[D]: Remaining stack - {:?}", stacker::remaining_stack());
        let n = eval.table.len() as u64;
        let m = eval.constraints.len() as u64;
        eval.h = Coset::new(rng, std::cmp::max(n, m) + MAX_BLIND_TERMS, Slots::COUNT)
            .ok_or(TraceError::FailedToMakeCoset(m))?;
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
        self.evals
            .get(&wire)
            .cloned()
            .ok_or(TraceError::WireNotInCache(wire))
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
                let lhs_val = match self.evals.get(&lhs) {
                    Some(val) => val,
                    None => {
                        stack.push(wire);
                        stack.push(lhs);
                        return Ok(None);
                    }
                };
                let rhs_val = match self.evals.get(&rhs) {
                    Some(val) => val,
                    None => {
                        stack.push(wire);
                        stack.push(rhs);
                        return Ok(None);
                    }
                };
                let out_val = match arith_wire {
                    ArithWire::AddGate(_, _) => lhs_val + rhs_val,
                    ArithWire::MulGate(_, _) => lhs_val * rhs_val,
                    ArithWire::Lookup(op, _, _) => self.lookup_value(op, lhs_val, rhs_val)?,
                    _ => unreachable!(),
                }
                .set_id(wire)
                .set_bit_type(wires);
                let constraint = match arith_wire {
                    ArithWire::AddGate(_, _) => Constraints::add(lhs_val, rhs_val, &out_val),
                    ArithWire::MulGate(_, _) => Constraints::mul(lhs_val, rhs_val, &out_val),
                    ArithWire::Lookup(op, _, _) => {
                        Constraints::lookup(op, lhs_val, rhs_val, &out_val)
                    }
                    _ => unreachable!(),
                };
                Ok(Some((constraint, out_val)))
            }
        }
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
    pub fn lookup_value(&self, op: PlonkupOps, a: &Value, b: &Value) -> Result<Value, TraceError> {
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
    fn gate_polys(&self) -> (Vec<Poly>, Vec<Poly>) {
        let mut points_slots: Vec<Vec<Scalar>> = vec![vec![]; Slots::COUNT];
        let mut points_selectors: Vec<Vec<Scalar>> = vec![vec![]; Selectors::COUNT + 1];
        for eqn in self.constraints.iter() {
            for slot in Slots::iter() {
                points_slots[slot as usize].push(eqn[Terms::F(slot)].into());
            }
            for selector in Selectors::iter() {
                points_selectors[selector as usize].push(eqn[Terms::Q(selector)].into());
            }
            points_selectors[Selectors::COUNT].push(eqn[Terms::PublicInputs].into());
        }
        (
            points_slots
                .into_iter()
                .map(|ps| self.h.interpolate_zf(ps))
                .collect(),
            points_selectors
                .into_iter()
                .map(|ps| self.h.interpolate_zf(ps))
                .collect(),
        )
    }

    /// Compute the permutation and identity permutation polynomials.
    fn copy_constraints(&self) -> Vec<Poly> {
        let mut points: Vec<Vec<Scalar>> = vec![vec![]; Slots::COUNT * 2];
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
        points
            .into_iter()
            .map(|ps| self.h.interpolate(ps))
            .collect()
    }
}

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
        let (mut gs, mut qs) = eval.gate_polys();
        let mut ss = eval.copy_constraints();

        let mut qs_comms: Vec<PallasPoint> = qs
            .iter()
            .map(|q| pcdl::commit(&q.poly, eval.d, None))
            .collect();
        let mut ss_comms: Vec<PallasPoint> = (0..Slots::COUNT)
            .map(|i| pcdl::commit(&ss[i].poly, eval.d, None))
            .collect();

        gs.iter()
            .chain(qs.iter())
            .chain(ss.iter())
            .for_each(|p: &Poly| assert!(p.degree() as usize <= d));

        let x = CircuitPublic {
            d: eval.d,
            h: eval.h.clone(),
            pip_com: qs_comms.pop().unwrap(),
            pl_j_com: qs_comms.pop().unwrap(),
            pl_qk_com: qs_comms.pop().unwrap(),
            qc_com: qs_comms.pop().unwrap(),
            qm_com: qs_comms.pop().unwrap(),
            qo_com: qs_comms.pop().unwrap(),
            qr_com: qs_comms.pop().unwrap(),
            ql_com: qs_comms.pop().unwrap(),
            sc_com: ss_comms.pop().unwrap(),
            sb_com: ss_comms.pop().unwrap(),
            sa_com: ss_comms.pop().unwrap(),
            pip: qs.pop().unwrap(),
            pl_j: qs.pop().unwrap(),
            pl_qk: qs.pop().unwrap(),
            qc: qs.pop().unwrap(),
            qm: qs.pop().unwrap(),
            qo: qs.pop().unwrap(),
            qr: qs.pop().unwrap(),
            ql: qs.pop().unwrap(),
            sidc: ss.pop().unwrap(),
            sidb: ss.pop().unwrap(),
            sida: ss.pop().unwrap(),
            sc: ss.pop().unwrap(),
            sb: ss.pop().unwrap(),
            sa: ss.pop().unwrap(),
        };
        let w = CircuitPrivate {
            c: gs.pop().unwrap(),
            b: gs.pop().unwrap(),
            a: gs.pop().unwrap(),
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
                &w.a, &w.b, &w.c, &x.ql, &x.qr, &x.qo, &x.qm, &x.qc, &x.pl_qk, &x.pl_j, &x.pip,
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
            let p = [&x.sa, &x.sb, &x.sc];
            for slot in Slots::iter() {
                let y = p[slot as usize].evaluate(wi);
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
        let eval_res = Trace::new(
            rng,
            (1 << 10) - 1,
            &circuit.wires,
            input_scalars,
            output_ids,
        );
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
                (1 << 10) - 1,
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
                    (1 << 10) - 1,
                    expected_constraints,
                    expected_permutation,
                    TableRegistry::new()
                )
                    .into()
        );
        // plonk structural equality
    }
}

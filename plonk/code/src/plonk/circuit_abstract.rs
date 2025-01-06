use super::{circuit_print::map_to_alphabet, circuit_protocol::Circuit, utils::Evals, Wire};
use ark_ff::{AdditiveGroup, Field};
use bimap::BiMap;
use halo_accumulation::group::PallasScalar;
use log::debug;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub type WireId = usize;
pub type EqClasses = Vec<Vec<(usize, usize)>>;
pub type GateVec = Vec<(Gate, WireId)>;
pub type ConstVec = Vec<(PallasScalar, WireId)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Gate {
    pub l: WireId,
    pub r: WireId,
    pub is_add: bool,
}

impl Gate {
    pub fn new(l: WireId, r: WireId, is_add: bool) -> Self {
        Gate { l, r, is_add }
    }
    pub fn add(l: WireId, r: WireId) -> Self {
        Gate { l, r, is_add: true }
    }
    pub fn mul(l: WireId, r: WireId) -> Self {
        Gate {
            l,
            r,
            is_add: false,
        }
    }
}

#[derive(Debug)]
pub struct AbstractCircuit<const L: usize> {
    uuid: usize,
    const_wires: BiMap<PallasScalar, WireId>,
    gates: BiMap<Gate, WireId>,
}

impl<const M: usize> Default for AbstractCircuit<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const M: usize> AbstractCircuit<M> {
    pub fn new() -> Self {
        AbstractCircuit {
            uuid: M,
            const_wires: BiMap::new(),
            gates: BiMap::new(),
        }
    }

    pub fn build() -> [Wire<M>; M] {
        let circuit_ref = Rc::new(RefCell::new(Self::new()));

        let mut wires = Vec::new();
        for wire_id in 0..M {
            let wire = Wire::new(wire_id, circuit_ref.clone(), map_to_alphabet(wire_id));
            wires.push(wire);
        }
        wires.try_into().unwrap()
    }

    pub fn n(&self) -> usize {
        self.gates.len()
    }

    // getters ----------------------------------------------------------------

    pub fn const_wires_iter(&self) -> impl Iterator<Item = (&PallasScalar, &WireId)> {
        self.const_wires.iter()
    }

    pub fn const_wires_get_by_right(&self, wire: &WireId) -> Option<&PallasScalar> {
        self.const_wires.get_by_right(wire)
    }

    pub fn gates_iter(&self) -> impl Iterator<Item = (&Gate, &WireId)> {
        self.gates.iter()
    }

    pub fn gates_right_values(&self) -> impl Iterator<Item = &WireId> {
        self.gates.right_values()
    }

    pub fn gates_get_by_right(&self, wire: &WireId) -> Option<&Gate> {
        self.gates.get_by_right(wire)
    }

    // circuit operations -------------------------------------------------------

    pub fn add(&mut self, l: WireId, r: WireId) -> WireId {
        debug!("add {} {}", l, r);
        self.gate(Gate::add(l, r))
    }

    pub fn mul(&mut self, l: WireId, r: WireId) -> WireId {
        debug!("mul {} {}", l, r);
        self.gate(Gate::mul(l, r))
    }

    pub fn add_c(&mut self, x: WireId, c: PallasScalar) -> WireId {
        debug!("add_c {} {}", x, c);
        self.gate_c(x, c, true)
    }

    pub fn mul_c(&mut self, x: WireId, c: PallasScalar) -> WireId {
        debug!("mul_c {} {}", x, c);
        self.gate_c(x, c, false)
    }

    fn gate(&mut self, g: Gate) -> WireId {
        debug!("gate {:?}", g);
        match self.gates.get_by_left(&g) {
            Option::Some(&wire) => wire,
            Option::None => {
                let uuid = self.uuid;
                self.gates.insert(g, uuid);
                self.uuid += 1;
                uuid
            }
        }
    }

    fn get_const_wire(&mut self, c: PallasScalar) -> WireId {
        self.const_wires
            .get_by_left(&c)
            .copied()
            .unwrap_or_else(|| {
                let uuid = self.uuid;
                self.const_wires.insert(c, uuid);
                self.uuid += 1;
                uuid
            })
    }

    fn gate_c(&mut self, x: WireId, c: PallasScalar, is_add: bool) -> WireId {
        debug!("gate_c {} {} {}", x, c, is_add);
        let r = self.get_const_wire(c);
        self.gate(Gate::new(x, r, is_add))
    }

    // circuit construction -----------------------------------------------------

    /// Given a gate wire, resolve its value recursively
    fn resolve_gate_vals(&self, gate: WireId, vals: &mut HashMap<WireId, PallasScalar>) {
        if !vals.contains_key(&gate) {
            let &Gate {
                l: l_,
                r: r_,
                is_add,
            } = self
                .gates_get_by_right(&gate)
                .expect("Circuit Error: Gate not found");
            self.resolve_gate_vals(l_, vals);
            self.resolve_gate_vals(r_, vals);
            let l = vals.get(&l_).expect("Circuit Error: Operand not found");
            let r = vals.get(&r_).expect("Circuit Error: Operand not found");
            vals.insert(gate, if is_add { l + r } else { l * r });
        }
    }

    /// Resolve all wire values given a set of secret `input_wire` values
    fn resolve_vals(&self, inputs: &[PallasScalar; M]) -> HashMap<WireId, PallasScalar> {
        let mut vals = HashMap::new();
        for (wire, &input) in inputs.iter().enumerate().take(M) {
            vals.insert(wire, input);
        }
        for (&val, &wire) in self.const_wires_iter() {
            vals.insert(wire, val);
        }
        for &gate in self.gates_right_values() {
            self.resolve_gate_vals(gate, &mut vals);
        }
        vals
    }

    /// Compute the evaluations for each a,b,c,ql,qr,qo,qm,qc polynomials
    fn compute_evals(
        &self,
        inputs: &[PallasScalar; M],
        gates: &[(Gate, WireId)],
        consts: &[(PallasScalar, WireId)],
    ) -> Evals<8> {
        const ONE: PallasScalar = PallasScalar::ONE;
        const ZERO: PallasScalar = PallasScalar::ZERO;
        let vals = self.resolve_vals(inputs);
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut c = Vec::new();
        let mut ql = Vec::new();
        let mut qr = Vec::new();
        let mut qo = Vec::new();
        let mut qm = Vec::new();
        let mut qc = Vec::new();
        for &input in inputs.iter().take(M) {
            a.push(input);
            b.push(ZERO);
            c.push(ZERO);
            ql.push(ONE);
            qr.push(ZERO);
            qo.push(ZERO);
            qm.push(ZERO);
            qc.push(-input);
        }
        for (val, _) in consts.iter() {
            a.push(*val);
            b.push(ZERO);
            c.push(ZERO);
            ql.push(ONE);
            qr.push(ZERO);
            qo.push(ZERO);
            qm.push(ZERO);
            qc.push(-*val);
        }
        for &(Gate { l, r, is_add }, o) in gates.iter() {
            a.push(*vals.get(&l).unwrap());
            b.push(*vals.get(&r).unwrap());
            c.push(*vals.get(&o).unwrap());
            ql.push(if is_add { ONE } else { ZERO });
            qr.push(if is_add { ONE } else { ZERO });
            qo.push(-ONE);
            qm.push(if is_add { ZERO } else { ONE });
            qc.push(ZERO);
        }
        let n = a.len();

        let order = n.next_power_of_two();
        for _ in n..order {
            a.push(ZERO);
            b.push(ZERO);
            c.push(ZERO);
            ql.push(ZERO);
            qr.push(ZERO);
            qo.push(ZERO);
            qm.push(ZERO);
            qc.push(ZERO);
        }
        [a, b, c, ql, qr, qo, qm, qc]
    }

    /// Construct concrete circuit from abstract circuit given public inputs
    pub fn prepare(&self, inputs: [PallasScalar; M]) -> Circuit<M> {
        let mut constraints_maps = vec![HashSet::new(); self.uuid];
        let mut gates = Vec::new();
        let mut consts = Vec::new();

        // public inputs
        for (i, map) in constraints_maps.iter_mut().enumerate().take(M) {
            map.insert((0, i));
        }

        // constants
        for (i, (&val, &wire)) in self.const_wires.iter().enumerate() {
            constraints_maps[wire].insert((0, M + i));
            consts.push((val, wire));
        }

        // gates
        let mut n = consts.len();
        for (i, (&Gate { l, r, is_add }, &o)) in self.gates.iter().enumerate() {
            constraints_maps[l].insert((0, M + n + i));
            constraints_maps[r].insert((1, M + n + i));
            constraints_maps[o].insert((2, M + n + i));
            gates.push((Gate { l, r, is_add }, o));
        }

        // equivalence classes
        n += M + self.gates.len();
        let order = n.next_power_of_two() as u64;
        let mut classes = Vec::with_capacity(n);
        for set in constraints_maps.iter() {
            classes.push(set.iter().cloned().collect());
        }

        // computed values
        let evals = self.compute_evals(&inputs, &gates, &consts);
        println!("evals len: {}", evals[0].len());
        Circuit::new(n as u64, order, classes, gates, consts, evals, inputs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit() {
        let [a, b] = &AbstractCircuit::<2>::build();

        // construct circuit
        let final_wire = ((a + b) * (a + b)) + 2;
        let circuit = final_wire.circuit().borrow_mut();
        let n = circuit.n();

        // test builder values
        assert_eq!(n, 3);
        assert_eq!([a.id(), b.id()], [0, 1]);

        // test gates
        assert_eq!(circuit.gates.len(), 3);
        assert_eq!(
            circuit.gates.get_by_left(&Gate {
                l: a.id(),
                r: b.id(),
                is_add: true
            }),
            Some(&2)
        );
        assert_eq!(
            circuit.gates.get_by_left(&Gate {
                l: 2,
                r: 2,
                is_add: false
            }),
            Some(&3)
        );
        assert_eq!(
            circuit.gates.get_by_left(&Gate {
                l: 3,
                r: 4,
                is_add: true
            }),
            Some(&5)
        );
    }
}

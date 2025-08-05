use std::{array, fmt};

use derivative::Derivative;
use halo_group::{
    PastaConfig, Scalar,
    ark_std::{One, Zero},
};
use petgraph::{
    Direction::{Incoming, Outgoing},
    dot::{Config, Dot},
    graph::{DiGraph, EdgeIndex, NodeIndex},
    visit::EdgeRef,
};

const NEW_WTINESS_POLYS: usize = 15;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlotId {
    row: usize,
    column: usize,
}

impl SlotId {
    pub fn new(row: usize, column: usize) -> Self {
        assert!(row != 0 && column != 0);
        Self { row, column }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn row_0_indexed(&self) -> usize {
        self.row - 1
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn column_0_indexed(&self) -> usize {
        self.column - 1
    }

    pub fn to_usize(&self, total_rows: usize) -> usize {
        self.row - 1 + (self.column - 1) * total_rows
    }

    pub fn from_usize(u: usize, n: usize) -> Self {
        SlotId {
            row: 1 + (u % n),
            column: 1 + (u / n),
        }
    }

    pub fn to_scalar<P: PastaConfig>(&self, total_rows: usize) -> Scalar<P> {
        P::scalar_from_u64((self.row + (self.column - 1) * total_rows) as u64)
    }
}

impl std::fmt::Debug for SlotId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

/// A wire is uniquely identified from its node-id and slot_id
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Wire {
    pub(crate) id: usize,
    /// The node identifier of the incoming gate
    pub(crate) node_idx: NodeIndex,
    /// An identifier for which output of the incoming gate this wire carries
    pub(crate) output_id: u32,
}

#[derive(Clone, Copy, Derivative, PartialEq, Eq)]
#[derivative(Debug(bound = "Scalar<P>: std::fmt::Debug"))]
pub(crate) enum GateType<P: PastaConfig> {
    Witness((), [Wire; 1]),
    PublicInput((), [Wire; 1]),
    Constant((), [Wire; 1], Scalar<P>),
    Output([Wire; 1], (), usize),
    Invert([Wire; 2], [Wire; 1]),
    Negate([Wire; 2], [Wire; 1]),
    AssertEq([Wire; 2], ()),
    Add([Wire; 2], [Wire; 1]),
    Multiply([Wire; 2], [Wire; 1]),
    PoseidonEnd([Wire; 3], [Wire; 3]),
    Poseidon([Wire; 3], [Wire; 3], [Scalar<P>; NEW_WTINESS_POLYS]),
    CurveAdd([Wire; 4], [Wire; 2]),
}

impl<P: PastaConfig> fmt::Display for GateType<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct CircuitSpec<P: PastaConfig> {
    pub(crate) graph: DiGraph<GateType<P>, Wire>,
    pub(crate) zero: Wire,
    pub(crate) one: Wire,
    pub(crate) witness_wire_count: usize,
    pub(crate) public_input_wire_count: usize,
    pub(crate) output_wire_count: usize,
    pub(crate) row_count: usize,
    pub(crate) wire_count: usize,
}

impl<P: PastaConfig> CircuitSpec<P> {
    fn new_without_zero_one() -> Self {
        let dummy_wire = Wire {
            id: 0,
            node_idx: NodeIndex::new(0),
            output_id: 0,
        };
        Self {
            graph: DiGraph::new(),
            zero: dummy_wire,
            one: dummy_wire,
            public_input_wire_count: 0,
            witness_wire_count: 0,
            output_wire_count: 0,
            wire_count: 0,
            row_count: 0,
        }
    }

    pub fn new() -> Self {
        let mut spec = Self::new_without_zero_one();
        let zero = spec.constant_gate(Scalar::<P>::zero());
        let one = spec.constant_gate(Scalar::<P>::one());
        spec.zero = zero;
        spec.one = one;
        spec
    }

    pub fn is_empty(&self) -> bool {
        self.wire_count == 0
    }

    // WARNING: This might be dangerous if the petgraph crate changes its internals
    fn next_node_index(&self) -> NodeIndex {
        let node_index: NodeIndex = NodeIndex::new(self.graph.node_count());
        node_index
    }

    fn new_wires<const N: usize>(&mut self) -> [Wire; N] {
        let mut new_wire = |i| {
            let id = self.wire_count;
            let node_idx = self.next_node_index();
            self.wire_count += 1;
            Wire {
                id,
                node_idx,
                output_id: i as u32,
            }
        };
        array::from_fn(|i| new_wire(i))
    }

    pub fn constant_gate(&mut self, c: P::ScalarField) -> Wire {
        println!("{}", P::SF::ID);
        self.row_count += 1;
        let out_wires = self.new_wires();
        let node = self.graph.add_node(GateType::Constant((), out_wires, c));
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        out_wires[0]
    }

    pub fn witness_gate(&mut self) -> Wire {
        self.witness_wire_count += 1;
        let out_wires = self.new_wires();
        let node = self.graph.add_node(GateType::Witness((), out_wires));
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        out_wires[0]
    }

    pub fn public_input_gate(&mut self) -> Wire {
        self.public_input_wire_count += 1;
        self.row_count += 1;
        let out_wires = self.new_wires();

        let node = self.graph.add_node(GateType::PublicInput((), out_wires));
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        out_wires[0]
    }

    pub fn add_gate(&mut self, left: Wire, right: Wire) -> Wire {
        self.row_count += 1;
        let out_wires = self.new_wires();

        let node = self.graph.add_node(GateType::Add([left, right], out_wires));
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        self.graph.add_edge(left.node_idx, node, left);
        self.graph.add_edge(right.node_idx, node, right);
        out_wires[0]
    }

    pub fn mul_gate(&mut self, left: Wire, right: Wire) -> Wire {
        self.row_count += 1;
        let in_wires = [left, right];
        let out_wires = self.new_wires();

        let node = self.graph.add_node(GateType::Multiply(in_wires, out_wires));
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        self.graph.add_edge(left.node_idx, node, left);
        self.graph.add_edge(right.node_idx, node, right);
        out_wires[0]
    }

    pub fn poseidon_gate(&mut self, round: usize, in_wires: [Wire; 3]) -> [Wire; 3] {
        self.row_count += 1;
        let out_wires = self.new_wires();

        let round_constants: [Scalar<P>; NEW_WTINESS_POLYS] =
            array::from_fn(|i| P::SCALAR_POSEIDON_ROUND_CONSTANTS[5 * round + i / 3][i % 3]);
        let gate_type = GateType::Poseidon(in_wires, out_wires, round_constants);
        let node = self.graph.add_node(gate_type);
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        for wire in in_wires {
            self.graph.add_edge(wire.node_idx, node, wire);
        }
        out_wires
    }

    pub fn poseidon_gate_finish(&mut self, in_wires: [Wire; 3]) -> [Wire; 3] {
        self.row_count += 1;
        let out_wires = self.new_wires();

        let gate_type = GateType::PoseidonEnd(in_wires, out_wires);
        let node = self.graph.add_node(gate_type);
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        for wire in in_wires {
            self.graph.add_edge(wire.node_idx, node, wire);
        }
        out_wires
    }

    pub fn add_points(&mut self, p: (Wire, Wire), q: (Wire, Wire)) -> (Wire, Wire) {
        self.row_count += 1;
        let out_wires = self.new_wires();
        let in_wires = [p.0, p.1, q.0, q.1];

        let node = self.graph.add_node(GateType::CurveAdd(in_wires, out_wires));
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        for wire in in_wires {
            self.graph.add_edge(wire.node_idx, node, wire);
        }
        out_wires.into()
    }

    pub fn neg_gate(&mut self, x: Wire) -> Wire {
        self.row_count += 1;
        let in_wires = [x, self.zero];
        let out_wires = self.new_wires();

        let node = self.graph.add_node(GateType::Negate(in_wires, out_wires));
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        self.graph.add_edge(x.node_idx, node, x);
        self.graph.add_edge(self.zero.node_idx, node, self.zero);
        out_wires[0]
    }

    pub fn inv_gate(&mut self, x: Wire) -> Wire {
        self.row_count += 1;
        let in_wires = [x, self.one];
        let out_wires = self.new_wires();

        let node = self.graph.add_node(GateType::Invert(in_wires, out_wires));
        self.graph.add_edge(x.node_idx, node, x);
        self.graph.add_edge(self.one.node_idx, node, self.one);

        out_wires[0]
    }

    pub fn assert_eq_gate(&mut self, left: Wire, right: Wire) {
        let node = self.graph.add_node(GateType::AssertEq([left, right], ()));
        self.graph.add_edge(left.node_idx, node, left);
        self.graph.add_edge(right.node_idx, node, right);
    }

    pub fn output_gate(&mut self, input: Wire) {
        let out_id = self.output_wire_count;
        self.output_wire_count += 1;

        let node = self.graph.add_node(GateType::Output([input], (), out_id));
        self.graph.add_edge(input.node_idx, node, input);
    }
}

impl<P: PastaConfig> std::fmt::Debug for CircuitSpec<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dot = Dot::with_config(&self.graph, &[Config::EdgeNoLabel]);
        write!(f, "{dot:?}")
    }
}

use std::array;

use halo_group::{PastaConfig, Scalar};
use petgraph::Direction::Incoming;
use petgraph::graph::{DiGraph, NodeIndex};

use crate::utils::WITNESS_POLYS;

#[derive(Clone, Copy, PartialEq, Eq)]
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

    pub fn column(&self) -> usize {
        self.column
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

    pub fn to_sigma<P: PastaConfig>(&self, total_rows: usize) -> Scalar<P> {
        P::scalar_from_u64((self.row + (self.column - 1) * total_rows) as u64)
    }
}

impl std::fmt::Debug for SlotId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

/// A wire is uniquely identified from its node-id and slot_id
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wire {
    pub(crate) id: usize,
    pub(crate) node_idx: NodeIndex,
    /// None for witness and public-input gates
    pub(crate) output_slot_id: Option<SlotId>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum GateType {
    Witness,
    PublicInput,
    Constant(SlotId),
    AssertEq,
    Add([SlotId; 3]),
    Multiply([SlotId; 3]),
}

pub struct CircuitSpec<P: PastaConfig> {
    pub(crate) graph: DiGraph<GateType, Wire>,
    pub(crate) witness_wires: Vec<Wire>,
    pub(crate) public_input_wires: Vec<Wire>,
    pub(crate) constant_wires: Vec<(Wire, P::ScalarField)>,
    pub(crate) row_count: usize,
    pub(crate) wire_count: usize,
}

impl<P: PastaConfig> CircuitSpec<P> {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            witness_wires: Vec::new(),
            public_input_wires: Vec::new(),
            constant_wires: Vec::new(),
            wire_count: 0,
            row_count: 0,
        }
    }

    pub(crate) fn get_gate_inputs(&self, node_idx: NodeIndex) -> Vec<Wire> {
        self.graph
            .edges_directed(node_idx, Incoming)
            .map(|x| x.weight().clone())
            .collect()
    }

    fn get_slot_ids(&mut self) -> [SlotId; WITNESS_POLYS] {
        let row = self.row_count;
        self.row_count += 1;
        let f = |column| SlotId::new(row + 1, column + 1);
        let slot_ids = array::from_fn(f);
        println!("{:?}", slot_ids);
        slot_ids
    }

    fn new_wire(&mut self, node_idx: NodeIndex, output_slot_id: Option<SlotId>) -> Wire {
        let wire_id = self.wire_count;
        self.wire_count += 1;
        Wire {
            id: wire_id,
            node_idx,
            output_slot_id,
        }
    }

    pub fn constant_gate(&mut self, c: P::ScalarField) -> Wire {
        let slot_ids = self.get_slot_ids();
        let constant_node = self.graph.add_node(GateType::Constant(slot_ids[0]));
        let output_wire = self.new_wire(constant_node, Some(slot_ids[0]));
        self.constant_wires.push((output_wire, c));
        output_wire
    }

    pub fn witness_gate(&mut self) -> Wire {
        let node = self.graph.add_node(GateType::Witness);
        let wire = self.new_wire(node, None);
        self.witness_wires.push(wire);
        wire
    }

    pub fn public_input_gate(&mut self) -> Wire {
        let slot_ids = self.get_slot_ids();
        let node = self.graph.add_node(GateType::PublicInput);
        let wire = self.new_wire(node, Some(slot_ids[0]));
        self.public_input_wires.push(wire);
        wire
    }

    pub fn add_gate(&mut self, left: Wire, right: Wire) -> Wire {
        let slot_ids = self.get_slot_ids();
        let node = self.graph.add_node(GateType::Add(slot_ids));
        self.graph.add_edge(left.node_idx, node, left);
        self.graph.add_edge(right.node_idx, node, right);
        self.new_wire(node, slot_ids.last().copied())
    }

    pub fn mul_gate(&mut self, left: Wire, right: Wire) -> Wire {
        let slot_ids = self.get_slot_ids();
        let node = self.graph.add_node(GateType::Multiply(slot_ids));
        self.graph.add_edge(left.node_idx, node, left);
        self.graph.add_edge(right.node_idx, node, right);
        self.new_wire(node, slot_ids.last().copied())
    }

    pub fn assert_eq_gate(&mut self, left: Wire, right: Wire) {
        let node = self.graph.add_node(GateType::AssertEq);
        self.graph.add_edge(left.node_idx, node, left);
        self.graph.add_edge(right.node_idx, node, right);
    }
}

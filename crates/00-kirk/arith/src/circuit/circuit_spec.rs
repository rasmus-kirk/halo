use std::{array, fmt};

use derivative::Derivative;
use halo_group::{PastaConfig, Scalar};
use petgraph::Direction::{Incoming, Outgoing};
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, EdgeIndex, NodeIndex};
use petgraph::visit::EdgeRef;

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

#[derive(Clone, Copy, Derivative)]
#[derivative(Debug(bound = "Scalar<P>: std::fmt::Debug"))]
pub(crate) enum GateType<P: PastaConfig> {
    Witness,
    PublicInput,
    Output,
    AssertEq,
    Poseidon([Scalar<P>; NEW_WTINESS_POLYS]),
    Constant(Scalar<P>),
    Add,
    Multiply,
}
impl<P: PastaConfig> fmt::Display for GateType<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GateType::Witness => write!(f, "Witness"),
            GateType::PublicInput => write!(f, "PublicInput"),
            GateType::Output => write!(f, "Output"),
            GateType::AssertEq => write!(f, "AssertEq"),
            GateType::Constant(_) => write!(f, "Constant"),
            GateType::Poseidon(_) => write!(f, "Poseidon"),
            GateType::Add => write!(f, "Add"),
            GateType::Multiply => write!(f, "Multiply"),
        }
    }
}

#[derive(Clone)]
pub struct CircuitSpec<P: PastaConfig> {
    pub(crate) graph: DiGraph<GateType<P>, Wire>,
    pub(crate) witness_wire_count: usize,
    pub(crate) public_input_wire_count: usize,
    pub(crate) row_count: usize,
    pub(crate) wire_count: usize,
}

impl<P: PastaConfig> CircuitSpec<P> {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            public_input_wire_count: 0,
            witness_wire_count: 0,
            wire_count: 0,
            row_count: 0,
        }
    }

    pub(crate) fn get_parent_node(&self, edge: EdgeIndex) -> NodeIndex {
        self.graph
            .edge_endpoints(edge)
            .expect("Edge not in graph!")
            .0
    }

    pub(crate) fn get_child_node(&self, edge: EdgeIndex) -> NodeIndex {
        self.graph
            .edge_endpoints(edge)
            .expect("Edge not in graph!")
            .1
    }

    pub(crate) fn get_gate_inputs<const N: usize>(
        &self,
        node_idx: NodeIndex,
    ) -> [(Wire, EdgeIndex); N] {
        let nodes: Vec<_> = self
            .graph
            .edges_directed(node_idx, Incoming)
            .map(|x| (x.weight().clone(), x.id()))
            .collect();
        assert_eq!(nodes.len(), N);
        array::from_fn(|i| nodes[i])
    }

    pub(crate) fn get_gate_outputs(&self, node_idx: NodeIndex) -> Vec<(Wire, EdgeIndex)> {
        self.graph
            .edges_directed(node_idx, Outgoing)
            .map(|x| (x.weight().clone(), x.id()))
            .collect()
    }

    fn new_wire(&mut self, node_idx: NodeIndex, output_id: u32) -> Wire {
        let id = self.wire_count;
        self.wire_count += 1;
        Wire {
            id,
            node_idx,
            output_id,
        }
    }

    pub fn constant_gate(&mut self, c: P::ScalarField) -> Wire {
        let constant_node = self.graph.add_node(GateType::Constant(c));
        self.row_count += 1;
        self.new_wire(constant_node, 0)
    }

    pub fn witness_gate(&mut self) -> Wire {
        self.witness_wire_count += 1;
        let node = self.graph.add_node(GateType::Witness);
        self.new_wire(node, 0)
    }

    pub fn public_input_gate(&mut self) -> Wire {
        self.public_input_wire_count += 1;
        self.row_count += 1;
        let node = self.graph.add_node(GateType::PublicInput);
        self.new_wire(node, 0)
    }

    pub fn add_gate(&mut self, left: Wire, right: Wire) -> Wire {
        let node = self.graph.add_node(GateType::Add);
        self.graph.add_edge(left.node_idx, node, left);
        self.graph.add_edge(right.node_idx, node, right);
        self.row_count += 1;
        self.new_wire(node, 0)
    }

    pub fn mul_gate(&mut self, left: Wire, right: Wire) -> Wire {
        let node = self.graph.add_node(GateType::Multiply);
        self.graph.add_edge(left.node_idx, node, left);
        self.graph.add_edge(right.node_idx, node, right);
        self.row_count += 1;
        self.new_wire(node, 0)
    }

    pub fn poseidon_gate(&mut self, round: usize, input_wires: [Wire; 3]) -> [Wire; 3] {
        self.row_count += 1;

        let round_constants: [Scalar<P>; NEW_WTINESS_POLYS] =
            array::from_fn(|i| P::SCALAR_POSEIDON_ROUND_CONSTANTS[i / 3][i % 3]);
        let node = self.graph.add_node(GateType::Poseidon(round_constants));

        for wire in input_wires {
            self.graph.add_edge(wire.node_idx, node, wire);
        }
        array::from_fn(|i| self.new_wire(node, i as u32))
    }

    pub fn assert_eq_gate(&mut self, left: Wire, right: Wire) {
        let node = self.graph.add_node(GateType::AssertEq);
        self.graph.add_edge(left.node_idx, node, left);
        self.graph.add_edge(right.node_idx, node, right);
    }

    pub fn output_gate(&mut self, input: Wire) {
        let node = self.graph.add_node(GateType::Output);
        self.graph.add_edge(input.node_idx, node, input);
    }
}

impl<P: PastaConfig> std::fmt::Debug for CircuitSpec<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dot = Dot::with_config(&self.graph, &[Config::EdgeNoLabel]);
        write!(f, "{dot:?}")
    }
}

use std::{
    array,
    fmt::{self, Debug},
    marker::PhantomData,
};

use halo_group::{
    Fp, Fq, PastaConfig, PastaFE, PastaFieldId, PastaScalar, Scalar,
    ark_ec::CurveConfig,
    ark_ff::{BigInteger, PrimeField},
    ark_std::{One, Zero},
};
use petgraph::{
    dot::{Config, Dot},
    graph::{DiGraph, NodeIndex},
};

use crate::utils::{ROUND_COEFF_POLYS, WITNESS_POLYS};

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
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Wire {
    pub(crate) id: usize,
    /// The node identifier of the incoming gate
    pub(crate) node_idx: NodeIndex,
    /// An identifier for which output of the incoming gate this wire carries
    pub(crate) output_id: u32,
    pub(crate) fid: PastaFieldId,
}
impl std::fmt::Debug for Wire {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({}.{})[{}]",
            self.fid,
            self.id,
            self.output_id,
            self.node_idx.index()
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum GateType {
    // Scalar Field
    Witness((), [Wire; 1]),
    PublicInput((), [Wire; 1]),
    Constant((), [Wire; 1], PastaFE),
    Output([Wire; 1], (), usize),
    ScalarMul([Wire; 4], [Wire; 2]),
    FpMessagePass([Wire; 1], [Wire; 2]),
    Invert([Wire; 2], [Wire; 1]),
    Negate([Wire; 2], [Wire; 1]),
    AssertEq([Wire; 2], ()),
    Add([Wire; 2], [Wire; 1]),
    Multiply([Wire; 2], [Wire; 1]),
    // Base Field
    PoseidonEnd([Wire; 3], [Wire; 3]),
    Poseidon([Wire; 3], [Wire; 3], [PastaFE; ROUND_COEFF_POLYS]),
    AffineAdd([Wire; 4], [Wire; 2]),
}
impl fmt::Display for GateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct CircuitSpec {
    pub(crate) graph: DiGraph<GateType, Wire>,
    pub(crate) zero: [Wire; 2],
    pub(crate) one: [Wire; 2],
    pub(crate) witness_wire_count: [usize; 2],
    pub(crate) public_input_wire_count: [usize; 2],
    pub(crate) message_pass_wire_count: [usize; 2],
    pub(crate) output_wire_count: [usize; 2],
    pub(crate) row_count: [usize; 2],
    pub(crate) wire_count: [usize; 2],
}

impl CircuitSpec {
    fn new_without_zero_one() -> Self {
        let dummy_wire = Wire {
            id: 0,
            fid: PastaFieldId::Fp,
            node_idx: NodeIndex::new(0),
            output_id: 0,
        };
        Self {
            graph: DiGraph::new(),
            zero: [dummy_wire, dummy_wire],
            one: [dummy_wire, dummy_wire],
            public_input_wire_count: [0, 0],
            message_pass_wire_count: [0, 0],
            witness_wire_count: [0, 0],
            output_wire_count: [0, 0],
            wire_count: [0, 0],
            row_count: [0, 0],
        }
    }

    pub fn new() -> Self {
        let mut spec = Self::new_without_zero_one();
        let fp_zero = spec.constant(PastaFE::zero(Some(PastaFieldId::Fp)));
        let fp_one = spec.constant(PastaFE::one(Some(PastaFieldId::Fp)));
        let fq_zero = spec.constant(PastaFE::zero(Some(PastaFieldId::Fq)));
        let fq_one = spec.constant(PastaFE::one(Some(PastaFieldId::Fq)));
        spec.zero = [fp_zero, fq_zero];
        spec.one = [fp_one, fq_one];
        spec
    }

    pub fn is_empty(&self) -> bool {
        self.wire_count[0] == 0 && self.wire_count[1] == 0
    }

    // WARNING: This might be dangerous if the petgraph crate changes its internals
    fn next_node_index(&self) -> NodeIndex {
        let node_index: NodeIndex = NodeIndex::new(self.graph.node_count());
        node_index
    }

    fn new_wires<const N: usize>(&mut self, fid: PastaFieldId) -> [Wire; N] {
        let mut new_wire = |i| {
            let id = self.wire_count[fid as usize];
            let node_idx = self.next_node_index();
            self.wire_count[fid as usize] += 1;
            Wire {
                id,
                fid,
                node_idx,
                output_id: i as u32,
            }
        };
        array::from_fn(|i| new_wire(i))
    }

    pub fn witness(&mut self, fid: PastaFieldId) -> Wire {
        self.witness_wire_count[fid as usize] += 1;

        let out_wires = self.new_wires(fid);

        let node = self.graph.add_node(GateType::Witness((), out_wires));

        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        out_wires[0]
    }

    pub fn public_input(&mut self, fid: PastaFieldId) -> Wire {
        self.public_input_wire_count[fid as usize] += 1;
        self.row_count[fid as usize] += 1;

        let out_wires = self.new_wires(fid);

        let node = self.graph.add_node(GateType::PublicInput((), out_wires));

        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        out_wires[0]
    }

    pub fn fp_witness(&mut self) -> Wire {
        self.witness(PastaFieldId::Fp)
    }

    pub fn fq_witness(&mut self) -> Wire {
        self.witness(PastaFieldId::Fq)
    }

    pub fn fp_public_input(&mut self) -> Wire {
        self.public_input(PastaFieldId::Fp)
    }

    pub fn fq_public_input(&mut self) -> Wire {
        self.public_input(PastaFieldId::Fq)
    }

    pub fn constant(&mut self, c: PastaFE) -> Wire {
        let fid = c.fid.unwrap();
        self.row_count[fid as usize] += 1;

        let out_wires = self.new_wires(fid);

        let node = self.graph.add_node(GateType::Constant((), out_wires, c));

        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        out_wires[0]
    }

    pub fn add(&mut self, left: Wire, right: Wire) -> Wire {
        let fid = left.fid;
        self.row_count[fid as usize] += 1;

        let in_wires = [left, right];
        let out_wires = self.new_wires(fid);

        let node = self.graph.add_node(GateType::Add(in_wires, out_wires));

        self.graph.add_edge(left.node_idx, node, left);
        self.graph.add_edge(right.node_idx, node, right);

        in_wires.iter().for_each(|x| assert_eq!(fid, x.fid));
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        out_wires[0]
    }

    pub fn mul(&mut self, left: Wire, right: Wire) -> Wire {
        let fid = left.fid;
        self.row_count[fid as usize] += 1;

        let in_wires = [left, right];
        let out_wires = self.new_wires(fid);

        let node = self.graph.add_node(GateType::Multiply(in_wires, out_wires));
        self.graph.add_edge(left.node_idx, node, left);
        self.graph.add_edge(right.node_idx, node, right);

        in_wires.iter().for_each(|x| assert_eq!(fid, x.fid));
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        out_wires[0]
    }

    pub fn poseidon(&mut self, round: usize, in_wires: [Wire; 3]) -> [Wire; 3] {
        let fid = in_wires[0].fid;
        self.row_count[fid as usize] += 1;

        let out_wires = self.new_wires(in_wires[0].fid);

        let round_constants: [PastaFE; ROUND_COEFF_POLYS] =
            array::from_fn(|i| fid.poseidon_round_constants()[5 * round + i / 3][i % 3]);
        let gate_type = GateType::Poseidon(in_wires, out_wires, round_constants);
        let node = self.graph.add_node(gate_type);

        for wire in in_wires {
            self.graph.add_edge(wire.node_idx, node, wire);
        }

        in_wires.iter().for_each(|x| assert_eq!(fid, x.fid));
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        out_wires
    }

    pub fn poseidon_finish(&mut self, in_wires: [Wire; 3]) -> [Wire; 3] {
        let fid = in_wires[0].fid;
        self.row_count[fid as usize] += 1;

        let out_wires = self.new_wires(fid);

        let gate_type = GateType::PoseidonEnd(in_wires, out_wires);
        let node = self.graph.add_node(gate_type);
        for wire in in_wires {
            self.graph.add_edge(wire.node_idx, node, wire);
        }

        in_wires.iter().for_each(|x| assert_eq!(fid, x.fid));
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        out_wires
    }

    pub fn add_points(&mut self, p: (Wire, Wire), q: (Wire, Wire)) -> (Wire, Wire) {
        let fid = p.0.fid;
        self.row_count[fid as usize] += 1;

        let in_wires = [p.0, p.1, q.0, q.1];
        let out_wires = self.new_wires(p.0.fid);

        let node = self
            .graph
            .add_node(GateType::AffineAdd(in_wires, out_wires));
        for wire in in_wires {
            self.graph.add_edge(wire.node_idx, node, wire);
        }

        in_wires.iter().for_each(|x| assert_eq!(fid, x.fid));
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        out_wires.into()
    }

    pub fn neg_gate(&mut self, x: Wire) -> Wire {
        let fid = x.fid;
        let zero = self.zero[fid as usize];
        self.row_count[fid as usize] += 1;

        let in_wires = [x, zero];
        let out_wires = self.new_wires(fid);

        let node = self.graph.add_node(GateType::Negate(in_wires, out_wires));

        self.graph.add_edge(x.node_idx, node, x);
        self.graph.add_edge(zero.node_idx, node, zero);

        in_wires.iter().for_each(|x| assert_eq!(fid, x.fid));
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        out_wires[0]
    }

    pub fn inv(&mut self, x: Wire) -> Wire {
        let fid = x.fid;
        let one = self.one[fid as usize];
        self.row_count[fid as usize] += 1;

        let in_wires = [x, one];
        let out_wires = self.new_wires(fid);

        let node = self.graph.add_node(GateType::Invert(in_wires, out_wires));
        self.graph.add_edge(x.node_idx, node, x);
        self.graph.add_edge(one.node_idx, node, one);

        in_wires.iter().for_each(|x| assert_eq!(fid, x.fid));
        out_wires.iter().for_each(|x| assert_eq!(x.node_idx, node));

        out_wires[0]
    }

    pub fn assert_eq_gate(&mut self, left: Wire, right: Wire) {
        let node = self.graph.add_node(GateType::AssertEq([left, right], ()));
        self.graph.add_edge(left.node_idx, node, left);
        self.graph.add_edge(right.node_idx, node, right);

        assert_eq!(left.fid, right.fid);
    }

    pub fn scalar_mul(&mut self, scalar: (Wire, Wire), point: (Wire, Wire)) -> (Wire, Wire) {
        let fid = point.0.fid;
        // TODO: ceil(lg_p) = 255, so this should be 255 + 1
        // 255 rows needed for scalar 1 zero row at the end
        self.row_count[fid as usize] += 256 + 1;

        let out_wires = self.new_wires(fid);

        let gate_type = GateType::ScalarMul([scalar.0, scalar.1, point.0, point.1], out_wires);
        let node = self.graph.add_node(gate_type);
        self.graph.add_edge(point.0.node_idx, node, point.0);
        self.graph.add_edge(point.1.node_idx, node, point.1);

        assert_eq!(fid, scalar.0.fid);
        assert_eq!(fid, scalar.1.fid);
        assert_eq!(fid, point.0.fid);
        assert_eq!(fid, point.1.fid);
        assert_eq!(out_wires[0].node_idx, node);
        assert_eq!(out_wires[1].node_idx, node);

        (out_wires[0], out_wires[1])
    }

    pub fn fp_message_pass(&mut self, input: Wire) -> (Wire, Wire) {
        assert_eq!(input.fid, PastaFieldId::Fp);
        let fid = input.fid.inv();
        self.message_pass_wire_count[fid as usize] += 2;
        self.row_count[fid as usize] += 2;

        let out_wires = self.new_wires(fid);

        let gate_type = GateType::FpMessagePass([input], out_wires);
        let node = self.graph.add_node(gate_type);
        self.graph.add_edge(input.node_idx, node, input);

        assert_eq!(out_wires[0].node_idx, node);
        assert_eq!(out_wires[1].node_idx, node);
        assert_eq!(out_wires[0].fid, PastaFieldId::Fq);
        assert_eq!(out_wires[1].fid, PastaFieldId::Fq);

        (out_wires[0], out_wires[1])
    }

    pub fn output_gate(&mut self, input: Wire) {
        let fid = input.fid;
        let out_id = self.output_wire_count[fid as usize];
        self.output_wire_count[fid as usize] += 1;

        let node = self.graph.add_node(GateType::Output([input], (), out_id));
        self.graph.add_edge(input.node_idx, node, input);
    }
}

impl std::fmt::Debug for CircuitSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dot = Dot::with_config(&self.graph, &[Config::EdgeNoLabel]);
        write!(f, "{dot:?}")
    }
}

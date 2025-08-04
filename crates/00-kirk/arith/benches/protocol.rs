use std::{
    collections::{HashMap, VecDeque},
    fmt,
    time::Instant,
};

use anyhow::Result;
use arith::{
    circuit::{CircuitSpec, Trace, TraceBuilder, Wire},
    plonk::PlonkProof,
};
use log::debug;
use petgraph::{
    Direction::Incoming,
    algo::toposort,
    graph::{DiGraph, NodeIndex},
    visit::EdgeRef,
};

use criterion::Criterion;
use halo_group::{
    PallasConfig, PastaConfig,
    ark_std::rand::{Rng, RngCore, thread_rng},
    ark_std::test_rng,
};

const MIN: usize = 3;
const MAX: usize = 20;
// const WARMUP: Duration = Duration::from_millis(1000);

#[derive(Debug, Clone)]
enum RandGate {
    Witness,
    PublicInput,
    Output,
    AssertEq,
    // Poseidon,
    Constant,
    Add,
    Multiply,
}
impl RandGate {
    const GATE_COUNT: usize = 7;
    fn input_outputs(&self) -> (usize, usize) {
        match self {
            RandGate::Witness => (0, 1),
            RandGate::PublicInput => (0, 1),
            RandGate::Constant => (0, 1),
            RandGate::Output => (1, 0),
            RandGate::AssertEq => (2, 0),
            RandGate::Add => (2, 1),
            RandGate::Multiply => (2, 1),
        }
    }

    fn rand<R: Rng>(rng: &mut R) -> Self {
        let x = rng.gen_range(0..2);
        match x {
            // 0 => RandGate::Witness,
            // 1 => RandGate::PublicInput,
            // 0 => RandGate::Witness,
            // 1 => RandGate::Witness,
            // 2 => RandGate::Constant,
            // 3 => RandGate::Output,
            // 0 => RandGate::AssertEq,
            0 => RandGate::Add,
            1 => RandGate::Multiply,
            _ => unreachable!(),
        }
    }

    fn rand_with_constraints<R: Rng>(rng: &mut R, open_gate_count: usize) -> RandGate {
        loop {
            let gate = Self::rand(rng);
            let (_, outputs) = gate.input_outputs();

            let cond1 = !(open_gate_count == 1 && outputs == 0);
            let cond2 = !(open_gate_count < outputs);
            if cond1 && cond2 {
                return gate;
            }
        }
    }
}
impl fmt::Display for RandGate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RandGate::Witness => write!(f, "Witness"),
            RandGate::PublicInput => write!(f, "PublicInput"),
            RandGate::Output => write!(f, "Output"),
            RandGate::AssertEq => write!(f, "AssertEq"),
            RandGate::Constant => write!(f, "Constant"),
            RandGate::Add => write!(f, "Add"),
            RandGate::Multiply => write!(f, "Multiply"),
        }
    }
}

fn push_gate(open_wires: &mut HashMap<NodeIndex, Vec<Wire>>, node_idx: &NodeIndex, wire: Wire) {
    match open_wires.get_mut(&node_idx) {
        None => {
            let _ = open_wires.insert(*node_idx, vec![wire]);
        }
        Some(vec) => vec.push(wire),
    }
}

fn get_incoming_nodes(graph: &DiGraph<RandGate, &str>, node_idx: NodeIndex) -> Vec<NodeIndex> {
    graph
        .edges_directed(node_idx, Incoming)
        .map(|e| e.source())
        .collect()
}

fn rand<P: PastaConfig>(wire_count: usize) -> Result<Trace<P>> {
    assert!(wire_count.is_power_of_two());

    let now = Instant::now();

    let mut total_wires = 0;
    let rng = &mut thread_rng();
    let mut graph: DiGraph<RandGate, &str> = DiGraph::new();

    let output_node = graph.add_node(RandGate::Output);
    let mut open_gates = VecDeque::from([output_node]);

    while total_wires + open_gates.len() <= wire_count {
        let gate = RandGate::rand_with_constraints(rng, open_gates.len());
        let (inputs, outputs) = gate.input_outputs();
        let new_node = graph.add_node(gate);

        for _ in 0..outputs {
            total_wires += 1;
            let open_node = open_gates.pop_back().unwrap();
            graph.add_edge(new_node, open_node, "");
        }
        for _ in 0..inputs {
            open_gates.push_front(new_node);
        }
    }
    for _ in 0..open_gates.len() {
        let open_node = open_gates.pop_back().unwrap();
        let witness_node = graph.add_node(RandGate::Witness);
        graph.add_edge(witness_node, open_node, "");
    }
    assert!(open_gates.len() == 0);

    let topo_order = toposort(&graph, None).unwrap();
    let mut circuit_spec = CircuitSpec::<P>::new();

    let mut open_wires = HashMap::<NodeIndex, Vec<Wire>>::new();
    let mut witness_wires = Vec::new();
    let mut public_input_wires = Vec::new();
    for node_idx in topo_order {
        let gate = graph.node_weight(node_idx).unwrap();
        match gate {
            RandGate::Witness => {
                let out_wire = circuit_spec.witness_gate();
                witness_wires.push(out_wire);
                push_gate(&mut open_wires, &node_idx, out_wire)
            }
            RandGate::PublicInput => {
                let out_wire = circuit_spec.public_input_gate();
                public_input_wires.push(out_wire);
                push_gate(&mut open_wires, &node_idx, out_wire)
            }
            RandGate::Constant => {
                let c = P::scalar_from_u64(rng.gen_range(0..=10));
                let out_wire = circuit_spec.constant_gate(c);
                push_gate(&mut open_wires, &node_idx, out_wire)
            }
            RandGate::Output => {
                let in_nodes = get_incoming_nodes(&graph, node_idx);
                assert_eq!(in_nodes.len(), 1);
                let in_wire = open_wires.get_mut(&in_nodes[0]).unwrap().pop().unwrap();
                circuit_spec.output_gate(in_wire);
            }
            RandGate::AssertEq => {
                let in_nodes = get_incoming_nodes(&graph, node_idx);
                assert_eq!(in_nodes.len(), 2);
                let left = open_wires.get_mut(&in_nodes[0]).unwrap().pop().unwrap();
                let right = open_wires.get_mut(&in_nodes[1]).unwrap().pop().unwrap();
                circuit_spec.assert_eq_gate(left, right);
            }
            RandGate::Add => {
                let in_nodes = get_incoming_nodes(&graph, node_idx);
                assert_eq!(in_nodes.len(), 2);
                let left = open_wires.get_mut(&in_nodes[0]).unwrap().pop().unwrap();
                let right = open_wires.get_mut(&in_nodes[1]).unwrap().pop().unwrap();
                let out_wire = circuit_spec.add_gate(left, right);
                push_gate(&mut open_wires, &node_idx, out_wire)
            }
            RandGate::Multiply => {
                let in_nodes = get_incoming_nodes(&graph, node_idx);
                assert_eq!(in_nodes.len(), 2);
                let left = open_wires.get_mut(&in_nodes[0]).unwrap().pop().unwrap();
                let right = open_wires.get_mut(&in_nodes[1]).unwrap().pop().unwrap();
                let out_wire = circuit_spec.mul_gate(left, right);
                push_gate(&mut open_wires, &node_idx, out_wire)
            }
        }
    }

    let mut trace_builder = TraceBuilder::new(circuit_spec);
    for wire in public_input_wires {
        let public_input = P::scalar_from_u64(rng.gen_range(0..=10));
        trace_builder.public_input(wire, public_input)?
    }
    for wire in witness_wires {
        let witness = P::scalar_from_u64(rng.gen_range(0..=10));
        trace_builder.witness(wire, witness)?
    }

    debug!("rand_time = {}", now.elapsed().as_secs_f32());

    let trace = trace_builder.trace()?;
    Ok(trace)
}

pub fn prover_verifier(c: &mut Criterion) {
    env_logger::init();
    let group = c.benchmark_group("prover_verifier");
    let rng = &mut test_rng();

    println!("|‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|‾‾‾‾‾‾‾‾‾‾‾‾‾‾|");
    println!("| n  | Trace (s)    | NaiveP (s)   | Prover (s)   | Verifier (s) |");
    println!("|====|==============|==============|==============|==============|");
    for size in MIN..MAX + 1 {
        let n = 2usize.pow(size as u32);

        let start_time = Instant::now();
        let trace = rand(n).unwrap();
        let random_circ_time = start_time.elapsed().as_secs_f32();

        let trace_clone = trace.clone();
        let start_time = Instant::now();
        let pi = PlonkProof::<PallasConfig>::naive_prover(rng, trace_clone);
        let naive_prover_time = start_time.elapsed().as_secs_f32();
        pi.verify(trace.clone()).unwrap();

        let trace_clone = trace.clone();
        let start_time = Instant::now();
        let pi = PlonkProof::<PallasConfig>::prove(rng, trace_clone);
        let prover_time = start_time.elapsed().as_secs_f32();

        let start_time = Instant::now();
        pi.verify(trace.clone()).unwrap();
        let verifier_time = start_time.elapsed().as_secs_f32();

        println!(
            "| {:02} | {:>12.8} | {:>12.8} | {:>12.8} | {:>12.8} |",
            size, random_circ_time, naive_prover_time, prover_time, verifier_time
        );
    }
    println!("|____|______________|______________|______________|");

    group.finish();
}

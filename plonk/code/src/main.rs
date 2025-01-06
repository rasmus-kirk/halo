mod plonk;
use plonk::{AbstractCircuit, SnarkProof};
use rand::rngs::ThreadRng;

fn main() {
    env_logger::init();

    // build circuit
    let [x, y] = &AbstractCircuit::<2>::build();
    let output_wire = 3 * (x * x) + (y * 5) - 47;
    let circuit = output_wire.input([1, 2]);
    circuit.print_circuit_info();

    // run protocol
    let snark: SnarkProof = circuit.prove(&mut ThreadRng::default());
    // snark.print();
    assert!(circuit.verify(&snark));
    println!("Plonk: Verified");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit() {
        let [x, y] = &AbstractCircuit::<2>::build();
        let circuit = (3 * (x * x) + (y * 5) - 47).input([1, 2]);
        assert!(circuit.verify(&circuit.prove(&mut ThreadRng::default())));
    }
}

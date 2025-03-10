pub mod curve;
pub mod protocol;
pub mod util;

#[cfg(test)]
mod tests {
    use super::*;
    use protocol::{arithmetizer::Arithmetizer, circuit::print_poly_evaluations, plonk};

    #[test]
    fn circuit() {
        let rng = &mut rand::thread_rng();
        let [x, y] = &Arithmetizer::build();
        let input_values = vec![1, 2];
        let output_wires = &[3 * (x * x) + (y * 5) - 47];
        println!("{}", Arithmetizer::to_string(&input_values, output_wires));
        let ((x, w), e) = &Arithmetizer::to_circuit(rng, input_values, output_wires).unwrap();
        println!("{}", e);
        assert_eq!(e.clone(), (x, w).into());
        print_poly_evaluations(x, w);
        let pi = plonk::proof(rng, x, w);
        let sat = plonk::verify(x, pi);
        assert!(sat);
    }

    #[test]
    fn new_circuit() {
        let rng = &mut rand::thread_rng();
        let [x, y] = &Arithmetizer::build();
        let input_values = vec![1, 2];
        let output_wires = &[3 * (x * x) + (y * 5) - 47];
        println!("{}", Arithmetizer::to_string(&input_values, output_wires));
        let ((x, w), e) = &Arithmetizer::to_circuit(rng, input_values, output_wires).unwrap();
        println!("{}", e);
        assert_eq!(e.clone(), (x, w).into());
        print_poly_evaluations(x, w);
        let pi_x = plonk::proof(rng, x, w);
        let pi = plonk::prove(rng, x, w);
        plonk::verifier(x, &pi).unwrap();
    }

    #[test]
    fn circuit_bool() {
        let rng = &mut rand::thread_rng();
        let [x, y] = &Arithmetizer::build();
        let input_values = vec![1, 0];
        let output_wires = &[(x | y.is_bit()).is_public()];
        println!("{}", Arithmetizer::to_string(&input_values, output_wires));
        let ((x, w), e) = &Arithmetizer::to_circuit(rng, input_values, output_wires).unwrap();
        println!("{}", e);
        print_poly_evaluations(x, w);
        let pi = plonk::proof(rng, x, w);
        let sat = plonk::verify(x, pi);
        assert!(sat);
    }

    #[test]
    fn circuit_synthesize() {
        let rng = &mut rand::thread_rng();
        let out = Arithmetizer::synthesize::<_, 2>(rng, 4);
        let input_values = vec![3, 4];
        let output_wires = &[out];
        println!("{}", Arithmetizer::to_string(&input_values, output_wires));
        let ((x, w), e) = &Arithmetizer::to_circuit(rng, input_values, output_wires).unwrap();
        println!("{}", e);
        print_poly_evaluations(x, w);
        let pi = plonk::proof(rng, x, w);
        let sat = plonk::verify(x, pi);
        assert!(sat);
    }
}

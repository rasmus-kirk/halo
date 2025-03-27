pub mod curve;
pub mod protocol;
pub mod util;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use protocol::{arithmetizer::Arithmetizer, circuit::print_poly_evaluations, plonk};

    #[test]
    fn circuit() -> Result<()> {
        let rng = &mut rand::thread_rng();
        let [x, y] = Arithmetizer::build();
        let input_values = vec![1, 2];
        let output_wires = &[3 * (x.clone() * x) + (y * 5) - 47];
        println!("{}", Arithmetizer::to_string(&input_values, output_wires));
        let ((x, w), e) =
            &Arithmetizer::to_circuit(rng, 2usize.pow(7) - 1, input_values, output_wires)?;
        println!("{}", e);
        assert_eq!(e.clone(), (x, w).into());
        print_poly_evaluations(x, w);
        // let _ = plonk::proof(rng, x, w);
        let pi = plonk::prove(rng, x, w);
        plonk::verify(x, pi)?;

        Ok(())
    }

    #[test]
    fn circuit_bool() -> Result<()> {
        let rng = &mut rand::thread_rng();
        let [x, y] = Arithmetizer::build();
        let input_values = vec![1, 0];
        let output_wires = &[(x.clone() ^ (y | x).is_bit()).is_public()];
        println!("{}", Arithmetizer::to_string(&input_values, output_wires));
        let ((x, w), e) =
            &Arithmetizer::to_circuit(rng, 2usize.pow(7) - 1, input_values, output_wires)?;
        println!("{}", e);
        print_poly_evaluations(x, w);
        let pi = plonk::prove(rng, x, w);
        // TODO: Fails?
        plonk::verify(x, pi)?;

        Ok(())
    }

    #[test]
    fn circuit_synthesize() -> Result<()> {
        let rng = &mut rand::thread_rng();
        let output_wires = &Arithmetizer::synthesize::<_, 2>(rng, 4);
        let input_values = vec![3, 4];
        println!("{}", Arithmetizer::to_string(&input_values, output_wires));
        let ((x, w), e) =
            &Arithmetizer::to_circuit(rng, 2usize.pow(7) - 1, input_values, output_wires)?;
        println!("{}", e);
        print_poly_evaluations(x, w);
        let pi = plonk::prove(rng, x, w);
        plonk::verify(x, pi)?;

        Ok(())
    }
}

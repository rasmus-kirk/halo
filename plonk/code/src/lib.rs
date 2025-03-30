pub mod arithmetizer;
pub mod circuit;
pub mod coset;
pub mod protocol;
pub mod scheme;
pub mod util;

pub use coset::Coset;

#[cfg(test)]
mod tests {
    use crate::util::misc::on_debug;

    use super::*;
    use anyhow::Result;
    use arithmetizer::Arithmetizer;
    use circuit::poly_evaluations_to_string;
    use log::debug;
    use protocol;

    #[test]
    fn circuit() -> Result<()> {
        let rng = &mut rand::thread_rng();
        let [x, y] = Arithmetizer::build();
        let input_values = vec![1, 2];
        let output_wires = &[3 * (x.clone() * x) + (y * 5) - 47];
        debug!("{}", Arithmetizer::to_string(&input_values, output_wires));
        let (x, w) = &Arithmetizer::to_circuit(rng, input_values, output_wires, None)?;
        debug!("{}", poly_evaluations_to_string(x, w));
        // let _ = plonk::proof(rng, x, w);
        let pi = protocol::prove(rng, x, w);
        protocol::verify(x, pi)?;

        Ok(())
    }

    #[test]
    fn circuit_bool() -> Result<()> {
        on_debug();
        let rng = &mut rand::thread_rng();
        let [x, y] = Arithmetizer::build();
        let input_values = vec![1, 0];
        let output_wires = &[(x.clone() ^ (y | x).is_bit()).is_public()];
        debug!("\n{}", Arithmetizer::to_string(&input_values, output_wires));
        let (x, w) = &Arithmetizer::to_circuit(rng, input_values, output_wires, None)?;
        debug!("\n{}", poly_evaluations_to_string(x, w));
        let pi = protocol::prove(rng, x, w);
        protocol::verify(x, pi)?;

        Ok(())
    }

    #[test]
    fn circuit_synthesize() -> Result<()> {
        let rng = &mut rand::thread_rng();
        let output_wires = &Arithmetizer::synthesize::<_, 2>(rng, 4);
        let input_values = vec![3, 4];
        debug!("{}", Arithmetizer::to_string(&input_values, output_wires));
        let (x, w) = &Arithmetizer::to_circuit(rng, input_values, output_wires, None)?;
        debug!("{}", poly_evaluations_to_string(x, w));
        let pi = protocol::prove(rng, x, w);
        protocol::verify(x, pi)?;

        Ok(())
    }
}

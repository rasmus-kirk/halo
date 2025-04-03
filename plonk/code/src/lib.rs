pub mod arithmetizer;
pub mod circuit;
pub mod coset;
pub mod protocol;
mod scheme;
mod utils;

pub use coset::Coset;

#[cfg(test)]
mod tests {
    use crate::{
        arithmetizer::{BinXorOr, EmptyOpSet},
        utils::misc::tests::on_debug,
    };

    use super::*;
    use anyhow::Result;
    use arithmetizer::Arithmetizer;
    use ark_pallas::PallasConfig;
    use circuit::poly_evaluations_to_string;
    use log::debug;
    use protocol;

    type PallasArithmetizer = Arithmetizer<EmptyOpSet, PallasConfig>;
    type BitXorArithmetizer = Arithmetizer<BinXorOr, PallasConfig>;

    #[test]
    fn circuit_canonical() -> Result<()> {
        on_debug();
        let rng = &mut rand::thread_rng();
        let [x, y] = PallasArithmetizer::build();
        let input_values = vec![1, 2];
        let output_wires = &[(x.clone() * x) * 3 + (y * 5) - 47];
        debug!(
            "{}",
            PallasArithmetizer::to_string(&input_values, output_wires)
        );
        let (x, w) = &PallasArithmetizer::to_circuit(rng, input_values, output_wires, None)?;
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
        let [x, y] = BitXorArithmetizer::build();
        let input_values = vec![1, 0];
        let output_wires = &[(x.clone() ^ (y | x).is_bit()).is_public()];
        debug!(
            "\n{}",
            BitXorArithmetizer::to_string(&input_values, output_wires)
        );
        let (x, w) = &BitXorArithmetizer::to_circuit(rng, input_values, output_wires, None)?;
        debug!("\n{}", poly_evaluations_to_string(x, w));
        let pi = protocol::prove(rng, x, w);
        protocol::verify(x, pi)?;

        Ok(())
    }

    #[test]
    fn circuit_synthesize() -> Result<()> {
        on_debug();
        let rng = &mut rand::thread_rng();
        let output_wires = &PallasArithmetizer::synthesize::<_, 2>(rng, 4);
        let input_values = vec![3, 4];
        debug!(
            "{}",
            PallasArithmetizer::to_string(&input_values, output_wires)
        );
        let (x, w) = &PallasArithmetizer::to_circuit(rng, input_values, output_wires, None)?;
        debug!("{}", poly_evaluations_to_string(x, w));
        let pi = protocol::prove(rng, x, w);
        protocol::verify(x, pi)?;

        Ok(())
    }
}

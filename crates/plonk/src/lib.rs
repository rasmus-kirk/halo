pub mod arithmetizer;
pub mod circuit;
pub mod coset;
mod gadgets;
pub mod pcs;
pub mod protocol;
mod scheme;
mod utils;
pub use coset::Coset;

#[cfg(test)]
mod tests {
    use crate::{
        arithmetizer::{PallasBitArith, PallasEmptyArith, Witness},
        circuit::poly_evaluations_to_string,
        pcs::PCSPallas,
        protocol,
        utils::misc::tests::on_debug,
    };

    use anyhow::Result;
    use log::debug;

    #[test]
    fn circuit_canonical() -> Result<()> {
        on_debug();
        let rng = &mut rand::thread_rng();
        let [x, y] = PallasEmptyArith::build();
        let input_values = &[1, 2];
        let output_wires = &[(x.clone() * x) * 3 + (y * 5) - 47];
        debug!(
            "{}",
            PallasEmptyArith::to_string(input_values, output_wires)
        );
        let (x, w) = &PallasEmptyArith::to_circuit::<_, _, PCSPallas>(
            rng,
            input_values,
            output_wires,
            None,
        )?;
        debug!("{}", poly_evaluations_to_string(x, w));
        // let _ = plonk::proof(rng, x, w);
        let pi = protocol::prove::<_, _, PCSPallas>(rng, x, w);
        protocol::verify(false, x, pi)?;

        Ok(())
    }

    #[test]
    fn circuit_bool() -> Result<()> {
        on_debug();
        let rng = &mut rand::thread_rng();
        let [x, y] = PallasBitArith::build();
        let input_values = &[1, 0];
        let output_wires = &[(x.clone() ^ (y | x).is_bit()).is_public()];
        debug!(
            "\n{}",
            PallasBitArith::to_string(input_values, output_wires)
        );
        let (x, w) =
            &PallasBitArith::to_circuit::<_, _, PCSPallas>(rng, input_values, output_wires, None)?;
        debug!("\n{}", poly_evaluations_to_string(x, w));
        let pi = protocol::prove::<_, _, PCSPallas>(rng, x, w);
        protocol::verify(false, x, pi)?;

        Ok(())
    }

    #[test]
    fn circuit_witness() -> Result<()> {
        on_debug();
        let rng = &mut rand::thread_rng();
        let [x] = PallasEmptyArith::build();
        let input_values = &[3];
        let output_wires = &[(x * Witness::new(4) + Witness::new(5)) / Witness::new(3)];
        debug!(
            "\n{}",
            PallasEmptyArith::to_string(input_values, output_wires)
        );
        let (x, w) = &PallasEmptyArith::to_circuit::<_, _, PCSPallas>(
            rng,
            input_values,
            output_wires,
            None,
        )?;
        debug!("\n{}", poly_evaluations_to_string(x, w));
        let pi = protocol::prove::<_, _, PCSPallas>(rng, x, w);
        protocol::verify(false, x, pi)?;

        Ok(())
    }

    #[test]
    fn circuit_synthesize() -> Result<()> {
        on_debug();
        let rng = &mut rand::thread_rng();
        let output_wires = &PallasEmptyArith::synthesize::<2, _>(rng, 4);
        let input_values = &[3, 4];
        debug!(
            "{}",
            PallasEmptyArith::to_string(input_values, output_wires)
        );
        let (x, w) = &PallasEmptyArith::to_circuit::<_, _, PCSPallas>(
            rng,
            input_values,
            output_wires,
            None,
        )?;
        debug!("{}", poly_evaluations_to_string(x, w));
        let pi = protocol::prove::<_, _, PCSPallas>(rng, x, w);
        protocol::verify(false, x, pi)?;

        Ok(())
    }
}

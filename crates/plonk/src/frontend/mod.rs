use anyhow::Result;
use halo_accumulation::acc::Accumulator;
use std::{cell::RefCell, marker::PhantomData};

use halo_group::{Affine, PallasConfig, PastaConfig, PastaFE, Scalar, VestaConfig};

use crate::{
    circuit::{CircuitSpec, PlonkCircuit, Trace, TraceBuilder},
    frontend::primitives::{WireAffine, WireBool, WireScalar},
};

pub mod asdl;
pub mod ivc;
pub mod pcdl;
pub mod plonk;
pub mod poseidon;
pub mod primitives;
pub mod signature;

thread_local! {
    static FRONTEND: RefCell<Frontend> = RefCell::new(Frontend::new());
}

#[derive(Clone)]
pub struct Frontend {
    circuit: CircuitSpec,
}
impl Frontend {
    pub fn new() -> Self {
        Self {
            circuit: CircuitSpec::new(),
        }
    }

    pub fn reset() {
        FRONTEND.with(|frontend| {
            let mut frontend = frontend.borrow_mut();
            *frontend = Self::new()
        })
    }
}

pub struct Call {
    trace_builder: TraceBuilder,
}
impl Call {
    pub fn new() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            let trace_builder = TraceBuilder::new(frontend.circuit.clone());
            Call { trace_builder }
        })
    }
    pub fn witness<P: PastaConfig>(&mut self, fp: WireScalar<P>, scalar: Scalar<P>) -> Result<()> {
        self.trace_builder
            .witness(fp.wire, PastaFE::from_scalar::<P>(scalar))
    }
    pub fn witness_bool<P: PastaConfig>(&mut self, fp: WireBool<P>, b: bool) -> Result<()> {
        assert_eq!(fp.wire.fid, P::SFID);
        let fe = if b {
            PastaFE::one(Some(P::SFID))
        } else {
            PastaFE::zero(Some(P::SFID))
        };
        self.trace_builder.witness(fp.wire, fe)
    }
    pub fn witness_scalar_bool<P: PastaConfig>(&mut self, fp: WireBool<P>, b: bool) -> Result<()> {
        let fe = if b {
            PastaFE::one(Some(P::SFID))
        } else {
            PastaFE::zero(Some(P::SFID))
        };
        self.trace_builder.witness(fp.wire, fe)
    }
    pub fn witness_base_bool<P: PastaConfig>(
        &mut self,
        fp: WireBool<P::OtherCurve>,
        b: bool,
    ) -> Result<()> {
        let fe = if b {
            PastaFE::one(Some(P::BFID))
        } else {
            PastaFE::zero(Some(P::BFID))
        };
        self.trace_builder.witness(fp.wire, fe)
    }
    pub fn witness_affine<P: PastaConfig>(
        &mut self,
        p: WireAffine<P>,
        affine: Affine<P>,
    ) -> Result<()> {
        assert!(affine.is_on_curve());
        self.trace_builder
            .witness(p.x.wire, PastaFE::from_basefield::<P>(affine.x))?;
        self.trace_builder
            .witness(p.y.wire, PastaFE::from_basefield::<P>(affine.y))?;
        Ok(())
    }
    pub fn public_input_affine<P: PastaConfig>(
        &mut self,
        p: WireAffine<P>,
        affine: Affine<P>,
    ) -> Result<()> {
        assert!(affine.is_on_curve());
        self.trace_builder
            .public_input(p.x.wire, PastaFE::from_basefield::<P>(affine.x))?;
        self.trace_builder
            .public_input(p.y.wire, PastaFE::from_basefield::<P>(affine.y))?;
        Ok(())
    }
    pub fn public_input<P: PastaConfig>(
        &mut self,
        fp: WireScalar<P>,
        scalar: Scalar<P>,
    ) -> Result<()> {
        self.trace_builder
            .public_input(fp.wire, PastaFE::from_scalar::<P>(scalar))
    }
    pub fn trace(self) -> Result<(Trace<PallasConfig>, Trace<VestaConfig>)> {
        Ok(self.trace_builder.trace(None, None)?)
    }
    pub fn trace_with_params(
        self,
        accs_prev: Option<(Accumulator<PallasConfig>, Accumulator<VestaConfig>)>,
        static_circuit: Option<(PlonkCircuit<PallasConfig>, PlonkCircuit<VestaConfig>)>,
    ) -> Result<(Trace<PallasConfig>, Trace<VestaConfig>)> {
        Ok(self.trace_builder.trace(accs_prev, static_circuit)?)
    }
}

use anyhow::Result;
use std::{cell::RefCell, marker::PhantomData};

use halo_group::{
    Affine, PallasConfig, PastaConfig, PastaFE, Scalar, VestaConfig,
    ark_ec::CurveConfig,
    ark_std::{One, Zero},
};

use crate::{
    circuit::{CircuitSpec, Trace, TraceBuilder},
    frontend::{
        // curve::CurvePoint,
        field::WireScalar,
    },
};

pub mod curve;
pub mod field;
pub mod poseidon;
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

pub struct Call<P: PastaConfig> {
    trace_builder: TraceBuilder,
    _p: PhantomData<P>,
}
impl<P: PastaConfig> Call<P> {
    pub fn new() -> Self {
        FRONTEND.with(|frontend| {
            let frontend = frontend.borrow();
            let trace_builder = TraceBuilder::new(frontend.circuit.clone());
            Call {
                trace_builder,
                _p: PhantomData,
            }
        })
    }
    pub fn witness(&mut self, fp: WireScalar<P>, scalar: Scalar<P>) -> Result<()> {
        self.trace_builder
            .witness(fp.wire, PastaFE::from_scalar::<P>(scalar))
    }
    // pub fn curve_witness(&mut self, p: CurvePoint, affine: Affine<PallasConfig>) -> Result<()> {
    //     assert!(affine.is_on_curve());
    //     self.trace_builder.witness(p.x.wire, affine.x)?;
    //     self.trace_builder.witness(p.y.wire, affine.y)?;
    //     Ok(())
    // }
    pub fn public_input(&mut self, fp: WireScalar<P>, scalar: Scalar<P>) -> Result<()> {
        self.trace_builder
            .public_input(fp.wire, PastaFE::from_scalar::<P>(scalar))
    }
    pub fn trace(self) -> Result<(Trace<PallasConfig>, Trace<VestaConfig>)> {
        Ok(self.trace_builder.trace()?)
    }
}

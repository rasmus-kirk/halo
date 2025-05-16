use crate::{
    arithmetizer::{PallasEmptyArith, PlookupOps, Trace, Wire},
    utils::Scalar,
};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;

use anyhow::{ensure, Result};
use rand::{distributions::Standard, prelude::Distribution, Rng};

const FUZZING_COUNT: usize = 20;

pub fn test_extensionality<F, T, Op: PlookupOps, P: SWCurveConfig>(
    rust_func: F,
    input_values: &[T],
    output_wire: Wire<Op, P>,
) -> Result<()>
where
    T: Copy,
    Scalar<P>: From<T>,
    F: Fn(&[Scalar<P>]) -> Scalar<P>,
    Standard: Distribution<Scalar<P>>,
{
    let rng = &mut rand::thread_rng();
    let input_scalars: Vec<Scalar<P>> = input_values.iter().map(|&x| x.into()).collect();
    let output_ids = vec![output_wire.id()];
    let wires = &output_wire.arith().borrow().wires;
    let trace_op = Trace::<P>::new(rng, None, wires, input_scalars.clone(), output_ids);
    ensure!(trace_op.is_ok(), "Failed to create trace");
    let trace = trace_op.unwrap();
    let res_opt = trace.get(output_wire.id());
    ensure!(res_opt.is_some(), "Output wire not found in trace");
    let res = res_opt.unwrap().to_fp();
    let expected = rust_func(&input_scalars);
    ensure!(res == expected, "Expected: {:?}, got: {:?}", expected, res);
    Ok(())
}

#[test]
fn test_add() {
    let rng = &mut rand::thread_rng();
    let [x, y] = PallasEmptyArith::build();
    let output_wire = x.clone() + y;
    for _ in 0..FUZZING_COUNT {
        let input_values: Vec<u32> = vec![rng.gen(), rng.gen()];
        test_extensionality(
            |inputs| inputs[0] + inputs[1],
            &input_values,
            output_wire.clone(),
        )
        .unwrap();
    }
}

#[test]
fn test_mul() {
    let rng = &mut rand::thread_rng();
    let [x, y] = PallasEmptyArith::build();
    let output_wire = x.clone() * y;
    for _ in 0..FUZZING_COUNT {
        let input_values: Vec<u32> = vec![rng.gen(), rng.gen()];
        test_extensionality(
            |inputs| inputs[0] * inputs[1],
            &input_values,
            output_wire.clone(),
        )
        .unwrap();
    }
}

#[test]
fn test_mul_inv() {
    let rng = &mut rand::thread_rng();
    let [x] = PallasEmptyArith::build();
    let output_wire = x.inv();
    for _ in 0..FUZZING_COUNT {
        let mut scalar = rng.gen();
        while scalar == 0 {
            scalar = rng.gen();
        }
        let input_values: Vec<u32> = vec![scalar];
        test_extensionality(
            |inputs| inputs[0].inverse().unwrap(),
            &input_values,
            output_wire.clone(),
        )
        .unwrap();
    }
}

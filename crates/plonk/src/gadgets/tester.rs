use crate::{
    arithmetizer::{PallasEmptyArith, PlookupOps, Trace, Wire},
    utils::Scalar,
};

use anyhow::{ensure, Result};
use ark_ec::short_weierstrass::SWCurveConfig;
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

// struct ArithPoint<T: PlookupOps, U: SWCurveConfig> {
//     x: Wire<T, U>,
//     y: Wire<T, U>,
// }

// fn if_then_else<T: PlookupOps, U: SWCurveConfig>(b: Wire<T, U>, x: Wire<T, U>, y: Wire<T, U>) -> Wire<T, U> {
//     b.is_bit();
//     b.clone() * x + !b * y
// }

// fn eq<T: PlookupOps, U: SWCurveConfig>(_: Wire<T, U>, _: Wire<T, U>) {
//     ()
// }

// impl<T: PlookupOps, U: SWCurveConfig> ArithPoint<T, U> {
//     fn new(x: Wire<T, U>, y: Wire<T, U>) -> Self {
//         let x_cubed = x.clone() * x.clone() * x.clone();
//         let y_squared = y.clone() * y.clone();
//         let z = y_squared - x_cubed - 5;
//         //eq(x * z, 0);
//         //eq(y * z, 0);
//         Self { x, y }
//     }

//     fn incomplete_add(self, other: ArithPoint<T, U>) {
//         let x = Affine::<U>::new(self.x, self.y);
//         (xr + xq + xp) * (xp - xq) * (xp - xq) - (yp - yq) * (yp - yq)
//     }
// }

// #[test]
// fn test_ecc() {
//     let rng = &mut rand::thread_rng();
//     let [p_x, p_y, q_x, q_y] = PallasEmptyArith::build();

//     let p = ArithPoint::new(p_x, p_y);
//     let q = ArithPoint::new(q_x, q_y);

//     for _ in 0..FUZZING_COUNT {
//         let input_values: Vec<u32> = vec![rng.gen(), rng.gen()];
//         test_extensionality(
//             |inputs| inputs[0] + inputs[1],
//             &input_values,
//             output_wire.clone(),
//         )
//         .unwrap();
//     }
// }

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

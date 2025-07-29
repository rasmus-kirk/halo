use std::array;

use halo_group::{
    Evals, PastaConfig, Scalar,
    ark_ff::{BigInt, BigInteger, Field},
    ark_poly::Polynomial,
    ark_std::Zero,
};

use crate::circuit::Trace;

pub const WITNESS_POLYS: usize = 3;
pub const SELECTOR_POLYS: usize = 5;

pub trait MultiAssign<P: PastaConfig> {
    fn multi_assign<const N: usize>(&mut self, row: usize, values: [P::ScalarField; N]);
}

impl<P: PastaConfig, const N: usize> MultiAssign<P> for [Evals<P>; N] {
    fn multi_assign<const M: usize>(&mut self, row: usize, values: [P::ScalarField; M]) {
        assert_eq!(
            self.len(),
            values.len(),
            "Number of values must match number of vectors"
        );
        for (evals, value) in self.iter_mut().zip(values) {
            evals.vec[row] = value;
        }
    }
}

pub trait IteratorSplitExt: Iterator {
    fn split_array<const N: usize>(self) -> [Vec<Self::Item>; N]
    where
        Self::Item: Clone,
        Self: Sized,
    {
        assert!(N > 0, "N must be greater than 0");

        let mut result = [(); N].map(|_| Vec::new());
        for (i, item) in self.enumerate() {
            let group = i % N;
            result[group].push(item);
        }

        // Check if all groups have the same length to ensure input length was divisible by N
        let first_len = result.get(0).map_or(0, |v| v.len());
        assert!(
            result.iter().all(|v| v.len() == first_len),
            "Input length must be divisible by N"
        );

        result
    }
}

// Implement the trait for all types that implement Iterator
impl<T: Iterator> IteratorSplitExt for T {}

fn fmt_scalar<P: PastaConfig>(x: Scalar<P>) -> String {
    let x_big = P::scalar_into_bigint(x);
    let half = P::FP_MODULUS >> 1;
    let one_hundred = BigInt::<4>::new([0, 0, 0, 100]);

    if x_big > half {
        let mut y = P::FP_MODULUS;
        y.sub_with_borrow(&x_big);
        if y > one_hundred {
            format!(",")
        } else {
            format!("-{}", y)
        }
    } else if x_big > one_hundred {
        format!(".")
    } else {
        format!("{}", x_big)
    }
}

fn reorder<T: Clone>(cols: &[Vec<T>]) -> Vec<Vec<T>> {
    let num_cols = cols.len(); // Number of columns in column-major form
    let num_rows = cols[0].len(); // Number of rows per column

    // Flatten the column-major matrix into a flat vector (row-major order)
    let mut flat = Vec::with_capacity(num_cols * num_rows);
    for row in 0..num_rows {
        for col in 0..num_cols {
            flat.push(cols[col][row].clone());
        }
    }

    // Group the flat vector into rows of `num_cols` elements each
    flat.chunks(num_rows) // `* 1` is optional, just makes it explicit
        .map(|chunk| chunk.to_vec())
        .collect()
}

impl<P: PastaConfig> std::fmt::Debug for Trace<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut id_polys: [_; WITNESS_POLYS] =
            array::from_fn(|_| vec![Scalar::<P>::zero(); self.rows]);
        let mut sigma_polys: [_; WITNESS_POLYS] =
            array::from_fn(|_| vec![Scalar::<P>::zero(); self.rows]);

        for i in 0..self.rows {
            let i = i + 1;
            for j in 0..WITNESS_POLYS {
                id_polys[j][i - 1] = self.id_polys[j].evaluate(&self.omega.pow([i as u64]));
                sigma_polys[j][i - 1] = self.sigma_polys[j].evaluate(&self.omega.pow([i as u64]));
            }
        }
        let id_polys = reorder(&id_polys);
        let sigma_polys = reorder(&sigma_polys);

        let width = 4;
        write!(f, "|   i || ")?;
        for i in 0..self.ws.len() {
            write!(f, "  w_{} |", i)?;
        }
        for i in 0..self.qs.len() {
            write!(f, "  q_{} |", i)?;
        }
        for i in 0..self.id_polys.len() {
            write!(f, " id_{} |", i)?;
        }
        for i in 0..self.sigma_polys.len() {
            write!(f, "  s_{} |", i)?;
        }
        write!(f, "\n|-----||-")?;
        for _ in 0..self.qs.len() + self.ws.len() + self.id_polys.len() + self.sigma_polys.len() {
            write!(f, "------|")?;
        }
        write!(f, "\n")?;

        for i in 0..self.rows {
            let i = i + 1;
            write!(f, "|   {i} || ")?;
            for j in 0..self.ws.len() {
                let eval = self.ws[j].evaluate(&self.omega.pow([i as u64]));
                write!(f, " {:>width$} |", fmt_scalar::<P>(eval))?;
            }
            for j in 0..self.qs.len() {
                let eval = self.qs[j].evaluate(&self.omega.pow([i as u64]));
                write!(f, " {:>width$} |", fmt_scalar::<P>(eval))?;
            }
            for j in 0..WITNESS_POLYS {
                write!(f, " {:>width$} |", fmt_scalar::<P>(id_polys[j][i - 1]))?;
            }
            for j in 0..WITNESS_POLYS {
                write!(f, " {:>width$} |", fmt_scalar::<P>(sigma_polys[j][i - 1]))?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

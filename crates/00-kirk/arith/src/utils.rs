use std::array;

use halo_group::ark_ff::PrimeField;
use halo_group::ark_poly::EvaluationDomain;
use halo_group::{Fp, Fq};
use halo_group::{
    PastaConfig, Poly, Scalar,
    ark_ff::{BigInt, BigInteger, Field},
    ark_poly::{GeneralEvaluationDomain, Polynomial},
    ark_std::Zero,
};

use crate::circuit::Trace;

/// The maximum degree of the polynomial f(X) where t(X) = f(X) / z_H(X) is F_MAX_DEGREE_MULTIPLIER * row_count.
/// This depends on the largest degree term in f(x) which is set by how many degree n polynomials are multiplied.
pub const T_POLYS: usize = 16;
/// How many witness polynomials in plonk
pub const W_POLYS: usize = 16;
/// How many round coefficient polynomials in plonk
pub const R_POLYS: usize = 15;
/// How many selector polynomials in plonk
pub const Q_POLYS: usize = 9;
/// How many copy constraint polynomials in plonk
pub const S_POLYS: usize = 8;

pub trait MultiAssign<T> {
    fn multi_assign<const N: usize>(&mut self, row: usize, values: [T; N]);
}
impl<T, const N: usize> MultiAssign<T> for [Vec<T>; N]
where
    T: std::fmt::Debug,
{
    fn multi_assign<const M: usize>(&mut self, row: usize, values: [T; M]) {
        assert_eq!(N, M);
        assert_eq!(
            self.len(),
            values.len(),
            "Number of values must match number of vectors"
        );
        for (vec, value) in self.iter_mut().zip(values) {
            assert!(
                row < vec.len(),
                "Error: row >= vec.len(): {row} >= {:?}",
                vec.len()
            );
            vec[row] = value;
        }
    }
}

pub fn fmt_scalar<P: PastaConfig>(x: Scalar<P>) -> String {
    let x_big = P::scalar_into_bigint(x);
    let half = P::SCALAR_MODULUS >> 1;
    let one_hundred = BigInt::<4>::new([0, 0, 0, 1000]);

    if x_big > half {
        let mut y = P::SCALAR_MODULUS;
        y.sub_with_borrow(&x_big);
        if y > one_hundred {
            let s = format!("-{}", y);
            s.get(s.len() - 3..).unwrap().to_string()
        } else {
            format!("-{}", y)
        }
    } else if x_big > one_hundred {
        let s = format!("{}", x_big);
        s.get(s.len() - 3..).unwrap().to_string()
    } else {
        format!("{}", x_big)
    }
}

impl<P: PastaConfig> std::fmt::Debug for Trace<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut id_polys: [_; S_POLYS] = array::from_fn(|_| vec![Scalar::<P>::zero(); self.rows]);
        let mut sigma_polys: [_; S_POLYS] =
            array::from_fn(|_| vec![Scalar::<P>::zero(); self.rows]);

        for i in 0..self.rows {
            let i = i + 1;
            for j in 0..S_POLYS {
                id_polys[j][i - 1] = self.id_polys[j].evaluate(&self.omega.pow([i as u64]));
                sigma_polys[j][i - 1] = self.sigma_polys[j].evaluate(&self.omega.pow([i as u64]));
            }
        }
        // writeln!(f, "{:?}", id_polys)?;
        // writeln!(f, "{:?}", sigma_polys)?;
        // let id_polys = reorder(&id_polys);
        // let sigma_polys = reorder(&sigma_polys);
        // writeln!(f, "{:?}", id_polys)?;
        // writeln!(f, "{:?}", sigma_polys)?;

        let width = 4;
        let i_width = 2;
        // write!(f, "|  i ||")?;
        // for i in 0..self.w_polys.len() {
        //     write!(f, "  w{} |", i)?;
        // }
        // for i in 0..self.q_polys.len() {
        //     write!(f, "  q{} |", i)?;
        // }
        // write!(f, "  pi |")?;
        // for i in 0..self.id_polys.len() {
        //     write!(f, " id{} |", i)?;
        // }
        // for i in 0..self.sigma_polys.len() {
        //     write!(f, "  s{} |", i)?;
        // }

        write!(f, "\n|  i ||")?;
        for i in 0..self.w_polys.len() {
            write!(f, " w{:>2} |", i)?;
        }
        write!(f, " pi  |")?;
        write!(f, "\n|----||")?;
        for _ in 0..self.w_polys.len() + 1 {
            write!(f, "-----|")?;
        }
        write!(f, "\n")?;
        for i in 0..self.rows {
            let i = i + 1;
            let omega_i = &self.omega.pow([i as u64]);

            let mut x = 99;
            for j in 0..self.q_polys.len() {
                if self.q_polys[j].evaluate(&omega_i) == Scalar::<P>::ONE {
                    x = j
                }
            }
            //                    [l, r, o, m, c, p, +, *, =]
            let string = match x {
                0 => "_",
                1 => "_",
                2 => "_",
                3 => "_",
                4 => "C",
                5 => "P",
                6 => "+",
                7 => "*",
                8 => "=",
                99 => "0",
                _ => "_",
            };

            write!(f, "| {:>i_width$} ||", i)?;
            for j in 0..self.w_polys.len() {
                let eval = self.w_polys[j].evaluate(&omega_i);
                write!(f, "{:>width$} |", fmt_scalar::<P>(eval))?;
            }
            let pi_i = self.public_inputs_poly.evaluate(&omega_i);
            write!(f, "{:>width$} |", fmt_scalar::<P>(pi_i))?;
            write!(f, "{:>width$} |", string)?;
            write!(f, "\n")?;
        }

        // write!(f, "\n|  i ||")?;
        // for i in 0..self.q_polys.len() {
        //     write!(f, " q{:>2} |", i)?;
        // }
        // write!(f, "\n|----||")?;
        // for _ in 0..self.q_polys.len() {
        //     write!(f, "-----|")?;
        // }
        // write!(f, "\n")?;
        // for i in 0..self.rows {
        //     let i = i + 1;
        //     let omega_i = &self.omega.pow([i as u64]);
        //     write!(f, "| {:>i_width$} ||", i)?;
        //     for j in 0..self.q_polys.len() {
        //         let eval = self.q_polys[j].evaluate(&omega_i);
        //         write!(f, "{:>width$} |", fmt_scalar::<P>(eval))?;
        //     }
        //     write!(f, "\n")?;
        // }

        // write!(f, "\n|  i ||")?;
        // for i in 0..self.sigma_polys.len() {
        //     write!(f, " o{:>2} |", i)?;
        // }
        // write!(f, "\n|----||")?;
        // for _ in 0..self.sigma_polys.len() {
        //     write!(f, "-----|")?;
        // }
        // write!(f, "\n")?;
        // for i in 0..self.rows {
        //     let i = i + 1;
        //     let omega_i = &self.omega.pow([i as u64]);
        //     write!(f, "| {:>i_width$} ||", i)?;
        //     for j in 0..self.sigma_polys.len() {
        //         let eval = self.sigma_polys[j].evaluate(&omega_i);
        //         write!(f, "{:>width$} |", fmt_scalar::<P>(eval))?;
        //     }
        //     write!(f, "\n")?;
        // }

        // write!(f, "\n|  i ||")?;
        // for i in 0..self.r_polys.len() {
        //     write!(f, " r{:>2} |", i)?;
        // }
        // write!(f, "\n|----||")?;
        // for _ in 0..self.r_polys.len() {
        //     write!(f, "-----|")?;
        // }
        // write!(f, "\n")?;
        // for i in 0..self.rows {
        //     let i = i + 1;
        //     let omega_i = &self.omega.pow([i as u64]);
        //     write!(f, "| {:>i_width$} ||", i)?;
        //     for j in 0..self.r_polys.len() {
        //         let eval = self.r_polys[j].evaluate(&omega_i);
        //         write!(f, "{:>width$} |", fmt_scalar::<P>(eval))?;
        //     }
        //     write!(f, "\n")?;
        // }

        Ok(())
    }
}

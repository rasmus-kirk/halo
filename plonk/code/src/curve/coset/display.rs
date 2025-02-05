use super::Coset;
use crate::{
    curve::{Poly, Scalar},
    util::{to_subscript, to_superscript},
};

use ascii_table::{Align, AsciiTable};
use std::fmt;

impl Coset {
    fn w_str(i: u64) -> String {
        format!("ω{}", to_superscript(i))
    }

    /// Util to print Scalars formatted as elements in H otherwise just print the scalar.
    fn v_str(&self, x: &Scalar) -> String {
        if *x == Scalar::ONE {
            return Scalar::ONE.to_string();
        }
        for slot in 0..self.ks.len() {
            if x == &self.ks[slot] {
                return format!("k{}", to_subscript(slot as u64));
            }
            for (i_, w) in self.vec_k(slot).iter().enumerate() {
                let i = (i_ + 1) as u64;
                if x == w {
                    return if slot == 0 {
                        format!("ω{}", to_superscript(i))
                    } else {
                        format!("k{} ω{}", to_subscript(slot as u64), to_superscript(i))
                    };
                }
            }
        }
        x.to_string()
    }

    /// Print the evaluations of a vector of polynomials for all elements in the coset.
    pub fn evals_str(&self, fs: &[&Poly], hs: Vec<String>, is_pos: Vec<bool>) -> String {
        let data: Vec<Vec<String>> = (0..self.n() + 1)
            .map(|i| {
                let mut row = vec![Self::w_str(i).to_string()];
                row.extend(fs.iter().enumerate().map(|(j, &f)| {
                    let y = f.evaluate(&self.w(i));
                    if is_pos[j] {
                        self.v_str(&y)
                    } else {
                        y.to_string()
                    }
                }));
                row
            })
            .collect();

        let mut ascii_table = AsciiTable::default();
        ascii_table.column(0).set_header("").set_align(Align::Left);
        for i in 1..fs.len() + 1 {
            ascii_table
                .column(i)
                .set_header(hs[i - 1].to_string())
                .set_align(Align::Right);
        }
        ascii_table.format(data)
    }

    /// Print the evaluation of a polynomial for all elements in the coset.
    pub fn poly_str(&self, p: &Poly, pos: bool, header: String) -> String {
        self.evals_str(&[p], vec![header], vec![pos])
    }
}

impl fmt::Display for Coset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ω{} = 1", to_superscript(self.n()))
    }
}

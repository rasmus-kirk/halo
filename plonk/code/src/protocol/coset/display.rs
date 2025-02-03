use super::{Coset, Slots};
use crate::{
    curve::{Poly, Scalar},
    protocol::arithmetizer::Pos,
};

use ascii_table::{Align, AsciiTable};
use std::fmt;

impl Coset {
    fn w_str(i: u64) -> String {
        Pos::new(Slots::A, i).to_string()
    }

    /// util to print Scalars formatted as elements in H otherwise just print the scalar.
    fn v_str(&self, x: &Scalar) -> String {
        for slot in Slots::iter() {
            if x == &self.ks[slot as usize] {
                return Pos::new(slot, 0).to_string();
            }
            for (i_, w) in self.vec_k(slot).iter().enumerate() {
                let i = (i_ + 1) as u64;
                if x == w {
                    return Pos::new(slot, i).to_string();
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
        write!(f, "{} = 1", Pos::new(Slots::A, self.n()))
    }
}

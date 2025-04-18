use super::misc::{to_subscript, to_superscript};
use crate::Coset;

use halo_accumulation::group::{PallasPoly, PallasScalar};

use ark_ff::Field;
use ark_poly::Polynomial;
use ascii_table::{Align, AsciiTable};

type Scalar = PallasScalar;
type Poly = PallasPoly;

pub fn print_scalar(x: &Scalar) -> String {
    let s1 = format!("{}", x);
    let s2 = format!("{}", -*x);
    if s2.len() < s1.len() {
        format!("-{}", s2)
    } else {
        s1
    }
}

fn w_str(i: u64) -> String {
    format!("ω{}", to_superscript(i))
}

/// Util to print Scalars formatted as elements in H otherwise just print the scalar.
fn v_str(h: &Coset, x: &Scalar) -> String {
    if *x == Scalar::ONE {
        return Scalar::ONE.to_string();
    }
    for slot in 0..h.l() {
        if x == &h.h(slot, 1) {
            return format!("k{}", to_subscript(slot as u64));
        }
        for (i_, w) in h.vec_k(slot).iter().enumerate() {
            let i = (i_ + 1) as u64;
            if x == w {
                return if slot == 0 {
                    w_str(i)
                } else {
                    format!("k{} ω{}", to_subscript(slot as u64), to_superscript(i))
                };
            }
        }
    }
    print_scalar(x)
}

/// Print the evaluations of a vector of polynomials for all elements in the coset.
pub fn evals_str(h: &Coset, fs: Vec<&Poly>, hs: Vec<String>, is_pos: Vec<bool>) -> String {
    let data: Vec<Vec<String>> = (0..h.n() + 1)
        .map(|i| {
            let mut row = vec![w_str(i).to_string()];
            row.extend(fs.iter().enumerate().map(|(j, &f)| {
                let y = f.evaluate(&h.w(i));
                if is_pos[j] {
                    v_str(h, &y)
                } else {
                    print_scalar(&y)
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

// /// Print the evaluation of a polynomial for all elements in the coset.
// pub fn poly_str(h: &Coset, p: &Poly, pos: bool, header: String) -> String {
//     evals_str(h, vec![p], vec![header], vec![pos])
// }

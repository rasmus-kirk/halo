use super::{
    misc::{to_subscript, to_superscript, EnumIter},
    Poly, Scalar,
};
use crate::{scheme::Slots, Coset};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_poly::Polynomial;
use ascii_table::{Align, AsciiTable};

pub fn print_scalar<P: SWCurveConfig>(x: Scalar<P>) -> String {
    let s1 = format!("{}", x);
    let s2 = format!("{}", -x);
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
fn v_str<P: SWCurveConfig>(h: &Coset<P>, x: Scalar<P>) -> String {
    for slot in Slots::iter() {
        for (i, w) in h.k_iter().enumerate() {
            if x == *w {
                return format!("k{}", to_subscript(i as u64));
            }
        }
        for (i_, w) in h.vec_k(slot).into_iter().enumerate() {
            let i = (i_ + 1) as u64;
            if x == w {
                return format!("k{}ω{}", to_subscript(slot as u64), to_superscript(i));
            }
        }
    }
    print_scalar::<P>(x)
}

/// Print the evaluations of a vector of polynomials for all elements in the coset.
pub fn evals_str<P: SWCurveConfig>(
    h: &Coset<P>,
    fs: Vec<&Poly<P>>,
    hs: Vec<String>,
    is_pos: Vec<bool>,
) -> String {
    let data: Vec<Vec<String>> = (0..h.n() + 1)
        .map(|i| {
            let mut row = vec![w_str(i).to_string()];
            row.extend(fs.iter().enumerate().map(|(j, &f)| {
                let y = f.evaluate(&h.w(i));
                if is_pos[j] {
                    v_str(h, y)
                } else {
                    print_scalar::<P>(y)
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

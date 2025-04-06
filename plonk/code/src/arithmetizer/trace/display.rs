use super::{ConstraintID, Pos, Trace};
use crate::{
    pcs::PCS,
    scheme::{Slots, Terms},
    utils::misc::EnumIter,
    Coset,
};

use ark_ec::short_weierstrass::SWCurveConfig;
use ascii_table::{Align, AsciiTable};
use std::fmt::{self, Display};

impl<P: SWCurveConfig, PCST: PCS<P>> Trace<P, PCST> {
    /// Get the debugging table data for the evaluator.
    fn table_data(&self) -> Vec<Vec<String>> {
        self.constraints
            .iter()
            .enumerate()
            .map(|(i_, eqn)| {
                let i = i_ as ConstraintID + 1;
                std::iter::once(format!("{}", Pos::new(Slots::A, i)))
                    .chain(Terms::iter().map(|term| {
                        let value = &eqn[term];
                        if term.is_slot() && value.is_anon() && value.is_zero() {
                            "".to_string()
                        } else {
                            format!("{}", value)
                        }
                    }))
                    .chain(Slots::iter().map(|slot| {
                        let pos = Pos::new(slot, i);
                        format!("{}", self.permutation.get(&pos).unwrap_or(&pos))
                    }))
                    .collect()
            })
            .collect()
    }
}

impl<P: SWCurveConfig, PCST: PCS<P>> Display for Trace<P, PCST> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ascii_table = AsciiTable::default();
        ascii_table.column(0).set_header("").set_align(Align::Left);
        // omega header
        Terms::iter().enumerate().for_each(|(i, term)| {
            ascii_table
                .column(i + 1)
                .set_header(format!("{}", term))
                .set_align(if term.is_slot() {
                    Align::Left
                } else {
                    Align::Right
                });
        });
        // all term headers
        Slots::iter().enumerate().for_each(|(i, slot)| {
            ascii_table
                .column(1 + Terms::COUNT + i)
                .set_header(slot.perm_string().to_string())
                .set_align(Align::Right);
        });
        // permutation header
        writeln!(f, "Trace {{")?;
        if self.h != Coset::default() {
            writeln!(f, "    {},", self.h)?;
        }
        // print omega
        ascii_table
            .format(self.table_data())
            .lines()
            .try_for_each(|line| writeln!(f, "   {}", line))?;
        // print data
        write!(f, "}}")
    }
}

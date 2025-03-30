use super::{value::Value, ConstraintID, Pos, Trace};
use crate::{
    curve::Coset,
    protocol::scheme::{Selectors, Slots, Terms},
};

use halo_accumulation::group::PallasScalar;

use ark_ff::AdditiveGroup;
use ascii_table::{Align, AsciiTable};
use std::fmt;

type Scalar = PallasScalar;

impl Trace {
    /// Get the debugging table data for the evaluator.
    fn table_data(&self) -> Vec<Vec<String>> {
        self.constraints
            .iter()
            .enumerate()
            .map(|(i_, eqn)| {
                let i = (i_ + 1) as ConstraintID;
                let mut row: Vec<String> = vec![format!("{}", Pos::new(Slots::A, i))];
                row.extend(Slots::iter().map(|term| match eqn[Terms::F(term)] {
                    Value::AnonWire(x) if x == Scalar::ZERO => "".to_string(),
                    x => format!("{}", x),
                }));
                row.extend(
                    Selectors::iter().map(|selector| format!("{}", eqn[Terms::Q(selector)])),
                );
                row.extend([eqn[Terms::PublicInputs]].iter().map(|x| format!("{}", x)));
                row.extend(Slots::iter().map(|slot| {
                    let pos = Pos::new(slot, i);
                    format!("{}", self.permutation.get(&pos).unwrap_or(&pos))
                }));
                row
            })
            .collect()
    }
}

impl fmt::Display for Trace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ascii_table = AsciiTable::default();
        ascii_table.column(0).set_header("").set_align(Align::Left);
        for (i, slot) in Slots::iter().enumerate() {
            ascii_table
                .column(1 + i)
                .set_header(format!("{}", slot))
                .set_align(Align::Left);
        }
        for (i, selector) in Selectors::iter().enumerate() {
            ascii_table
                .column(1 + Slots::COUNT + i)
                .set_header(format!("{}", selector))
                .set_align(Align::Right);
        }
        ascii_table
            .column(Terms::COUNT)
            .set_header(format!("{}", Terms::PublicInputs))
            .set_align(Align::Right);
        for (i, slot) in Slots::iter().enumerate() {
            ascii_table
                .column(1 + Terms::COUNT + i)
                .set_header(slot.perm_string().to_string())
                .set_align(Align::Right);
        }
        writeln!(f, "Trace {{")?;
        if self.h != Coset::default() {
            writeln!(f, "    {},", self.h)?;
        }
        for line in ascii_table.format(self.table_data()).lines() {
            writeln!(f, "   {}", line)?;
        }
        write!(f, "}}")
    }
}

use std::fmt::Display;

use crate::{
    curve::{Coset, Scalar},
    protocol::scheme::{Selectors, Slots, Terms},
};
use lazy_static::lazy_static;

use super::trace::Constraints;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(usize)]
pub enum PlonkupOps {
    Xor,
    Or,
}

impl Display for PlonkupOps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            PlonkupOps::Xor => "XOR",
            PlonkupOps::Or => "OR",
        };
        write!(f, "{}", s)
    }
}

impl PlonkupOps {
    pub const COUNT: usize = 2;

    pub fn iter() -> impl Iterator<Item = PlonkupOps> {
        [PlonkupOps::Xor, PlonkupOps::Or].iter().copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Table(Vec<[Scalar; 3]>);

impl Table {
    pub fn new(table: Vec<[Scalar; 3]>) -> Self {
        Self(table)
    }

    pub fn compress(&self, zeta: &Scalar, j: usize, n: u64) -> Vec<Scalar> {
        let mut res = Vec::new();
        for row in self.0.iter() {
            let [a, b, c] = row;
            let t = a + (zeta * b) + (zeta * zeta * c) + (zeta * zeta * zeta * Scalar::from(j));
            res.push(t);
        }
        res.extend(vec![res[0]; n as usize - res.len()]);
        res
    }

    pub fn unwrap(&self) -> Vec<[Scalar; 3]> {
        self.0.clone()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<PlonkupOps> for Table {
    fn from(op: PlonkupOps) -> Self {
        match op {
            // PlonkupOps::Xor => {
            //     let mut rows = Vec::new();
            //     for a in 0..2 ^ 8 {
            //         for b in 0..2 ^ 8 {
            //             let c = a ^ b;
            //             rows.push([Scalar::from(a), Scalar::from(b), Scalar::from(c)]);
            //         }
            //     }
            //     Table::new(rows)
            // }
            PlonkupOps::Xor => Table::new(vec![
                [Scalar::ZERO, Scalar::ZERO, Scalar::ZERO],
                [Scalar::ZERO, Scalar::ONE, Scalar::ONE],
                [Scalar::ONE, Scalar::ZERO, Scalar::ONE],
                [Scalar::ONE, Scalar::ONE, Scalar::ZERO],
            ]),
            PlonkupOps::Or => Table::new(vec![
                [Scalar::ZERO, Scalar::ZERO, Scalar::ZERO],
                [Scalar::ZERO, Scalar::ONE, Scalar::ONE],
                [Scalar::ONE, Scalar::ZERO, Scalar::ONE],
                [Scalar::ONE, Scalar::ONE, Scalar::ONE],
            ]),
        }
    }
}

pub struct TableRegistry {
    tables: [Table; PlonkupOps::COUNT],
}

impl TableRegistry {
    fn new() -> Self {
        let mut tables = Vec::new();
        for op in PlonkupOps::iter() {
            let table = Table::from(op);
            tables.push(table);
        }
        Self {
            tables: tables.try_into().unwrap(),
        }
    }

    pub fn lookup(&self, op: PlonkupOps, a: &Scalar, b: &Scalar) -> Option<Scalar> {
        let table = &self.tables[op as usize];
        for row in table.unwrap() {
            if row[0] == *a && row[1] == *b {
                return Some(row[2]);
            }
        }
        None
    }

    pub fn query(op: PlonkupOps, zeta: &Scalar, a: &Scalar, b: &Scalar) -> Option<Scalar> {
        let c = &TABLE_REGISTRY.lookup(op, a, b)?;
        let j = op as usize;
        Some(Self::eval_compress(zeta, a, b, c, j))
    }

    pub fn eval_compress(zeta: &Scalar, a: &Scalar, b: &Scalar, c: &Scalar, j: usize) -> Scalar {
        a + (zeta * b) + (zeta * zeta * c) + (zeta * zeta * zeta * Scalar::from(j))
    }

    pub fn compute_vecs(
        &self,
        zeta: &Scalar,
        coset: Coset,
        constraints: Vec<Constraints>,
    ) -> (Vec<Scalar>, Vec<Scalar>, Vec<Scalar>) {
        let mut t = Vec::new();
        for op in PlonkupOps::iter() {
            let j = op as usize;
            t.extend(self.tables[j].compress(zeta, j, coset.n()));
        }
        t.sort();
        // table vector

        let mut f = Vec::new();
        for constraint in constraints.iter() {
            if Into::<Scalar>::into(constraint[Terms::Q(Selectors::Qk)]) == Scalar::ONE {
                let a: Scalar = constraint[Terms::F(Slots::A)].into();
                let b: Scalar = constraint[Terms::F(Slots::B)].into();
                let c: Scalar = constraint[Terms::F(Slots::C)].into();
                let j = constraint.lookup as usize;
                f.push(Self::eval_compress(zeta, &a, &b, &c, j));
            } else {
                f.push(t.last().unwrap().clone());
            }
        }
        // query vector

        let mut s = Vec::new();
        s.extend(&t);
        s.extend(&f);
        s.sort();
        // sort vector

        (t, f, s)
    }

    pub fn len(&self) -> usize {
        self.tables.iter().map(|table| table.len()).sum()
    }
}

lazy_static! {
    pub static ref TABLE_REGISTRY: TableRegistry = TableRegistry::new();
}

// 1. Gate Constraints update - Q_k, f, zeta                        X
// 2. Trace update                                                  X
//  - determine n from table length as well                         X
//  - compute polynomials                                           X
//    - qk                                                          X
//    - f                                                           X
//    - h1, h2 / s                                                  X
// 3. Arithmetizer update                                           X
//  - add plonkup ops as constructors in arithwire                  X
//  - add plonkup ops calls to construct arithmetized circuit       X
//    - general lookup with PlonkupOps argument                     X
//    - sugar for Xor and Or (temporary)                            X
//    - add ArithWire to cache                                      X
//    - make Xor and Or commutative (temporary)                     X
// 4. Prover and Verifier update
//  - update F_GC
//  - abstract interface for grand product arguments?
//  - compute Z_PCC
//  - compute F_PCC1 and F_PCC2
//  - update prover and verifier calls
// 5. multi table Xor and Or test

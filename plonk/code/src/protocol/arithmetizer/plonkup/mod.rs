use std::fmt::Display;

use crate::{
    curve::{Coset, Poly, Scalar},
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

    pub fn compress(&self, zeta: &Scalar, j: usize) -> Vec<Scalar> {
        let mut res = Vec::new();
        for row in self.0.iter() {
            let [a, b, c] = row;
            let t = a + (zeta * b) + (zeta * zeta * c) + (zeta * zeta * zeta * Scalar::from(j));
            res.push(t);
        }
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn compute_vecs<'a>(
        &self,
        zeta: &Scalar,
        coset: &Coset,
        constraints: &Vec<Constraints>,
    ) -> [Vec<Scalar>; 5] {
        let mut t = Vec::new();
        for op in PlonkupOps::iter() {
            let j = op as usize;
            t.extend(self.tables[j].compress(zeta, j));
        }
        t.sort();
        let extend = coset.n() as usize - t.len() - 1;
        t.extend(vec![t.last().unwrap().clone(); extend]);
        // table vector

        let mut f = Vec::new();
        let mut js = Vec::new();
        f.push(t.last().unwrap().clone());
        for constraint in constraints.iter() {
            if Into::<Scalar>::into(constraint[Terms::Q(Selectors::Qk)]) == Scalar::ONE {
                let a: Scalar = constraint[Terms::F(Slots::A)].into();
                let b: Scalar = constraint[Terms::F(Slots::B)].into();
                let c: Scalar = constraint[Terms::F(Slots::C)].into();
                let j = constraint.lookup as usize;
                f.push(Self::eval_compress(zeta, &a, &b, &c, j));
                js.push(Scalar::from(j));
            } else {
                f.push(t.last().unwrap().clone());
                js.push(Scalar::ZERO);
            }
        }
        let extend = coset.n() as usize - f.len() - 1;
        f.sort();
        f.extend(vec![f.last().unwrap().clone(); extend]);
        // query vector

        let mut s: Vec<Scalar> = Vec::new();
        s.extend(t.iter());
        s.extend(f.iter());
        s.sort();
        // sort vector

        let mut h1 = Vec::new();
        let mut h2 = Vec::new();
        for (i, x) in s.into_iter().enumerate() {
            if i % 2 == 0 {
                h1.push(x);
            } else {
                h2.push(x);
            }
        }
        [t, f, h1, h2, js]
    }

    pub fn len(&self) -> usize {
        self.tables.iter().map(|table| table.len()).sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlonkupVecCompute {
    coset: Coset,
    constraints: Vec<Constraints>,
}

impl PlonkupVecCompute {
    pub fn new(coset: Coset, constraints: Vec<Constraints>) -> Self {
        Self { coset, constraints }
    }

    pub fn compute(&self, zeta: &Scalar) -> [Poly; 5] {
        TABLE_REGISTRY
            .compute_vecs(zeta, &self.coset, &self.constraints)
            .into_iter()
            .map(|evals| self.coset.interpolate_zf(evals))
            .collect::<Vec<Poly>>()
            .try_into()
            .unwrap()
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
//    - remove Or circuit / comment it out
// 4. Prover and Verifier update
//  - update F_GC                                                   X
//  - abstract interface for grand product arguments?
//  - compute Z_PCC                                                 X
//  - compute F_PCC1 and F_PCC2
//  - update prover and verifier calls
// 5. multi table Xor and Or test

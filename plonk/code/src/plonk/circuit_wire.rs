use super::{
    circuit_abstract::{AbstractCircuit, WireId},
    circuit_print::{map_to_alphabet, scalar_to_str},
    circuit_protocol::Circuit,
};
use halo_accumulation::group::PallasScalar as Fq;
use std::{
    cell::RefCell,
    ops::{Add, Mul, Sub},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct Wire<const L: usize> {
    id: WireId,
    circuit_ref: Rc<RefCell<AbstractCircuit<L>>>,
    string_rep: String,
}
impl<const L: usize> Wire<L> {
    pub fn new(
        id: WireId,
        circuit_ref: Rc<RefCell<AbstractCircuit<L>>>,
        string_rep: String,
    ) -> Self {
        Self {
            id,
            circuit_ref,
            string_rep,
        }
    }

    pub fn default() -> Self {
        Self {
            id: WireId::default(),
            circuit_ref: Rc::new(RefCell::new(AbstractCircuit::new())),
            string_rep: map_to_alphabet(WireId::default()),
        }
    }

    pub fn id(&self) -> WireId {
        self.id
    }

    pub fn circuit(&self) -> &Rc<RefCell<AbstractCircuit<L>>> {
        &self.circuit_ref
    }

    // wrapper calls to `Circuit` ---------------------------------------------

    pub fn n(&self) -> usize {
        self.circuit_ref.borrow().n()
    }

    pub fn public_input_fq(&self, inputs: [Fq; L]) -> Circuit<L> {
        self.circuit_ref.borrow().prepare(inputs)
    }

    pub fn input(&self, inputs: [i64; L]) -> Circuit<L> {
        println!("\nCircuit:\n  {}\n", self.to_string_rep());
        let inputs_vec = inputs.iter().map(|x| Fq::from(*x)).collect::<Vec<Fq>>();
        let inputs_fp = inputs_vec.try_into().unwrap();
        self.circuit_ref.borrow().prepare(inputs_fp)
    }

    pub fn to_string_rep(&self) -> String {
        self.string_rep.to_string()
    }
}

// Add Wire and Wire ----------------------------------------------------------

impl<'a, const L: usize> Add for &'a Wire<L> {
    type Output = Wire<L>;

    fn add(self, other: Self) -> Self::Output {
        let new_id = self.circuit_ref.borrow_mut().add(self.id, other.id);
        Wire::new(
            new_id,
            Rc::clone(&self.circuit_ref),
            format!("{} + {}", self.string_rep, other.string_rep),
        )
    }
}

impl<const L: usize> Add for Wire<L> {
    type Output = Wire<L>;

    fn add(self, other: Self) -> Self::Output {
        let new_id = self.circuit_ref.borrow_mut().add(self.id, other.id);
        Wire::new(
            new_id,
            Rc::clone(&self.circuit_ref),
            format!("{} + {}", self.string_rep, other.string_rep),
        )
    }
}

impl<'a, const L: usize> Add<Wire<L>> for &'a Wire<L> {
    type Output = Wire<L>;

    fn add(self, other: Wire<L>) -> Self::Output {
        let new_id = self.circuit_ref.borrow_mut().add(self.id, other.id);
        Wire::new(
            new_id,
            Rc::clone(&self.circuit_ref),
            format!("{} + {}", self.string_rep, other.string_rep),
        )
    }
}

impl<'a, const L: usize> Add<&'a Wire<L>> for Wire<L> {
    type Output = Wire<L>;

    fn add(self, other: &'a Wire<L>) -> Self::Output {
        let new_id = self.circuit_ref.borrow_mut().add(self.id, other.id);
        Wire::new(
            new_id,
            Rc::clone(&self.circuit_ref),
            format!("{} + {}", self.string_rep, other.string_rep),
        )
    }
}

// Mul Wire and Wire ----------------------------------------------------------

impl<'a, const L: usize> Mul for &'a Wire<L> {
    type Output = Wire<L>;

    fn mul(self, other: Self) -> Self::Output {
        let new_id = self.circuit_ref.borrow_mut().mul(self.id, other.id);
        Wire::new(
            new_id,
            Rc::clone(&self.circuit_ref),
            format!("({} × {})", self.string_rep, other.string_rep),
        )
    }
}

impl<const L: usize> Mul for Wire<L> {
    type Output = Wire<L>;

    fn mul(self, other: Self) -> Self::Output {
        let new_id = self.circuit_ref.borrow_mut().mul(self.id, other.id);
        Wire::new(
            new_id,
            Rc::clone(&self.circuit_ref),
            format!("({} × {})", self.string_rep, other.string_rep),
        )
    }
}

impl<'a, const L: usize> Mul<Wire<L>> for &'a Wire<L> {
    type Output = Wire<L>;

    fn mul(self, other: Wire<L>) -> Self::Output {
        let new_id = self.circuit_ref.borrow_mut().mul(self.id, other.id);
        Wire::new(
            new_id,
            Rc::clone(&self.circuit_ref),
            format!("({} × {})", self.string_rep, other.string_rep),
        )
    }
}

impl<'a, const L: usize> Mul<&'a Wire<L>> for Wire<L> {
    type Output = Wire<L>;

    fn mul(self, other: &'a Wire<L>) -> Self::Output {
        let new_id = self.circuit_ref.borrow_mut().mul(self.id, other.id);
        Wire::new(
            new_id,
            Rc::clone(&self.circuit_ref),
            format!("({} × {})", self.string_rep, other.string_rep),
        )
    }
}

// Add Wire and Constant ------------------------------------------------------

impl<'a, const L: usize> Add<Fq> for &'a Wire<L> {
    type Output = Wire<L>;

    fn add(self, other: Fq) -> Self::Output {
        let new_id = self.circuit_ref.borrow_mut().add_c(self.id, other);
        Wire::new(
            new_id,
            Rc::clone(&self.circuit_ref),
            format!("{} + {}", self.string_rep, scalar_to_str(other)),
        )
    }
}

impl<'a, const L: usize> Sub<Fq> for &'a Wire<L> {
    type Output = Wire<L>;

    fn sub(self, other: Fq) -> Self::Output {
        let new_id = self.circuit_ref.borrow_mut().add_c(self.id, -other);
        Wire::new(
            new_id,
            Rc::clone(&self.circuit_ref),
            format!("{} - {}", self.string_rep, scalar_to_str(other)),
        )
    }
}

impl<'a, const L: usize> Add<&'a Wire<L>> for Fq {
    type Output = Wire<L>;

    fn add(self, other: &'a Wire<L>) -> Self::Output {
        let new_id = other.circuit_ref.borrow_mut().add_c(other.id, self);
        Wire::new(
            new_id,
            Rc::clone(&other.circuit_ref),
            format!("{} + {}", other.string_rep, scalar_to_str(self)),
        )
    }
}

impl<'a, const L: usize> Add<i64> for &'a Wire<L> {
    type Output = Wire<L>;

    fn add(self, other: i64) -> Self::Output {
        let fp_other = Fq::from(other);
        self + fp_other
    }
}

impl<'a, const L: usize> Sub<i64> for &'a Wire<L> {
    type Output = Wire<L>;

    fn sub(self, other: i64) -> Self::Output {
        let fp_other = Fq::from(other);
        self - fp_other
    }
}

impl<'a, const L: usize> Add<&'a Wire<L>> for i64 {
    type Output = Wire<L>;

    fn add(self, other: &'a Wire<L>) -> Self::Output {
        let fp_self = Fq::from(self);
        fp_self + other
    }
}

impl<const L: usize> Add<Fq> for Wire<L> {
    type Output = Wire<L>;

    fn add(self, other: Fq) -> Self::Output {
        let new_id = self.circuit_ref.borrow_mut().add_c(self.id, other);
        Wire::new(
            new_id,
            Rc::clone(&self.circuit_ref),
            format!("{} + {}", self.string_rep, scalar_to_str(other)),
        )
    }
}

impl<const L: usize> Sub<Fq> for Wire<L> {
    type Output = Wire<L>;

    fn sub(self, other: Fq) -> Self::Output {
        let new_id = self.circuit_ref.borrow_mut().add_c(self.id, -other);
        Wire::new(
            new_id,
            Rc::clone(&self.circuit_ref),
            format!("{} - {}", self.string_rep, scalar_to_str(other)),
        )
    }
}

impl<const L: usize> Add<Wire<L>> for Fq {
    type Output = Wire<L>;

    fn add(self, other: Wire<L>) -> Self::Output {
        let new_id = other.circuit_ref.borrow_mut().add_c(other.id, self);
        Wire::new(
            new_id,
            Rc::clone(&other.circuit_ref),
            format!("{} + {}", other.string_rep, scalar_to_str(self)),
        )
    }
}

impl<const L: usize> Add<i64> for Wire<L> {
    type Output = Wire<L>;

    fn add(self, other: i64) -> Self::Output {
        let fp_other = Fq::from(other);
        self + fp_other
    }
}

impl<const L: usize> Sub<i64> for Wire<L> {
    type Output = Wire<L>;

    fn sub(self, other: i64) -> Self::Output {
        let fp_other = Fq::from(other);
        self - fp_other
    }
}

impl<const L: usize> Add<Wire<L>> for i64 {
    type Output = Wire<L>;

    fn add(self, other: Wire<L>) -> Self::Output {
        let fp_self = Fq::from(self);
        fp_self + other
    }
}

// Mul Wire and Constant ------------------------------------------------------

impl<'a, const L: usize> Mul<Fq> for &'a Wire<L> {
    type Output = Wire<L>;

    fn mul(self, other: Fq) -> Self::Output {
        let new_id = self.circuit_ref.borrow_mut().mul_c(self.id, other);
        Wire::new(
            new_id,
            Rc::clone(&self.circuit_ref),
            format!("({} × {})", self.string_rep, scalar_to_str(other)),
        )
    }
}

impl<'a, const L: usize> Mul<&'a Wire<L>> for Fq {
    type Output = Wire<L>;

    fn mul(self, other: &'a Wire<L>) -> Self::Output {
        let new_id = other.circuit_ref.borrow_mut().mul_c(other.id, self);
        Wire::new(
            new_id,
            Rc::clone(&other.circuit_ref),
            format!("({} × {})", other.string_rep, scalar_to_str(self)),
        )
    }
}

impl<'a, const L: usize> Mul<i64> for &'a Wire<L> {
    type Output = Wire<L>;

    fn mul(self, other: i64) -> Self::Output {
        let fp_other = Fq::from(other);
        self * fp_other
    }
}

impl<'a, const L: usize> Mul<&'a Wire<L>> for i64 {
    type Output = Wire<L>;

    fn mul(self, other: &'a Wire<L>) -> Self::Output {
        let fp_self = Fq::from(self);
        fp_self * other
    }
}

impl<const L: usize> Mul<Fq> for Wire<L> {
    type Output = Wire<L>;

    fn mul(self, other: Fq) -> Self::Output {
        let new_id = self.circuit_ref.borrow_mut().mul_c(self.id, other);
        Wire::new(
            new_id,
            Rc::clone(&self.circuit_ref),
            format!("({} × {})", self.string_rep, scalar_to_str(other)),
        )
    }
}

impl<const L: usize> Mul<Wire<L>> for Fq {
    type Output = Wire<L>;

    fn mul(self, other: Wire<L>) -> Self::Output {
        let new_id = other.circuit_ref.borrow_mut().mul_c(other.id, self);
        Wire::new(
            new_id,
            Rc::clone(&other.circuit_ref),
            format!("({} × {})", other.string_rep, scalar_to_str(self)),
        )
    }
}

impl<const L: usize> Mul<i64> for Wire<L> {
    type Output = Wire<L>;

    fn mul(self, other: i64) -> Self::Output {
        let fp_other = Fq::from(other);
        self * fp_other
    }
}

impl<const L: usize> Mul<Wire<L>> for i64 {
    type Output = Wire<L>;

    fn mul(self, other: Wire<L>) -> Self::Output {
        let fp_self = Fq::from(self);
        fp_self * other
    }
}

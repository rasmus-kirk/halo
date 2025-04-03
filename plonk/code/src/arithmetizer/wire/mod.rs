mod ast;
mod op_scalar;
mod op_wire;

use super::{plookup::PlookupOps, Arithmetizer, WireID};
use crate::utils::misc::{if_debug, is_debug};
use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::{FpConfig, MontBackend};
use ark_pallas::{FrConfig, PallasConfig};
use ast::WireAST;
use educe::Educe;

use std::{
    cell::RefCell,
    fmt::{self, Debug, Display},
    rc::Rc,
};

#[derive(Educe)]
#[educe(Clone, PartialEq)]
pub struct Wire<
    Op: PlookupOps,
    const N: usize = 4,
    C: FpConfig<N> = MontBackend<FrConfig, 4>,
    P: SWCurveConfig = PallasConfig,
> {
    id: WireID,
    arith: Rc<RefCell<Arithmetizer<Op, N, C, P>>>,
    ast: Option<Rc<WireAST<Op, N, C>>>,
}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>, P: SWCurveConfig> Wire<Op, N, C, P> {
    // constructors -------------------------------------------------------

    pub fn new(
        id: WireID,
        arith: Rc<RefCell<Arithmetizer<Op, N, C, P>>>,
        ast: Option<Rc<WireAST<Op, N, C>>>,
    ) -> Self {
        Self { id, arith, ast }
    }

    /// Create a new input wire.
    pub fn new_input(id: WireID, arith: Rc<RefCell<Arithmetizer<Op, N, C, P>>>) -> Self {
        Self::new(id, arith, if_debug(Rc::new(WireAST::Input(id))))
    }

    // getters ------------------------------------------------------------

    /// Returns the unique identifier of the wire.
    pub fn id(&self) -> WireID {
        self.id
    }

    /// Returns the circuit that the wire belongs to.
    pub fn arith(&self) -> &Rc<RefCell<Arithmetizer<Op, N, C, P>>> {
        &self.arith
    }

    // operations ----------------------------------------------------------

    /// Requires that the wire is a bit
    pub fn is_bit(&self) -> Self {
        let mut arith = self.arith().borrow_mut();
        if let Err(e) = arith.enforce_bit(self.id) {
            panic!("Failed to enforce bit: {}", e);
        }
        self.clone()
    }

    /// Publicize the wire's value to Q꜀.
    pub fn is_public(&self) -> Self {
        let mut arith = self.arith().borrow_mut();
        arith.publicize(self.id);
        self.clone()
    }
}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>, P: SWCurveConfig> Display
    for Wire<Op, N, C, P>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if is_debug() {
            write!(f, "{}", self.ast.clone().unwrap())
        } else {
            write!(f, "AST only computed in `RUST_LOG=debug`")
        }
    }
}

impl<Op: PlookupOps, const N: usize, C: FpConfig<N>, P: SWCurveConfig> Debug for Wire<Op, N, C, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wire: {}", self)
    }
}

#[cfg(test)]
mod tests {
    use ark_ff::Field;
    use halo_accumulation::group::PallasScalar;

    use super::*;
    use crate::{
        arithmetizer::plookup::EmptyOpSet,
        utils::misc::{map_to_alphabet, tests::on_debug},
    };

    type PallasArithmetizer = Arithmetizer<EmptyOpSet, 4, MontBackend<FrConfig, 4>, PallasConfig>;

    #[test]
    fn new() {
        on_debug();
        let [wire_, _] = &PallasArithmetizer::build::<2>();
        let arithmetizer = wire_.arith().clone();
        let wire = Wire::new_input(0, arithmetizer);
        assert_eq!(wire.id, 0);
        assert_eq!(format!("{}", wire), map_to_alphabet(0));
    }

    #[test]
    fn add() {
        on_debug();
        let [a, b] = PallasArithmetizer::build::<2>();
        let c = a.clone() + b.clone();
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 1);
        assert_eq!(c.id, 2);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), map_to_alphabet(1));
        assert_eq!(
            format!("{}", c),
            format!("(+ {} {})", map_to_alphabet(0), map_to_alphabet(1))
        );
    }

    #[test]
    fn sub() {
        on_debug();
        let [a, b] = PallasArithmetizer::build::<2>();
        let c = a.clone() - b.clone();
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 1);
        assert_eq!(c.id, 4);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), map_to_alphabet(1));
        assert_eq!(
            format!("{}", c),
            format!("(+ {} (* {} -1))", map_to_alphabet(0), map_to_alphabet(1),)
        );
    }

    #[test]
    fn mul() {
        on_debug();
        let [a, b] = PallasArithmetizer::build::<2>();
        let c = a.clone() * b.clone();
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 1);
        assert_eq!(c.id, 2);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), map_to_alphabet(1));
        assert_eq!(
            format!("{}", c),
            format!("(* {} {})", map_to_alphabet(0), map_to_alphabet(1))
        );
    }

    #[test]
    fn add_const() {
        on_debug();
        let [a] = PallasArithmetizer::build::<1>();
        let b = a.clone() + PallasScalar::ONE;
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 2);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), format!("(+ {} 1)", map_to_alphabet(0)));
    }

    #[test]
    fn sub_const() {
        on_debug();
        let [a] = PallasArithmetizer::build::<1>();
        let b = a.clone() - PallasScalar::ONE;
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 2);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), format!("(+ {} -1)", map_to_alphabet(0)));
    }

    #[test]
    fn mul_const() {
        on_debug();
        let [a] = PallasArithmetizer::build::<1>();
        let b = a.clone() * PallasScalar::ONE;
        assert_eq!(a.id, 0);
        assert_eq!(b.id, 2);
        assert_eq!(format!("{}", a), map_to_alphabet(0));
        assert_eq!(format!("{}", b), format!("(* {} 1)", map_to_alphabet(0)));
    }
}

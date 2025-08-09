#![allow(non_snake_case)]

use std::{
    fmt,
    ops::{Add, Div, Mul, Neg, Sub},
    sync::OnceLock,
};

use crate::{
    group::{Affine, BaseField, Scalar},
    poseidon_consts::{FP_MDS, FP_ROUND_CONSTANTS, FQ_MDS, FQ_ROUND_CONSTANTS},
    pp::PublicParams,
    Fp, Fq, Point,
};
use ark_ec::{
    short_weierstrass::{Projective, SWCurveConfig},
    AffineRepr, CurveConfig, CurveGroup,
};
use ark_ff::{BigInt, BigInteger, FftField, Field, PrimeField};
use ark_pallas::PallasConfig;
use ark_std::{One, Zero};
use ark_vesta::VestaConfig;
use bincode::{Decode, Encode};

include!(concat!(env!("OUT_DIR"), "/pallas/pp_paths.rs"));
include!(concat!(env!("OUT_DIR"), "/vesta/pp_paths.rs"));

static PP_PALLAS: OnceLock<PublicParams<PallasConfig>> = OnceLock::new();
static PP_VESTA: OnceLock<PublicParams<VestaConfig>> = OnceLock::new();

#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PastaFieldId {
    /// Pallas Scalar Field, Vesta Base Field
    Fp = 0,
    /// Vesta Scalar Field, Pallas Base Field
    Fq = 1,
}
impl PastaFieldId {
    pub fn inv(&self) -> Self {
        match self {
            PastaFieldId::Fp => PastaFieldId::Fq,
            PastaFieldId::Fq => PastaFieldId::Fp,
        }
    }
    pub fn poseidon_round_constants(self) -> [[PastaFE; 3]; 55] {
        match self {
            PastaFieldId::Fp => FP_ROUND_CONSTANTS.map(|x| x.map(Into::into)),
            PastaFieldId::Fq => FQ_ROUND_CONSTANTS.map(|x| x.map(Into::into)),
        }
    }

    pub fn poseidon_mde_matrix(self) -> [[PastaFE; 3]; 3] {
        match self {
            PastaFieldId::Fp => FP_MDS.map(|x| x.map(Into::into)),
            PastaFieldId::Fq => FQ_MDS.map(|x| x.map(Into::into)),
        }
    }
}
impl std::fmt::Display for PastaFieldId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PastaFieldId::Fp => write!(f, "Fp"),
            PastaFieldId::Fq => write!(f, "Fq"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PastaAffine {
    pub x: PastaFE,
    pub y: PastaFE,
    fid: Option<PastaFieldId>,
}
impl std::fmt::Display for PastaAffine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}({}, {})", self.fid, self.x.inner, self.y.inner)
    }
}
impl PastaAffine {
    pub fn new(x: PastaFE, y: PastaFE) -> Self {
        let fid = match (x.fid, y.fid) {
            (Some(x_fid), Some(y_fid)) if x_fid == y_fid => Some(x_fid),
            (None, Some(y_fid)) => Some(y_fid),
            (Some(x_fid), None) => Some(x_fid),
            (None, None) => None,
            _ => panic!("PastaAffine Field Id's did not match!"),
        };
        Self { x, y, fid }
    }
    pub fn identity(fid: Option<PastaFieldId>) -> Self {
        Self::new(PastaFE::zero(fid), PastaFE::zero(fid))
    }
}
impl From<Affine<PallasConfig>> for PastaAffine {
    fn from(value: Affine<PallasConfig>) -> Self {
        if value.is_zero() {
            PastaAffine::identity(Some(PastaFieldId::Fq))
        } else {
            let x = PastaFE::from(value.x);
            let y = PastaFE::from(value.y);
            Self::new(x, y)
        }
    }
}
impl From<Affine<VestaConfig>> for PastaAffine {
    fn from(value: Affine<VestaConfig>) -> Self {
        if value.is_zero() {
            PastaAffine::identity(Some(PastaFieldId::Fp))
        } else {
            let x = PastaFE::from(value.x);
            let y = PastaFE::from(value.y);
            Self::new(x, y)
        }
    }
}
impl From<PastaAffine> for Affine<PallasConfig> {
    fn from(value: PastaAffine) -> Affine<PallasConfig> {
        if value.x == PastaFE::zero(None) && value.x == PastaFE::zero(None) {
            Affine::<PallasConfig>::identity()
        } else {
            Affine::<PallasConfig>::new(value.x.into(), value.y.into())
        }
    }
}
impl From<PastaAffine> for Affine<VestaConfig> {
    fn from(value: PastaAffine) -> Affine<VestaConfig> {
        if value.x == PastaFE::zero(None) && value.x == PastaFE::zero(None) {
            Affine::<VestaConfig>::identity()
        } else {
            Affine::<VestaConfig>::new(value.x.into(), value.y.into())
        }
    }
}
impl Neg for PastaAffine {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.x, -self.y)
    }
}
impl Add for PastaAffine {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self.fid, other.fid) {
            (Some(PastaFieldId::Fp), Some(PastaFieldId::Fp))
            | (Some(PastaFieldId::Fp), _)
            | (_, Some(PastaFieldId::Fp)) => {
                let res = Affine::<VestaConfig>::from(self) + Affine::<VestaConfig>::from(other);
                res.into_affine().into()
            }
            (Some(PastaFieldId::Fq), Some(PastaFieldId::Fq))
            | (Some(PastaFieldId::Fq), _)
            | (_, Some(PastaFieldId::Fq)) => {
                let res = Affine::<PallasConfig>::from(self) + Affine::<PallasConfig>::from(other);
                res.into_affine().into()
            }
            (None, None) => panic!("No Field ID found for PastaFE!"),
        }
    }
}

#[derive(Clone, Copy, Eq)]
pub struct PastaFE {
    inner: BigInt<4>,
    is_neg_one: bool,
    pub fid: Option<PastaFieldId>,
}
impl PastaFE {
    pub fn new(inner: BigInt<4>, fid: Option<PastaFieldId>) -> Self {
        Self {
            inner,
            is_neg_one: false,
            fid,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.inner.is_zero()
    }

    pub fn neg_one() -> Self {
        Self {
            inner: BigInt::zero(),
            is_neg_one: true,
            fid: None,
        }
    }

    pub fn from_u64(x: u64, fid: Option<PastaFieldId>) -> Self {
        Self::new(BigInt::from(x), fid)
    }

    pub fn from_bool(b: bool, fid: Option<PastaFieldId>) -> Self {
        if b {
            Self::new(BigInt::<4>::one(), fid)
        } else {
            Self::new(BigInt::<4>::zero(), fid)
        }
    }

    pub fn from_scalar<P: PastaConfig>(x: P::ScalarField) -> Self {
        Self::new(P::scalar_into_bigint(x), Some(P::SFID))
    }

    pub fn from_basefield<P: PastaConfig>(x: P::BaseField) -> Self {
        Self::new(P::basefield_into_bigint(x), Some(P::BFID))
    }

    pub fn into_bigint(&self) -> BigInt<4> {
        if self.is_neg_one {
            panic!("Can't convert -1 to bigint")
        } else {
            self.inner
        }
    }

    pub fn zero(fid: Option<PastaFieldId>) -> Self {
        Self::new(BigInt::<4>::zero(), fid)
    }

    pub fn one(fid: Option<PastaFieldId>) -> Self {
        Self::new(BigInt::<4>::one(), fid)
    }

    pub fn pow(&self, i: usize) -> Self {
        assert!(!self.is_neg_one);
        match self.fid {
            Some(PastaFieldId::Fp) => Fp::from(*self).pow([i as u64]).into(),
            Some(PastaFieldId::Fq) => Fq::from(*self).pow([i as u64]).into(),
            None => panic!("No Field ID found for PastaFE!"),
        }
    }

    pub fn square(&self) -> Self {
        assert!(!self.is_neg_one);
        match self.fid {
            Some(PastaFieldId::Fp) => Fp::from(*self).square().into(),
            Some(PastaFieldId::Fq) => Fq::from(*self).square().into(),
            None => panic!("No Field ID found for PastaFE!"),
        }
    }

    pub fn inverse(&self) -> Option<Self> {
        assert!(!self.is_neg_one);
        match self.fid {
            Some(PastaFieldId::Fp) => Some(Fp::from(*self).inverse()?.into()),
            Some(PastaFieldId::Fq) => Some(Fq::from(*self).inverse()?.into()),
            None => panic!("No Field ID found for PastaFE!"),
        }
    }

    pub fn inv0(&self) -> Self {
        assert!(!self.is_neg_one);
        if self.is_zero() {
            *self
        } else {
            self.inverse().unwrap()
        }
    }
}
impl std::fmt::Debug for PastaFE {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.is_neg_one, self.fid) {
            (false, Some(fid)) => write!(f, "{fid}({})", self.inner),
            (false, None) => write!(f, "NoID({})", self.inner),
            (true, Some(fid)) => write!(f, "{fid}(-1)"),
            (true, None) => write!(f, "NoID(-1)"),
        }
    }
}
impl PartialEq for PastaFE {
    fn eq(&self, other: &Self) -> bool {
        match (self.fid, other.fid) {
            (Some(self_fid), Some(other_fid)) if !(self_fid == other_fid) => {
                panic!("Can't compare two PastaFE with different FID's!")
            }
            (_, _) => (),
        }
        self.inner == other.inner && self.is_neg_one == other.is_neg_one
    }
}
impl From<u64> for PastaFE {
    fn from(value: u64) -> Self {
        Self::new(BigInt::from(value), None)
    }
}
impl From<Fp> for PastaFE {
    fn from(value: Fp) -> Self {
        Self::new(value.into_bigint(), Some(PastaFieldId::Fp))
    }
}
impl From<Fq> for PastaFE {
    fn from(value: Fq) -> Self {
        Self::new(value.into_bigint(), Some(PastaFieldId::Fq))
    }
}
impl From<PastaFE> for Fp {
    fn from(value: PastaFE) -> Fp {
        if value.is_neg_one {
            -Fp::one()
        } else {
            Fp::new(value.inner)
        }
    }
}
impl From<PastaFE> for Fq {
    fn from(value: PastaFE) -> Fq {
        if value.is_neg_one {
            -Fq::one()
        } else {
            Fq::new(value.inner)
        }
    }
}
impl Neg for PastaFE {
    type Output = Self;

    fn neg(self) -> Self::Output {
        assert!(!self.is_neg_one);
        match self.fid {
            Some(PastaFieldId::Fp) => Self::from(-Fp::from(self)),
            Some(PastaFieldId::Fq) => Self::from(-Fq::from(self)),
            None => panic!("No Field ID found for PastaFE!"),
        }
    }
}
impl Add for PastaFE {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        assert!(!self.is_neg_one && !other.is_neg_one);
        match (self.fid, other.fid) {
            (Some(PastaFieldId::Fp), Some(PastaFieldId::Fp))
            | (Some(PastaFieldId::Fp), _)
            | (_, Some(PastaFieldId::Fp)) => (Fp::from(self.inner) + Fp::from(other.inner)).into(),
            (Some(PastaFieldId::Fq), Some(PastaFieldId::Fq))
            | (Some(PastaFieldId::Fq), _)
            | (_, Some(PastaFieldId::Fq)) => (Fq::from(self.inner) + Fq::from(other.inner)).into(),
            (None, None) => panic!("No Field ID found for PastaFE!"),
        }
    }
}
impl Mul for PastaFE {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        assert!(!self.is_neg_one && !other.is_neg_one);
        match (self.fid, other.fid) {
            (Some(PastaFieldId::Fp), Some(PastaFieldId::Fp))
            | (Some(PastaFieldId::Fp), _)
            | (_, Some(PastaFieldId::Fp)) => (Fp::from(self.inner) * Fp::from(other.inner)).into(),
            (Some(PastaFieldId::Fq), Some(PastaFieldId::Fq))
            | (Some(PastaFieldId::Fq), _)
            | (_, Some(PastaFieldId::Fq)) => (Fq::from(self.inner) * Fq::from(other.inner)).into(),
            (None, None) => panic!("No Field ID found for PastaFE!"),
        }
    }
}
impl Div for PastaFE {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        assert!(!self.is_neg_one && !other.is_neg_one);
        match (self.fid, other.fid) {
            (Some(PastaFieldId::Fp), Some(PastaFieldId::Fp))
            | (Some(PastaFieldId::Fp), _)
            | (_, Some(PastaFieldId::Fp)) => (Fp::from(self.inner) / Fp::from(other.inner)).into(),
            (Some(PastaFieldId::Fq), Some(PastaFieldId::Fq))
            | (Some(PastaFieldId::Fq), _)
            | (_, Some(PastaFieldId::Fq)) => (Fq::from(self.inner) / Fq::from(other.inner)).into(),
            (None, None) => panic!("No Field ID found for PastaFE!"),
        }
    }
}
impl Sub for PastaFE {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        assert!(!self.is_neg_one && !other.is_neg_one);
        match (self.fid, other.fid) {
            (Some(PastaFieldId::Fp), Some(PastaFieldId::Fp))
            | (Some(PastaFieldId::Fp), _)
            | (_, Some(PastaFieldId::Fp)) => (Fp::from(self.inner) - Fp::from(other.inner)).into(),
            (Some(PastaFieldId::Fq), Some(PastaFieldId::Fq))
            | (Some(PastaFieldId::Fq), _)
            | (_, Some(PastaFieldId::Fq)) => (Fq::from(self.inner) - Fq::from(other.inner)).into(),
            (None, None) => panic!("No Field ID found for PastaFE!"),
        }
    }
}

pub enum PastaScalar {
    Pallas(Scalar<PallasConfig>),
    Vesta(Scalar<VestaConfig>),
}
impl PastaScalar {
    pub fn zero(fid: PastaFieldId) -> Self {
        match fid {
            PastaFieldId::Fp => PastaScalar::Pallas(Fp::zero()),
            PastaFieldId::Fq => PastaScalar::Vesta(Fq::zero()),
        }
    }

    pub fn one(fid: PastaFieldId) -> Self {
        match fid {
            PastaFieldId::Fp => PastaScalar::Pallas(Fp::one()),
            PastaFieldId::Fq => PastaScalar::Vesta(Fq::one()),
        }
    }

    pub fn fid(&self) -> PastaFieldId {
        match self {
            PastaScalar::Pallas(_) => PastaFieldId::Fp,
            PastaScalar::Vesta(_) => PastaFieldId::Fq,
        }
    }
}

pub trait PastaConfig: SWCurveConfig
where
    Self::ScalarField: std::fmt::Debug,
    Self::ScalarField: Clone,
    Self: Clone,
    Self: Copy,
{
    type OtherCurve: PastaConfig<OtherCurve = Self>;
    type SF: PrimeField;
    type BF: PrimeField;
    fn get_loaded_public_params() -> &'static OnceLock<PublicParams<Self>>;
    fn get_g_data() -> [&'static [u8]; 64];
    fn get_sh_data() -> &'static [u8];
    fn wrap_projective(p: Projective<Self>) -> WrappedPoint;
    fn unwrap_projective(w: WrappedPoint) -> Projective<Self>;
    fn wrap_affine(p: Affine<Self>) -> WrappedPoint;
    fn unwrap_affine(w: WrappedPoint) -> Affine<Self>;
    fn basefield_into_bigint(x: BaseField<Self>) -> BigInt<4>;
    fn scalar_into_bigint(x: Scalar<Self>) -> BigInt<4>;
    fn basefield_from_bigint(x: BigInt<4>) -> Option<BaseField<Self>>;
    fn scalar_from_bigint(x: BigInt<4>) -> Option<Scalar<Self>>;
    fn basefield_from_u64(x: u64) -> BaseField<Self>;
    fn scalar_from_u64(x: u64) -> Scalar<Self>;
    fn into_pastafe<T: Into<PastaFE>>(x: T) -> PastaFE;

    const IS_PALLAS: bool;
    const SFID: PastaFieldId;
    const BFID: PastaFieldId;
    const SCALAR_POSEIDON_MDS: [[Scalar<Self>; 3]; 3];
    const BASE_POSEIDON_MDS: [[Self::BaseField; 3]; 3];
    const SCALAR_POSEIDON_ROUND_CONSTANTS: [[Self::ScalarField; 3]; 55];
    const BASE_POSEIDON_ROUND_CONSTANTS: [[Self::BaseField; 3]; 55];
    const SCALAR_MODULUS: BigInt<4>;
    const BASE_MODULUS: BigInt<4>;
    const CURVE_NAME: &'static str;
}

impl PastaConfig for PallasConfig {
    type OtherCurve = VestaConfig;
    type SF = <VestaConfig as CurveConfig>::ScalarField;
    type BF = <VestaConfig as CurveConfig>::BaseField;
    fn wrap_projective(p: Projective<Self>) -> WrappedPoint {
        p.into()
    }
    fn unwrap_projective(w: WrappedPoint) -> Projective<Self> {
        w.into()
    }
    fn get_g_data() -> [&'static [u8]; 64] {
        G_PATHS_PALLAS
    }
    fn get_sh_data() -> &'static [u8] {
        SH_PATH_PALLAS
    }
    fn get_loaded_public_params() -> &'static OnceLock<PublicParams<Self>> {
        &PP_PALLAS
    }
    fn wrap_affine(p: Affine<Self>) -> WrappedPoint {
        p.into()
    }
    fn unwrap_affine(w: WrappedPoint) -> Affine<Self> {
        w.into()
    }
    fn basefield_into_bigint(x: BaseField<Self>) -> BigInt<4> {
        x.into_bigint()
    }
    fn scalar_into_bigint(x: Scalar<Self>) -> BigInt<4> {
        x.into_bigint()
    }
    fn basefield_from_bigint(x: BigInt<4>) -> Option<BaseField<Self>> {
        BaseField::<Self>::from_bigint(x)
    }
    fn scalar_from_bigint(x: BigInt<4>) -> Option<Scalar<Self>> {
        Scalar::<Self>::from_bigint(x)
    }
    fn basefield_from_u64(x: u64) -> BaseField<Self> {
        BaseField::<PallasConfig>::from_bigint(BigInt::from(x)).unwrap()
    }
    fn scalar_from_u64(x: u64) -> Scalar<Self> {
        Scalar::<PallasConfig>::from_bigint(BigInt::from(x)).unwrap()
    }
    fn into_pastafe<T: Into<PastaFE>>(x: T) -> PastaFE {
        x.into()
    }

    const IS_PALLAS: bool = true;
    const SFID: PastaFieldId = PastaFieldId::Fp;
    const BFID: PastaFieldId = PastaFieldId::Fq;
    const SCALAR_POSEIDON_MDS: [[<PallasConfig as CurveConfig>::ScalarField; 3]; 3] = FP_MDS;
    const BASE_POSEIDON_MDS: [[<PallasConfig as CurveConfig>::BaseField; 3]; 3] = FQ_MDS;
    const SCALAR_POSEIDON_ROUND_CONSTANTS: [[<PallasConfig as CurveConfig>::ScalarField; 3]; 55] =
        FP_ROUND_CONSTANTS;
    const BASE_POSEIDON_ROUND_CONSTANTS: [[<PallasConfig as CurveConfig>::BaseField; 3]; 55] =
        FQ_ROUND_CONSTANTS;
    const SCALAR_MODULUS: BigInt<4> = <PallasConfig as CurveConfig>::ScalarField::MODULUS;
    const BASE_MODULUS: BigInt<4> = <PallasConfig as CurveConfig>::BaseField::MODULUS;
    const CURVE_NAME: &'static str = "Pallas";
}

impl PastaConfig for VestaConfig {
    type OtherCurve = PallasConfig;
    type SF = <PallasConfig as CurveConfig>::ScalarField;
    type BF = <PallasConfig as CurveConfig>::BaseField;
    fn wrap_projective(p: Projective<Self>) -> WrappedPoint {
        p.into()
    }
    fn unwrap_projective(w: WrappedPoint) -> Projective<Self> {
        w.into()
    }
    fn wrap_affine(p: Affine<Self>) -> WrappedPoint {
        p.into()
    }
    fn unwrap_affine(w: WrappedPoint) -> Affine<Self> {
        w.into()
    }
    fn get_g_data() -> [&'static [u8]; 64] {
        G_PATHS_VESTA
    }
    fn get_sh_data() -> &'static [u8] {
        SH_PATH_VESTA
    }
    fn get_loaded_public_params() -> &'static OnceLock<PublicParams<Self>> {
        &PP_VESTA
    }
    fn basefield_into_bigint(x: BaseField<Self>) -> BigInt<4> {
        x.into_bigint()
    }
    fn scalar_into_bigint(x: Scalar<Self>) -> BigInt<4> {
        x.into_bigint()
    }
    fn basefield_from_bigint(x: BigInt<4>) -> Option<BaseField<Self>> {
        BaseField::<Self>::from_bigint(x)
    }
    fn scalar_from_bigint(x: BigInt<4>) -> Option<Scalar<Self>> {
        Scalar::<Self>::from_bigint(x)
    }
    fn basefield_from_u64(x: u64) -> BaseField<Self> {
        BaseField::<VestaConfig>::from_bigint(BigInt::from(x)).unwrap()
    }
    fn scalar_from_u64(x: u64) -> Scalar<Self> {
        Scalar::<VestaConfig>::from_bigint(BigInt::from(x)).unwrap()
    }
    fn into_pastafe<T: Into<PastaFE>>(x: T) -> PastaFE {
        x.into()
    }

    const IS_PALLAS: bool = false;
    const SFID: PastaFieldId = PastaFieldId::Fq;
    const BFID: PastaFieldId = PastaFieldId::Fp;
    const SCALAR_POSEIDON_MDS: [[<VestaConfig as CurveConfig>::ScalarField; 3]; 3] = FQ_MDS;
    const BASE_POSEIDON_MDS: [[<VestaConfig as CurveConfig>::BaseField; 3]; 3] = FP_MDS;
    const SCALAR_POSEIDON_ROUND_CONSTANTS: [[<VestaConfig as CurveConfig>::ScalarField; 3]; 55] =
        FQ_ROUND_CONSTANTS;
    const BASE_POSEIDON_ROUND_CONSTANTS: [[<VestaConfig as CurveConfig>::BaseField; 3]; 55] =
        FP_ROUND_CONSTANTS;
    const SCALAR_MODULUS: BigInt<4> = <VestaConfig as CurveConfig>::ScalarField::MODULUS;
    const BASE_MODULUS: BigInt<4> = <VestaConfig as CurveConfig>::BaseField::MODULUS;
    const CURVE_NAME: &'static str = "Vesta";
}

// -------------------- Wrappers -------------------- //

#[derive(Debug, Clone, Decode, Encode, PartialEq, Eq)]
#[repr(C)]
pub struct WrappedPoint {
    x: [u64; 4],
    y: [u64; 4],
}

// ---------- Pallas ---------- //

impl From<WrappedPoint> for ark_pallas::Affine {
    fn from(value: WrappedPoint) -> Self {
        let x = ark_pallas::Fq::new_unchecked(BigInt(value.x));
        let y = ark_pallas::Fq::new_unchecked(BigInt(value.y));
        let affine = ark_pallas::Affine::new_unchecked(x, y);
        assert!(affine.is_on_curve());
        affine
    }
}

impl From<ark_pallas::Affine> for WrappedPoint {
    fn from(value: ark_pallas::Affine) -> Self {
        WrappedPoint {
            x: value.x.0 .0,
            y: value.y.0 .0,
        }
    }
}

impl From<WrappedPoint> for ark_pallas::Projective {
    fn from(value: WrappedPoint) -> Self {
        let affine: ark_pallas::Affine = value.into();
        affine.into()
    }
}

impl From<ark_pallas::Projective> for WrappedPoint {
    fn from(value: ark_pallas::Projective) -> Self {
        value.into_affine().into()
    }
}

// ---------- Vesta ---------- //

impl From<WrappedPoint> for ark_vesta::Affine {
    fn from(value: WrappedPoint) -> Self {
        let x = ark_vesta::Fq::new_unchecked(BigInt(value.x));
        let y = ark_vesta::Fq::new_unchecked(BigInt(value.y));
        let affine = ark_vesta::Affine::new_unchecked(x, y);
        assert!(affine.is_on_curve());
        affine
    }
}

impl From<ark_vesta::Affine> for WrappedPoint {
    fn from(value: ark_vesta::Affine) -> Self {
        WrappedPoint {
            x: value.x.0 .0,
            y: value.y.0 .0,
        }
    }
}

impl From<WrappedPoint> for ark_vesta::Projective {
    fn from(value: WrappedPoint) -> Self {
        let affine: ark_vesta::Affine = value.into();
        affine.into()
    }
}

impl From<ark_vesta::Projective> for WrappedPoint {
    fn from(value: ark_vesta::Projective) -> Self {
        value.into_affine().into()
    }
}

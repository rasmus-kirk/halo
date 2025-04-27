#![allow(non_snake_case)]

use std::sync::OnceLock;

use crate::{consts::{G_BLOCKS_NO, G_BLOCKS_SIZE, N}, group::{Affine, Scalar}, pp::PublicParams};
use ark_ec::{short_weierstrass::{Projective, SWCurveConfig}, CurveGroup};
use ark_ff::{BigInt, PrimeField};
use ark_pallas::PallasConfig;
use ark_vesta::VestaConfig;
use bincode::{config::standard, Decode, Encode};

include!(concat!(env!("OUT_DIR"), "/pallas/pp_paths.rs"));
include!(concat!(env!("OUT_DIR"), "/vesta/pp_paths.rs"));

static PP_PALLAS: OnceLock<PublicParams<PallasConfig>> = OnceLock::new();
static PP_VESTA:  OnceLock<PublicParams<VestaConfig>>  = OnceLock::new();

pub trait PastaConfig: SWCurveConfig
where
{
    fn get_loaded_public_params() -> &'static OnceLock<PublicParams<Self>>;
    fn get_g_data() -> [&'static [u8]; 64];
    fn get_sh_data() -> &'static [u8];
    fn wrap_projective(p: Projective<Self>) -> WrappedPoint;
    fn unwrap_projective(w: WrappedPoint) -> Projective<Self>;
    fn wrap_affine(p: Affine<Self>) -> WrappedPoint;
    fn unwrap_affine(w: WrappedPoint) -> Affine<Self>;
}

impl PastaConfig for ark_pallas::PallasConfig {
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
}

impl PastaConfig for ark_vesta::VestaConfig {
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

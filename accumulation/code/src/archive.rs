use ark_ec::CurveGroup;
use ark_ff::BigInt;
use ark_pallas::{Affine, Fq, Fr, Projective};
use ark_poly::univariate::DensePolynomial;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
pub use bincode::config::standard as std_config;
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Decode, Encode, PartialEq, Eq)]
#[repr(C)]
pub struct WrappedPoint {
    x: WrappedFq,
    y: WrappedFq,
}

#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct WrappedPoly(Vec<u8>);

impl From<DensePolynomial<Fr>> for WrappedPoly {
    fn from(value: DensePolynomial<Fr>) -> Self {
        let mut data = Vec::with_capacity(value.compressed_size());
        value.serialize_compressed(&mut data).unwrap();
        WrappedPoly(data)
    }
}

impl From<WrappedPoly> for DensePolynomial<Fr> {
    fn from(value: WrappedPoly) -> Self {
        DensePolynomial::deserialize_uncompressed_unchecked(value.0.as_slice()).unwrap()
    }
}

impl From<WrappedPoint> for Affine {
    fn from(value: WrappedPoint) -> Self {
        let affine = Affine::new_unchecked(value.x.into(), value.y.into());
        assert!(affine.is_on_curve());
        affine
    }
}

impl From<WrappedPoint> for Projective {
    fn from(value: WrappedPoint) -> Self {
        let p_aff: Affine = value.into();
        p_aff.into()
    }
}

impl From<Affine> for WrappedPoint {
    fn from(value: Affine) -> Self {
        WrappedPoint {
            x: value.x.into(),
            y: value.y.into(),
        }
    }
}

impl From<Projective> for WrappedPoint {
    fn from(value: Projective) -> Self {
        let p = value.into_affine();
        WrappedPoint {
            x: p.x.into(),
            y: p.y.into(),
        }
    }
}

#[derive(Debug, Clone, Decode, Encode, PartialEq, Eq)]
#[repr(C)]
pub struct WrappedScalar([u64; 4]);

impl From<Fr> for WrappedScalar {
    fn from(value: Fr) -> Self {
        Self(value.0 .0)
    }
}

impl From<WrappedScalar> for Fr {
    fn from(value: WrappedScalar) -> Self {
        Self::new_unchecked(BigInt(value.0))
    }
}

#[derive(Debug, Clone, Decode, Encode, PartialEq, Eq)]
#[repr(C)]
pub struct WrappedFq([u64; 4]);

impl From<Fq> for WrappedFq {
    fn from(value: Fq) -> Self {
        Self(value.0 .0)
    }
}

impl From<WrappedFq> for Fq {
    fn from(value: WrappedFq) -> Self {
        Self::new_unchecked(BigInt(value.0))
    }
}

use ark_ec::CurveGroup;
use ark_pallas::{Affine, Fr};
use ark_poly::univariate::DensePolynomial;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
pub use bincode::config::standard as std_config;
use bincode::{Decode, Encode};

include!("./consts.rs");

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

// TODO: Make this actually safe, it's fine as long as the binary is not distributed though
// One problem is endianness, maybe that's it actually. Should be solved, there is some stuff about alignment though
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
        let p: Projective = p_aff.into();
        Projective::new_unchecked(
            Fq::new_unchecked(p.x.0),
            Fq::new_unchecked(p.y.0),
            Fq::new_unchecked(p.z.0),
        )
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
        let a = value.0 .0[0].into();
        let b = value.0 .0[1].into();
        let c = value.0 .0[2].into();
        let d = value.0 .0[3].into();
        WrappedScalar([a, b, c, d])
    }
}

impl From<WrappedScalar> for Fr {
    fn from(value: WrappedScalar) -> Self {
        let a = value.0[0].into();
        let b = value.0[1].into();
        let c = value.0[2].into();
        let d = value.0[3].into();
        Fr::new_unchecked(BigInt([a, b, c, d]))
    }
}

#[derive(Debug, Clone, Decode, Encode, PartialEq, Eq)]
#[repr(C)]
pub struct WrappedFq([u64; 4]);

impl From<Fq> for WrappedFq {
    fn from(value: Fq) -> Self {
        let a = value.0 .0[0].into();
        let b = value.0 .0[1].into();
        let c = value.0 .0[2].into();
        let d = value.0 .0[3].into();
        Self([a, b, c, d])
    }
}

impl From<WrappedFq> for Fq {
    fn from(value: WrappedFq) -> Self {
        let a = value.0[0].into();
        let b = value.0[1].into();
        let c = value.0[2].into();
        let d = value.0[3].into();
        Fq::new_unchecked(BigInt([a, b, c, d]))
    }
}

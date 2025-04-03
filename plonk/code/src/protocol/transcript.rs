use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::PrimeField;
use ark_serialize::CanonicalSerialize;
use merlin::Transcript;

use crate::utils::{Point, Scalar};

pub trait TranscriptProtocol<P: SWCurveConfig> {
    fn domain_sep(&mut self);
    fn append_point(&mut self, label: &'static [u8], point: &Point<P>);
    fn append_points(&mut self, label: &'static [u8], comms: &[Point<P>]);
    fn append_scalar(&mut self, label: &'static [u8], scalar: &Scalar<P>);
    fn append_scalars(&mut self, label: &'static [u8], scalars: &[Scalar<P>]);
    fn challenge_scalar(&mut self, label: &'static [u8]) -> Scalar<P>;
}

impl<P: SWCurveConfig> TranscriptProtocol<P> for Transcript {
    fn domain_sep(&mut self) {
        // A proof-specific domain separation label that should
        // uniquely identify the proof statement.
        self.append_message(b"dom-sep", b"Plonk");
    }

    fn append_point(&mut self, label: &'static [u8], point: &Point<P>) {
        let mut buf = Vec::new();
        point.serialize_compressed(&mut buf).unwrap();
        self.append_message(label, &buf);
    }

    fn append_points(&mut self, label: &'static [u8], points: &[Point<P>]) {
        let mut buf = Vec::new();
        for point in points {
            point.serialize_compressed(&mut buf).unwrap();
        }
        self.append_message(label, &buf);
    }

    fn append_scalar(&mut self, label: &'static [u8], scalar: &Scalar<P>) {
        let mut buf = [0; 64];
        scalar.serialize_compressed(buf.as_mut()).unwrap();
        self.append_message(label, &buf);
    }

    fn append_scalars(&mut self, label: &'static [u8], scalars: &[Scalar<P>]) {
        let mut buf = Vec::new();
        for scalar in scalars {
            let mut tmp = [0; 64];
            scalar.serialize_compressed(&mut tmp[..]).unwrap();
            buf.extend_from_slice(&tmp);
        }
        self.append_message(label, &buf);
    }

    fn challenge_scalar(&mut self, label: &'static [u8]) -> Scalar<P> {
        // Reduce a double-width scalar to ensure a uniform distribution
        let mut buf = [0; 64];
        self.challenge_bytes(label, &mut buf);
        Scalar::<P>::from_le_bytes_mod_order(&buf)
    }
}

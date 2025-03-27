use crate::curve::{Point, Scalar};

use ark_ff::PrimeField;
use ark_serialize::CanonicalSerialize;
use halo_accumulation::group::{PallasPoint, PallasScalar};
use merlin::Transcript;

pub trait TranscriptProtocol {
    fn domain_sep(&mut self);
    fn append_point(&mut self, label: &'static [u8], point: &Point);
    fn append_point_new(&mut self, label: &'static [u8], point: &PallasPoint);
    fn append_points(&mut self, label: &'static [u8], comms: &[Point]);
    fn append_points_new(&mut self, label: &'static [u8], comms: &[PallasPoint]);
    #[allow(dead_code)]
    fn append_scalar(&mut self, label: &'static [u8], scalar: &Scalar);
    fn append_scalar_new(&mut self, label: &'static [u8], scalar: &PallasScalar);
    fn append_scalars_new(&mut self, label: &'static [u8], scalars: &[PallasScalar]);
    fn challenge_scalar(&mut self, label: &'static [u8]) -> Scalar;
    fn challenge_scalar_new(&mut self, label: &'static [u8]) -> PallasScalar;
    fn challenge_scalar_augment(&mut self, val: u64, label: &'static [u8]) -> Scalar;
}

impl TranscriptProtocol for Transcript {
    fn domain_sep(&mut self) {
        // A proof-specific domain separation label that should
        // uniquely identify the proof statement.
        self.append_message(b"dom-sep", b"Plonk");
    }

    fn append_point(&mut self, label: &'static [u8], point: &Point) {
        let mut buf = Vec::new();
        let pallas_point: PallasPoint = point.into();
        pallas_point.serialize_compressed(&mut buf).unwrap();
        self.append_message(label, &buf);
    }

    fn append_point_new(&mut self, label: &'static [u8], point: &PallasPoint) {
        let mut buf = Vec::new();
        point.serialize_compressed(&mut buf).unwrap();
        self.append_message(label, &buf);
    }

    fn append_points_new(&mut self, label: &'static [u8], points: &[PallasPoint]) {
        let mut buf = Vec::new();
        for point in points {
            point.serialize_compressed(&mut buf).unwrap();
        }
        self.append_message(label, &buf);
    }

    fn append_points(&mut self, label: &'static [u8], comms: &[Point]) {
        for (i, comm) in comms.iter().enumerate() {
            let mut new_label = label.to_vec();
            new_label.extend_from_slice(i.to_string().as_bytes());
            self.append_point(Box::leak(new_label.into_boxed_slice()), comm);
        }
    }

    fn append_scalar(&mut self, label: &'static [u8], scalar: &Scalar) {
        let mut buf = [0; 64];
        let val: PallasScalar = scalar.into();
        val.serialize_compressed(buf.as_mut()).unwrap();
        self.append_message(label, &buf);
    }

    fn append_scalar_new(&mut self, label: &'static [u8], scalar: &PallasScalar) {
        let mut buf = [0; 64];
        scalar.serialize_compressed(buf.as_mut()).unwrap();
        self.append_message(label, &buf);
    }

    fn append_scalars_new(&mut self, label: &'static [u8], scalars: &[PallasScalar]) {
        let mut buf = Vec::new();
        for scalar in scalars {
            let mut tmp = [0; 64];
            scalar.serialize_compressed(&mut tmp[..]).unwrap();
            buf.extend_from_slice(&tmp);
        }
        self.append_message(label, &buf);
    }

    fn challenge_scalar(&mut self, label: &'static [u8]) -> Scalar {
        // Reduce a double-width scalar to ensure a uniform distribution
        let mut buf = [0; 64];
        self.challenge_bytes(label, &mut buf);
        PallasScalar::from_le_bytes_mod_order(&buf).into()
    }

    fn challenge_scalar_new(&mut self, label: &'static [u8]) -> PallasScalar {
        // Reduce a double-width scalar to ensure a uniform distribution
        let mut buf = [0; 64];
        self.challenge_bytes(label, &mut buf);
        PallasScalar::from_le_bytes_mod_order(&buf)
    }

    fn challenge_scalar_augment(&mut self, val: u64, label: &'static [u8]) -> Scalar {
        let mut t = self.clone();
        t.append_u64(b"aug", val);
        t.challenge_scalar(label)
    }
}

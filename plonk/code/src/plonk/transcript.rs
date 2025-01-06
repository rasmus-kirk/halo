use super::utils::{CommitData, Scalar};
use ark_ff::PrimeField;
use ark_serialize::CanonicalSerialize;
use halo_accumulation::group::{PallasPoint, PallasScalar};
use merlin::Transcript;

pub trait TranscriptProtocol {
    fn domain_sep(&mut self);
    fn append_point(&mut self, label: &'static [u8], point: &PallasPoint);
    fn append_comm(&mut self, label: &'static [u8], comm: &CommitData);
    fn append_scalar(&mut self, label: &'static [u8], scalar: &Scalar);
    fn challenge_scalar(&mut self, label: &'static [u8]) -> Scalar;
    fn challenge_scalar_augment(&mut self, val: u64, label: &'static [u8]) -> Scalar;
}

impl TranscriptProtocol for Transcript {
    fn domain_sep(&mut self) {
        // A proof-specific domain separation label that should
        // uniquely identify the proof statement.
        self.append_message(b"dom-sep", b"Plonk");
    }

    fn append_point(&mut self, label: &'static [u8], point: &PallasPoint) {
        let mut buf = Vec::new();
        point.serialize_compressed(&mut buf).unwrap();
        self.append_message(label, &buf);
    }

    fn append_comm(&mut self, label: &'static [u8], comm: &CommitData) {
        self.append_point(label, &comm.pt);
    }

    fn append_scalar(&mut self, label: &'static [u8], scalar: &Scalar) {
        let mut buf = [0; 64];
        scalar.val.serialize_compressed(buf.as_mut()).unwrap();
        self.append_message(label, &buf);
    }

    fn challenge_scalar(&mut self, label: &'static [u8]) -> Scalar {
        // Reduce a double-width scalar to ensure a uniform distribution
        let mut buf = [0; 64];
        self.challenge_bytes(label, &mut buf);
        PallasScalar::from_le_bytes_mod_order(&buf).into()
    }

    fn challenge_scalar_augment(&mut self, val: u64, label: &'static [u8]) -> Scalar {
        let mut t = self.clone();
        t.append_u64(b"aug", val);
        t.challenge_scalar(label)
    }
}

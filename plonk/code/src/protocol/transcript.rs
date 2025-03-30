use ark_ff::PrimeField;
use ark_serialize::CanonicalSerialize;
use halo_accumulation::group::{PallasPoint, PallasScalar};
use merlin::Transcript;

pub trait TranscriptProtocol {
    fn domain_sep(&mut self);
    fn append_point(&mut self, label: &'static [u8], point: &PallasPoint);
    fn append_points(&mut self, label: &'static [u8], comms: &[PallasPoint]);
    fn append_scalar(&mut self, label: &'static [u8], scalar: &PallasScalar);
    fn append_scalars(&mut self, label: &'static [u8], scalars: &[PallasScalar]);
    fn challenge_scalar(&mut self, label: &'static [u8]) -> PallasScalar;
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

    fn append_points(&mut self, label: &'static [u8], points: &[PallasPoint]) {
        let mut buf = Vec::new();
        for point in points {
            point.serialize_compressed(&mut buf).unwrap();
        }
        self.append_message(label, &buf);
    }

    fn append_scalar(&mut self, label: &'static [u8], scalar: &PallasScalar) {
        let mut buf = [0; 64];
        scalar.serialize_compressed(buf.as_mut()).unwrap();
        self.append_message(label, &buf);
    }

    fn append_scalars(&mut self, label: &'static [u8], scalars: &[PallasScalar]) {
        let mut buf = Vec::new();
        for scalar in scalars {
            let mut tmp = [0; 64];
            scalar.serialize_compressed(&mut tmp[..]).unwrap();
            buf.extend_from_slice(&tmp);
        }
        self.append_message(label, &buf);
    }

    fn challenge_scalar(&mut self, label: &'static [u8]) -> PallasScalar {
        // Reduce a double-width scalar to ensure a uniform distribution
        let mut buf = [0; 64];
        self.challenge_bytes(label, &mut buf);
        PallasScalar::from_le_bytes_mod_order(&buf)
    }
}

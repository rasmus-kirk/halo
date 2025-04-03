use ark_ff::{Fp, FpConfig, PrimeField};
use ark_serialize::CanonicalSerialize;
use halo_accumulation::group::PallasPoint;
use merlin::Transcript;

pub trait TranscriptProtocol<const N: usize, C: FpConfig<N>> {
    fn domain_sep(&mut self);
    fn append_point(&mut self, label: &'static [u8], point: &PallasPoint);
    fn append_points(&mut self, label: &'static [u8], comms: &[PallasPoint]);
    fn append_scalar(&mut self, label: &'static [u8], scalar: &Fp<C, N>);
    fn append_scalars(&mut self, label: &'static [u8], scalars: &[Fp<C, N>]);
    fn challenge_scalar(&mut self, label: &'static [u8]) -> Fp<C, N>;
}

impl<const N: usize, C: FpConfig<N>> TranscriptProtocol<N, C> for Transcript {
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

    fn append_scalar(&mut self, label: &'static [u8], scalar: &Fp<C, N>) {
        let mut buf = [0; 64];
        scalar.serialize_compressed(buf.as_mut()).unwrap();
        self.append_message(label, &buf);
    }

    fn append_scalars(&mut self, label: &'static [u8], scalars: &[Fp<C, N>]) {
        let mut buf = Vec::new();
        for scalar in scalars {
            let mut tmp = [0; 64];
            scalar.serialize_compressed(&mut tmp[..]).unwrap();
            buf.extend_from_slice(&tmp);
        }
        self.append_message(label, &buf);
    }

    fn challenge_scalar(&mut self, label: &'static [u8]) -> Fp<C, N> {
        // Reduce a double-width scalar to ensure a uniform distribution
        let mut buf = [0; 64];
        self.challenge_bytes(label, &mut buf);
        Fp::from_le_bytes_mod_order(&buf)
    }
}

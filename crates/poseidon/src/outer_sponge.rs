use halo_group::{
    ark_ec::{
        short_weierstrass::{Affine, Projective},
        CurveGroup,
    },
    ark_ff::{BigInt, BigInteger, One, PrimeField, Zero},
    PastaConfig,
};

use crate::inner_sponge::PoseidonSponge;

pub struct Sponge<P: PastaConfig> {
    sponge: PoseidonSponge<P>,
}

pub enum Protocols {
    PCDL = 0,
    ASDL = 1,
    PLONK = 2,
    SIGNATURE = 3,
}

impl<P: PastaConfig> Sponge<P> {
    fn add_g_affine(&mut self, g: &Affine<P>) {
        if g.infinity {
            // absorb a fake point (0, 0)
            self.sponge.absorb(&[P::BaseField::zero()]);
            self.sponge.absorb(&[P::BaseField::zero()]);
        } else {
            self.sponge.absorb(&[g.x]);
            self.sponge.absorb(&[g.y]);
        }
    }

    pub fn new(label: Protocols) -> Self {
        let mut inner_sponge = PoseidonSponge::new();
        let field_label = P::basefield_from_bigint(BigInt::<4>::from(label as u8)).unwrap();
        inner_sponge.absorb(&[field_label]);

        Sponge {
            sponge: inner_sponge,
        }
    }

    pub fn absorb_g(&mut self, g: &[Projective<P>]) {
        for g in g.iter() {
            self.add_g_affine(&g.into_affine());
        }
    }

    pub fn absorb_g_affine(&mut self, g: &[Affine<P>]) {
        for g in g.iter() {
            self.add_g_affine(g);
        }
    }

    pub fn absorb_fq(&mut self, x: &[P::BaseField]) {
        for fe in x {
            self.sponge.absorb(&[*fe])
        }
    }

    pub fn absorb_fr(&mut self, x: &[P::ScalarField]) {
        x.iter().for_each(|x| {
            let bits = x.into_bigint().to_bits_le();

            // absorb
            if P::SCALAR_MODULUS < P::BASE_MODULUS {
                let fe = P::basefield_from_bigint(BigInt::<4>::from_bits_le(&bits)).unwrap();
                self.sponge.absorb(&[fe]);
            } else {
                let low_bit = match bits[0] {
                    true => P::BaseField::one(),
                    false => P::BaseField::zero(),
                };
                let high_bits =
                    P::basefield_from_bigint(BigInt::<4>::from_bits_le(&bits[1..bits.len()]))
                        .unwrap();

                self.sponge.absorb(&[high_bits]);
                self.sponge.absorb(&[low_bit]);
            }
        });
    }

    pub fn challenge(&mut self) -> P::ScalarField {
        let bits = P::basefield_into_bigint(self.sponge.squeeze()).to_bits_le();
        if P::SCALAR_MODULUS < P::BASE_MODULUS {
            let high_bits =
                P::scalar_from_bigint(BigInt::<4>::from_bits_le(&bits[1..bits.len()])).unwrap();
            high_bits
        } else {
            P::scalar_from_bigint(BigInt::<4>::from_bits_le(&bits)).unwrap()
        }
    }

    pub fn reset(&mut self) {
        self.sponge.reset()
    }
}

use ark_ec::short_weierstrass::Affine;
use ark_ff::One;
use ark_ff::PrimeField;
use ark_ff::Zero;
use ark_ff::{BigInt, BigInteger};
use halo_group::wrappers::PastaConfig;

use crate::inner_sponge::PoseidonSponge;

pub struct Sponge<P: PastaConfig> {
    sponge: PoseidonSponge<P>,
}

pub enum Protocols {
    PCDL = 0,
    ASDL = 1,
    PLONK = 2,
}

impl<P: PastaConfig> Sponge<P> {
    pub fn new(label: Protocols) -> Self {
        let mut inner_sponge = PoseidonSponge::new();
        let field_label = P::basefield_from_bigint(BigInt::<4>::from(label as u8)).unwrap();
        inner_sponge.absorb(&[field_label]);

        Sponge {
            sponge: inner_sponge,
        }
    }

    pub fn absorb_g(&mut self, g: &[Affine<P>]) {
        for g in g.iter() {
            if g.infinity {
                // absorb a fake point (0, 0)
                self.sponge.absorb(&[P::BaseField::zero()]);
                self.sponge.absorb(&[P::BaseField::zero()]);
            } else {
                self.sponge.absorb(&[g.x]);
                self.sponge.absorb(&[g.y]);
            }
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
            if P::FP_MODULUS < P::FQ_MODULUS {
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

    // fn digest(mut self) -> P::ScalarField {
    //     let x: <P::BaseField as PrimeField>::BigInt = self.squeeze_field().into_bigint();
    //     // Returns zero for values that are too large.
    //     // This means that there is a bias for the value zero (in one of the curve).
    //     // An attacker could try to target that seed, in order to predict the challenges u and v produced by the Fr-Sponge.
    //     // This would allow the attacker to mess with the result of the aggregated evaluation proof.
    //     // Previously the attacker's odds were 1/q, now it's (q-p)/q.
    //     // Since log2(q-p) ~ 86 and log2(q) ~ 254 the odds of a successful attack are negligible.
    //     P::ScalarField::from_bigint(x.into()).unwrap_or_else(P::ScalarField::zero)
    // }

    pub fn challenge(&mut self) -> P::ScalarField {
        let x = P::basefield_into_bigint(self.sponge.squeeze());
        // Returns zero for values that are too large.
        // This means that there is a bias for the value zero (in one of the curve).
        // An attacker could try to target that seed, in order to predict the challenges u and v produced by the Fr-Sponge.
        // This would allow the attacker to mess with the result of the aggregated evaluation proof.
        // Previously the attacker's odds were 1/q, now it's (q-p)/q.
        // Since log2(q-p) ~ 86 and log2(q) ~ 254 the odds of a successful attack are negligible.
        P::scalar_from_bigint(x).unwrap_or_else(P::ScalarField::zero)
    }

    // fn challenge_fq(&mut self) -> P::BaseField {
    //     self.squeeze()
    // }
}

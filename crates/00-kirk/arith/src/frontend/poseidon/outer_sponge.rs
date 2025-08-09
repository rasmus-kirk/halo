use halo_group::PastaConfig;
use halo_poseidon::Protocols;

use crate::frontend::{curve::WireAffine, field::WireScalar, poseidon::inner_sponge::InnerSponge};

pub struct OuterSponge<P: PastaConfig> {
    sponge: InnerSponge<P::OtherCurve>,
}

impl<P: PastaConfig> OuterSponge<P> {
    pub fn new(label: Protocols) -> Self {
        let mut inner_sponge = InnerSponge::new();
        let field_label =
            WireScalar::<P::OtherCurve>::constant(P::OtherCurve::scalar_from_u64(label as u64));
        inner_sponge.absorb(&[field_label]);

        OuterSponge {
            sponge: inner_sponge,
        }
    }

    pub fn absorb_g(&mut self, gs: &[WireAffine<P>]) {
        for g in gs {
            self.sponge.absorb(&[g.x, g.y]);
        }
    }

    pub fn absorb_fq(&mut self, x: &[WireScalar<P::OtherCurve>]) {
        for fe in x {
            self.sponge.absorb(&[*fe])
        }
    }

    pub fn absorb_fp(&mut self, x: &[WireScalar<P>]) {
        x.iter().for_each(|x| {
            if P::SCALAR_MODULUS < P::BASE_MODULUS {
                todo!();
            } else {
                let (h, l) = x.message_pass();
                self.sponge.absorb(&[h]);
                self.sponge.absorb(&[l]);
            }
        });
    }

    pub fn challenge(&mut self) -> WireScalar<P> {
        let x = self.sponge.squeeze();
        if P::SCALAR_MODULUS < P::BASE_MODULUS {
            todo!();
        } else {
            let (h, _) = x.message_pass();
            h
        }
    }

    pub fn reset(&mut self) {
        self.sponge.reset()
    }
}

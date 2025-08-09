use anyhow::Result;
use halo_group::PastaConfig;
use halo_poseidon::Protocols;

use crate::frontend::{
    curve::WireAffine,
    field::WireScalar,
    pcdl::{WireHPoly, WireInstance, WirePublicParams},
    poseidon::outer_sponge::OuterSponge,
};

#[derive(Clone)]
pub struct WireAccumulator<P: PastaConfig> {
    instance: WireInstance<P>,
}

#[derive(Clone)]
pub struct AccumulatedHPolys<P: PastaConfig> {
    pub(crate) hs: Vec<WireHPoly<P>>,
    alpha: Option<WireScalar<P>>,
    alphas: Vec<WireScalar<P>>,
}

impl<P: PastaConfig> AccumulatedHPolys<P> {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            hs: Vec::with_capacity(capacity),
            alphas: Vec::with_capacity(capacity + 1),
            alpha: None,
        }
    }

    pub(crate) fn set_alpha(&mut self, alpha: WireScalar<P>) {
        self.alphas = alpha.geometric_series(self.alphas.capacity());
        self.alpha = Some(alpha)
    }

    // WARNING: This will panic if alphas has not been initialized, but should be fine since this is private
    pub(crate) fn eval(&self, z: WireScalar<P>) -> WireScalar<P> {
        let mut v = WireScalar::<P>::zero();
        for i in 0..self.hs.len() {
            v += self.hs[i].eval(z) * self.alphas[i + 1];
        }
        v
    }

    pub(crate) fn get_scalars(&self) -> Vec<WireScalar<P>> {
        let mut vec: Vec<_> = self.hs.iter().flat_map(|x| x.xis.clone()).collect();
        if let Some(alpha) = self.alpha {
            vec.push(alpha)
        }
        vec
    }
}

fn point_dot<P: PastaConfig>(a: &[WireScalar<P>], p: &[WireAffine<P>]) -> WireAffine<P> {
    assert_eq!(a.len(), p.len());
    assert!(a.len() >= 1);

    let mut result = p[0] * a[0];
    for i in 1..a.len() {
        result += p[i] * a[i];
    }
    result
}

/// D: Degree of the underlying polynomials
/// pi_V: Used for hiding
#[allow(clippy::type_complexity)]
pub fn common_subroutine<P: PastaConfig>(
    pp: WirePublicParams<P>,
    qs: Vec<WireInstance<P>>,
) -> Result<(WireAffine<P>, WireScalar<P>, AccumulatedHPolys<P>)> {
    let m = qs.len();

    let mut transcript = OuterSponge::new(Protocols::ASDL);

    // 1. Parse avk as (rk, ck^(1)_(PC)), and rk as (⟨group⟩ = (G, q, G), S, H, D).
    let mut hs = AccumulatedHPolys::with_capacity(m);
    let mut Us = Vec::with_capacity(m);

    // (2). Parse π_V as (h_0, U_0, ω), where h_0(X) = aX + b ∈ F_q[X], U_0 ∈ G, and ω ∈ F_q.

    // (3). Check that U_0 is a deterministic commitment to h_0: U_0 = PCDL.Commit_ρ0(ck^(1)_PC, h; ω = ⊥).

    // 4. For each i ∈ [m]:
    for q in qs {
        // 4.a Parse q_i as a tuple ((C_i, d_i, z_i, v_i), π_i).
        // 4.b Compute (h_i(X), U_i) := PCDL.SuccinctCheckρ0(rk, C_i, z_i, v_i, π_i) (see Figure 2).
        let (h_i, U_i) = q.succinct_check(pp);
        hs.hs.push(h_i);
        Us.push(U_i);

        // 5. For each i in [n], check that d_i = D. (We accumulate only the degree bound D.)
    }

    // 6. Compute the challenge α := ρ1([h_i, U_i]^n_(i=0)) ∈ F_q.
    transcript.absorb_fp(&hs.get_scalars());
    transcript.absorb_g(&Us);
    let alpha = transcript.challenge();
    hs.set_alpha(alpha);

    // 7. Set the polynomial h(X) := Σ^n_(i=0) α^i · h_i(X) ∈ Fq[X].

    // 8. Compute the accumulated commitment C := Σ^n_(i=0) α^i · U_i.
    let C = point_dot(&hs.alphas, &Us);

    // 9. Compute the challenge z := ρ1(C, h) ∈ F_q.
    let z = transcript.challenge();

    // 10. Randomize C : C_bar := C + ω · S ∈ G.
    let C_bar = C;

    // 11. Output (C_bar, d, z, h(X)).
    Ok((C_bar, z, hs))
}

pub fn verifier<P: PastaConfig>(
    pp: WirePublicParams<P>,
    qs: Vec<WireInstance<P>>,
    acc: WireAccumulator<P>,
) -> Result<()> {
    let WireInstance { C, z, v, pi: _ } = acc.instance;

    // 1. The accumulation verifier V computes (C_bar', d', z', h(X)) := T^ρ(avk, [qi]^n_(i=1), π_V)
    let (C_bar_prime, z_prime, h) = common_subroutine(pp, qs)?;

    // 2. Then checks that C_bar' = C_bar, d' = d, z' = z, and h(z) = v.
    C_bar_prime.assert_eq(C);
    z_prime.assert_eq(z);
    h.eval(z).assert_eq(v);

    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use halo_accumulation::pcdl::Instance;
    use halo_group::{
        PallasConfig, PublicParams, Scalar,
        ark_ec::{AffineRepr, CurveGroup},
        ark_ff::UniformRand,
        ark_std::test_rng,
    };

    use crate::{
        frontend::{
            Call,
            curve::WireAffine,
            field::WireScalar,
            pcdl::{WireEvalProof, WirePublicParams},
        },
        plonk::PlonkProof,
    };

    #[test]
    fn succinct_check() -> Result<()> {
        Ok(())
    }
}

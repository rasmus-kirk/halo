use halo_group::PastaConfig;
use halo_poseidon::Protocols;

use crate::frontend::{curve::WireAffine, field::WireScalar, poseidon::outer_sponge::OuterSponge};

#[derive(Clone)]
pub struct WireHPoly<P: PastaConfig> {
    pub(crate) xis: Vec<WireScalar<P>>,
}
impl<P: PastaConfig> WireHPoly<P> {
    pub(crate) fn new(xis: Vec<WireScalar<P>>) -> Self {
        Self { xis }
    }

    pub(crate) fn eval(&self, z: WireScalar<P>) -> WireScalar<P> {
        let lg_n = self.xis.len() - 1;
        let one = WireScalar::<P>::one();

        let mut v = one + self.xis[lg_n] * z;
        let mut z_i = z;

        for i in 1..lg_n {
            z_i = z_i.square();
            v *= one + self.xis[lg_n - i] * z_i;
        }
        v
    }
}

#[derive(Clone)]
pub struct WireEvalProof<P: PastaConfig> {
    pub Ls: Vec<WireAffine<P>>,
    pub Rs: Vec<WireAffine<P>>,
    pub U: WireAffine<P>,
    pub c: WireScalar<P>,
}

#[derive(Clone, Copy)]
pub struct WirePublicParams<P: PastaConfig> {
    pub H: WireAffine<P>,
    pub d: usize,
    pub lg_n: usize,
}

#[derive(Clone)]
pub struct WireInstance<P: PastaConfig> {
    pub C: WireAffine<P>,
    pub z: WireScalar<P>,
    pub v: WireScalar<P>,
    pub pi: WireEvalProof<P>,
}
impl<P: PastaConfig> WireInstance<P> {
    pub fn succinct_check(self, pp: WirePublicParams<P>) -> (WireHPoly<P>, WireAffine<P>) {
        // let n = d + 1;
        // let lg_n = n.ilog2() as usize;
        // assert!(n.is_power_of_two(), "n ({n}) is not a power of two");
        // ensure!(d <= pp.D, "d was larger than D!");

        let WireInstance { C, z, v, pi } = self;

        let mut transcript = OuterSponge::new(Protocols::PCDL);

        // 1. Parse rk as (⟨group⟩, S, H, d'), and π as (L, R, U, c, C_bar, ω').
        let WireEvalProof { Ls, Rs, U, c } = pi;

        // 2. Check that d = d'. Irrelevant, we just removed d'
        //ensure!(d == d_prime, "d ≠ d'");

        // 4. Compute the non-hiding commitment C' := C + α · C_bar − ω'· S ∈ G.
        let C_prime = C;

        // 5. Compute the 0-th challenge ξ_0 := ρ_0(C', z, v), and set H' := ξ_0 · H ∈ G.
        transcript.absorb_g(&[C_prime]);
        transcript.absorb_fp(&[z, v]);
        let xi_0 = transcript.challenge();
        let mut xis = Vec::new();
        xis.push(xi_0);

        let H_prime = pp.H * xi_0;

        // 6. Compute the group element C_0 := C' + v · H' ∈ G.
        let mut C_i = C_prime + H_prime * v;

        // 7. For each i ∈ [log_n]:
        for i in 0..pp.lg_n {
            // 7.a Generate the (i+1)-th challenge: ξ_(i+1) := ρ_0(ξ_i, L_i, R_i) ∈ F_q.
            transcript.absorb_fp(&[xis[i]]);
            transcript.absorb_g(&[Ls[i], Rs[i]]);
            let xi_next = transcript.challenge();
            xis.push(xi_next);

            // 7.b Compute the (i+1)-th commitment: C_(i+1) := C_i + ξ^(−1)_(i+1) · L_i + ξ_(i+1) · R_i ∈ G.
            C_i += Ls[i] * xi_next.inv() + Rs[i] * xi_next;
        }

        // 8. Define the univariate polynomial h(X) := π^(lg(n))_(i=0) (1 + ξ_(lg(n)−i) · X^(2^i)) ∈ F_q[X].
        let h = WireHPoly::new(xis);

        // 9. Compute the evaluation v' := c · h(z) ∈ F_q.
        let v_prime = c * h.eval(z);

        // 10. Check that C_(log_n) = CM.Commit_Σ(c || v'), where Σ = (U || H').

        C_i.assert_eq(U * c + H_prime * v_prime);

        // 11. Output (h, U).
        (h, U)
    }
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
            pcdl::{WireEvalProof, WireInstance, WirePublicParams},
        },
        plonk::PlonkProof,
    };

    #[test]
    fn succinct_check() -> Result<()> {
        let rng = &mut test_rng();
        let lg_n = 5usize;
        let n = 2usize.pow(lg_n as u32);
        let d = n - 1;

        let H_v = PublicParams::get_pp().H.into_affine();
        let instance = Instance::<PallasConfig>::rand_without_hiding(rng, n);
        let (h_expected, U_expected) = instance.succinct_check()?;
        let (C_v, _, z_v, v_v, pi_v) = instance.into_tuple();
        let (Ls_v, Rs_v, U_v, c_v, _, _) = pi_v.into_tuple();
        let r_v = Scalar::<PallasConfig>::rand(rng);

        let Ls: Vec<WireAffine<PallasConfig>> =
            Ls_v.iter().map(|_| WireAffine::witness()).collect();
        let Rs: Vec<WireAffine<PallasConfig>> =
            Rs_v.iter().map(|_| WireAffine::witness()).collect();
        let U = WireAffine::<PallasConfig>::witness();
        let c = WireScalar::<PallasConfig>::witness();
        let C = WireAffine::<PallasConfig>::witness();
        let z = WireScalar::<PallasConfig>::witness();
        let v = WireScalar::<PallasConfig>::witness();
        let r = WireScalar::constant(r_v);

        let pp = WirePublicParams {
            lg_n,
            H: WireAffine::<PallasConfig>::constant(H_v),
            d,
        };
        let pi = WireEvalProof {
            Ls: Ls.clone(),
            Rs: Rs.clone(),
            U,
            c,
        };
        let instance = WireInstance { C, z, v, pi };

        let (h, U) = instance.succinct_check(pp);
        U.output();
        let h_eval = h.eval(r);
        h_eval.output();

        let mut call = Call::new();

        for i in 0..Ls.len() {
            call.witness_affine(Ls[i], Ls_v[i].into_affine())?;
            call.witness_affine(Rs[i], Rs_v[i].into_affine())?;
        }
        call.witness_affine(U, U_v.into_affine())?;
        call.witness_affine(C, C_v.into_affine())?;
        call.witness(z, z_v)?;
        call.witness(v, v_v)?;
        call.witness(c, c_v)?;

        let (fp_trace, fq_trace) = call.trace()?;
        let Ux = fq_trace.outputs[0];
        let Uy = fq_trace.outputs[1];
        let h_eval_v = fp_trace.outputs[0];

        println!("{fq_trace:?}");

        assert_eq!(Ux, U_expected.into_affine().x().unwrap());
        assert_eq!(Uy, U_expected.into_affine().y().unwrap());
        assert_eq!(h_eval_v, h_expected.eval(&r_v));

        PlonkProof::naive_prover(rng, fp_trace.clone()).verify(fp_trace)?;
        PlonkProof::naive_prover(rng, fq_trace.clone()).verify(fq_trace)?;

        Ok(())
    }
}

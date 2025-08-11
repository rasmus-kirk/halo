use anyhow::Result;
use halo_accumulation::pcdl::{EvalProof, Instance};
use halo_group::{PastaConfig, PublicParams, ark_ec::CurveGroup};
use halo_poseidon::Protocols;

use crate::frontend::{
    Call,
    poseidon::outer_sponge::OuterSponge,
    primitives::{WireAffine, WireScalar},
};

pub trait CallInstance {
    fn witness_instance<P: PastaConfig>(
        &mut self,
        wire_instance: &WireInstance<P>,
        instance: &Instance<P>,
    ) -> Result<()>;
    fn witness_eval_proof<P: PastaConfig>(
        &mut self,
        wire_instance: &WireEvalProof<P>,
        instance: &EvalProof<P>,
    ) -> Result<()>;
}
impl CallInstance for Call {
    fn witness_instance<P: PastaConfig>(
        &mut self,
        wire_instance: &WireInstance<P>,
        instance: &Instance<P>,
    ) -> Result<()> {
        self.witness_eval_proof(&wire_instance.pi, &instance.pi)?;
        self.witness_affine(wire_instance.C, instance.C.into_affine())?;
        self.witness(wire_instance.z, instance.z)?;
        self.witness(wire_instance.v, instance.v)?;

        Ok(())
    }
    fn witness_eval_proof<P: PastaConfig>(
        &mut self,
        wire_eval_proof: &WireEvalProof<P>,
        eval_proof: &EvalProof<P>,
    ) -> Result<()> {
        assert_eq!(eval_proof.Ls.len(), eval_proof.Rs.len());
        assert_eq!(eval_proof.Ls.len(), wire_eval_proof.Ls.len());
        assert_eq!(eval_proof.Ls.len(), wire_eval_proof.Rs.len());
        for i in 0..eval_proof.Ls.len() {
            self.witness_affine(wire_eval_proof.Ls[i], eval_proof.Ls[i].into_affine())?;
            self.witness_affine(wire_eval_proof.Rs[i], eval_proof.Rs[i].into_affine())?;
        }
        self.witness_affine(wire_eval_proof.U, eval_proof.U.into_affine())?;
        self.witness(wire_eval_proof.c, eval_proof.c)?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct WireHPoly<P: PastaConfig> {
    pub xis: Vec<WireScalar<P>>,
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
impl<P: PastaConfig> WireEvalProof<P> {
    pub fn witness(n: usize) -> Self {
        let lg_n = n.ilog2() as usize;
        let mut Ls = Vec::with_capacity(lg_n);
        let mut Rs = Vec::with_capacity(lg_n);
        for _ in 0..lg_n {
            Ls.push(WireAffine::witness());
            Rs.push(WireAffine::witness());
        }
        let U = WireAffine::<P>::witness();
        let c = WireScalar::<P>::witness();
        WireEvalProof { Ls, Rs, U, c }
    }
}

#[derive(Clone, Copy)]
pub struct WirePublicParams<P: PastaConfig> {
    pub H: WireAffine<P>,
    pub d: usize,
    pub lg_n: usize,
}
impl<P: PastaConfig> WirePublicParams<P> {
    pub fn new(n: usize) -> Self {
        assert!(n.is_power_of_two());
        let H = PublicParams::get_pp().H.into_affine();
        let d = n - 1;
        let lg_n = n.ilog2() as usize;
        WirePublicParams {
            H: WireAffine::constant(H),
            d,
            lg_n,
        }
    }
}
#[derive(Clone)]
pub struct WireInstance<P: PastaConfig> {
    pub C: WireAffine<P>,
    pub z: WireScalar<P>,
    pub v: WireScalar<P>,
    pub pi: WireEvalProof<P>,
}
impl<P: PastaConfig> WireInstance<P> {
    pub fn new(C: WireAffine<P>, z: WireScalar<P>, v: WireScalar<P>, pi: WireEvalProof<P>) -> Self {
        WireInstance { C, z, v, pi }
    }

    pub fn witness(n: usize) -> Self {
        let C = WireAffine::<P>::witness();
        let z = WireScalar::<P>::witness();
        let v = WireScalar::<P>::witness();
        let pi = WireEvalProof::witness(n);
        WireInstance { C, z, v, pi }
    }

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
        PallasConfig, Scalar,
        ark_ec::{AffineRepr, CurveGroup},
        ark_ff::UniformRand,
        ark_std::test_rng,
    };

    use crate::{
        frontend::{
            Call,
            pcdl::{CallInstance, WireInstance, WirePublicParams},
            primitives::WireScalar,
        },
        plonk::PlonkProof,
    };

    #[test]
    fn succinct_check() -> Result<()> {
        let rng = &mut test_rng();
        let lg_n = 5usize;
        let n = 2usize.pow(lg_n as u32);

        let instance = Instance::<PallasConfig>::rand_without_hiding(rng, n);
        let (h_expected, U_expected) = instance.succinct_check()?;
        let r_v = Scalar::<PallasConfig>::rand(rng);

        let r = WireScalar::witness();
        let pp = WirePublicParams::new(n);
        let wire_instance = WireInstance::witness(n);

        let (h, U) = wire_instance.clone().succinct_check(pp);
        U.output();
        let h_eval = h.eval(r);
        h_eval.output();

        let mut call = Call::new();

        call.witness_instance(&wire_instance, &instance)?;
        call.witness(r, r_v)?;

        let (fp_trace, fq_trace) = call.trace(None)?;
        let Ux = fq_trace.outputs[0];
        let Uy = fq_trace.outputs[1];
        let h_eval_v = fp_trace.outputs[0];

        assert_eq!(Ux, U_expected.into_affine().x().unwrap());
        assert_eq!(Uy, U_expected.into_affine().y().unwrap());
        assert_eq!(h_eval_v, h_expected.eval(&r_v));

        let (plonk_public_input, plonk_witness) = fp_trace.consume();
        PlonkProof::naive_prover(rng, plonk_witness).verify(plonk_public_input)?;
        let (plonk_public_input, plonk_witness) = fq_trace.consume();
        PlonkProof::naive_prover(rng, plonk_witness).verify(plonk_public_input)?;

        Ok(())
    }
}

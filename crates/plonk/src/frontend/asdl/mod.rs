use anyhow::Result;
use halo_accumulation::acc::Accumulator;
use halo_group::PastaConfig;
use halo_poseidon::Protocols;

use crate::frontend::{
    Call,
    pcdl::{CallInstance, WireHPoly, WireInstance, WirePublicParams},
    poseidon::outer_sponge::OuterSponge,
    primitives::{WireAffine, WireBool, WireScalar},
};

fn point_dot<P: PastaConfig>(a: &[WireScalar<P>], p: &[WireAffine<P>]) -> WireAffine<P> {
    assert_eq!(a.len(), p.len());
    assert!(a.len() >= 1);

    let mut result = p[0] * a[0];
    for i in 1..a.len() {
        result += p[i] * a[i];
    }
    result
}

pub trait CallAccumulator {
    fn public_input_accumulator<P: PastaConfig>(
        &mut self,
        wire_accumulator: &WireAccumulator<P>,
        accumulator: &Accumulator<P>,
    ) -> Result<()>;
    fn witness_accumulator<P: PastaConfig>(
        &mut self,
        wire_accumulator: &WireAccumulator<P>,
        accumulator: &Accumulator<P>,
    ) -> Result<()>;
}
impl CallAccumulator for Call {
    fn witness_accumulator<P: PastaConfig>(
        &mut self,
        wire_accumulator: &WireAccumulator<P>,
        accumulator: &Accumulator<P>,
    ) -> Result<()> {
        let instance = &accumulator.q;
        let wire_instance = &wire_accumulator.instance;
        self.witness_instance(wire_instance, instance)?;
        Ok(())
    }
    fn public_input_accumulator<P: PastaConfig>(
        &mut self,
        wire_accumulator: &WireAccumulator<P>,
        accumulator: &Accumulator<P>,
    ) -> Result<()> {
        let instance = &accumulator.q;
        let wire_instance = &wire_accumulator.instance;
        self.public_input_instance(wire_instance, instance)?;
        Ok(())
    }
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
            alphas: Vec::with_capacity(capacity),
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
            v += self.hs[i].eval(z) * self.alphas[i];
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

#[derive(Clone)]
pub struct WireAccumulator<P: PastaConfig> {
    pub instance: WireInstance<P>,
}
impl<P: PastaConfig> WireAccumulator<P> {
    pub fn witness(n: usize) -> Self {
        let instance = WireInstance::witness(n);
        Self { instance }
    }

    pub fn public_input(n: usize) -> Self {
        let instance = WireInstance::public_input(n);
        Self { instance }
    }

    pub fn common_subroutine(
        pp: WirePublicParams<P>,
        qs: Vec<WireInstance<P>>,
    ) -> (
        WireBool<P::OtherCurve>,
        WireAffine<P>,
        WireScalar<P>,
        AccumulatedHPolys<P>,
    ) {
        let m = qs.len();

        let mut transcript = OuterSponge::new(Protocols::ASDL);

        // 1. Parse avk as (rk, ck^(1)_(PC)), and rk as (⟨group⟩ = (G, q, G), S, H, D).
        let mut hs = AccumulatedHPolys::with_capacity(m);
        let mut Us = Vec::with_capacity(m);

        // (2). Parse π_V as (h_0, U_0, ω), where h_0(X) = aX + b ∈ F_q[X], U_0 ∈ G, and ω ∈ F_q.

        // (3). Check that U_0 is a deterministic commitment to h_0: U_0 = PCDL.Commit_ρ0(ck^(1)_PC, h; ω = ⊥).

        // 4. For each i ∈ [m]:
        let mut res = WireBool::t();
        for q in qs {
            // 4.a Parse q_i as a tuple ((C_i, d_i, z_i, v_i), π_i).
            // 4.b Compute (h_i(X), U_i) := PCDL.SuccinctCheckρ0(rk, C_i, z_i, v_i, π_i) (see Figure 2).
            let (b, h_i, U_i) = q.succinct_check(pp);
            hs.hs.push(h_i);
            Us.push(U_i);
            res = res & b;

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
        (res, C_bar, z, hs)
    }

    pub fn verify(self, pp: WirePublicParams<P>, qs: Vec<WireInstance<P>>) -> WireBool<P> {
        let acc = self;
        let WireInstance { C, z, v, pi: _ } = acc.instance;

        // 1. The accumulation verifier V computes (C_bar', d', z', h(X)) := T^ρ(avk, [qi]^n_(i=1), π_V)
        let (is_subroutine_ok, C_bar_prime, z_prime, h) = Self::common_subroutine(pp, qs);

        // 2. Then checks that C_bar' = C_bar, d' = d, z' = z, and h(z) = v.
        let is_C_bar_eq = C_bar_prime.equals(C);
        let is_z_prime_eq = z_prime.equals(z);
        let is_h_eval_eq = h.eval(z).equals(v);
        (is_subroutine_ok & is_C_bar_eq).message_pass() & is_z_prime_eq & is_h_eval_eq
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use halo_accumulation::{
        acc::{self, Accumulator},
        pcdl::Instance,
    };
    use halo_group::{
        Fp, PallasConfig,
        ark_std::{One, test_rng},
    };

    use crate::{
        frontend::{
            Call,
            asdl::{CallAccumulator, WireAccumulator},
            pcdl::{CallInstance, WireInstance, WirePublicParams},
        },
        plonk::PlonkProof,
    };

    fn accumulate_random_instances(
        n: usize,
        k: usize,
    ) -> Result<(Vec<Instance<PallasConfig>>, Accumulator<PallasConfig>)> {
        let rng = &mut test_rng();

        let instances = vec![Instance::rand_without_hiding(rng, n); k];

        let accumulator = acc::prover(rng, &instances)?;
        acc::verifier(&instances, accumulator.clone())?;

        Ok((instances, accumulator))
    }

    #[test]
    fn acc_verify() -> Result<()> {
        let rng = &mut test_rng();
        let n = 2_usize.pow(4);
        let k = 2;

        let (instances, accumulator) = accumulate_random_instances(n, k)?;

        let pp = WirePublicParams::<PallasConfig>::new(n);
        let mut wire_instances = Vec::new();
        for _ in 0..instances.len() {
            wire_instances.push(WireInstance::witness(n));
        }
        let wire_accumulator = WireAccumulator::witness(n);
        wire_accumulator
            .clone()
            .verify(pp, wire_instances.clone())
            .output();

        let mut call = Call::new();

        call.witness_accumulator(&wire_accumulator, &accumulator)?;
        for (wire_instance, instance) in wire_instances.iter().zip(instances) {
            call.witness_instance(wire_instance, &instance)?
        }

        let (fp_trace, fq_trace) = call.trace()?;

        assert_eq!(fp_trace.outputs[0], Fp::one());
        let (circuit, x, w) = fp_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;
        let (circuit, x, w) = fq_trace.consume();
        PlonkProof::naive_prover(rng, circuit, &x, w).verify(circuit, &x)?;

        Ok(())
    }
}

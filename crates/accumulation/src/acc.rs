#![allow(non_snake_case)]

//! Accumulation scheme based on the Discrete Log assumption, using bulletproofs-style IPP

use anyhow::{ensure, Context, Result};
use ark_pallas::PallasConfig;
use ark_poly::{DenseUVPolynomial, Polynomial};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::{UniformRand, Zero};
use rand::Rng;

use halo_group::{construct_powers, point_dot, PastaConfig, Point, Poly, PublicParams, Scalar};
use halo_poseidon::{Protocols, Sponge};

use crate::pcdl::{self, Instance};

// -------------------- Accumulation Data Structures --------------------

/// acc in the paper
#[derive(Debug, Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct Accumulator<P: PastaConfig> {
    pub(crate) q: Instance<P>,
    pub pi_V: AccumulatorHiding<P>,
}

impl<P: PastaConfig> Accumulator<P> {
    pub fn new<R: Rng>(rng: &mut R, qs: &[Instance<P>]) -> Result<Self> {
        prover(rng, qs)
    }

    pub fn verifier(self, qs: &[Instance<P>]) -> Result<()> {
        verifier(qs, self)
    }

    pub fn decider(self) -> Result<()> {
        decider(self)
    }
}

/// pi_V in the paper, used for hiding only
#[derive(Debug, Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct AccumulatorHiding<P: PastaConfig> {
    pub(crate) h: Poly<P>,
    pub(crate) U: Point<P>,
    pub(crate) w: Scalar<P>,
}

#[derive(Clone, CanonicalSerialize)]
pub struct AccumulatedHPolys<P: PastaConfig> {
    h_0: Option<Poly<P>>,
    pub(crate) hs: Vec<pcdl::HPoly<P>>,
    alpha: Option<Scalar<P>>,
    alphas: Vec<Scalar<P>>,
}

impl<P: PastaConfig> AccumulatedHPolys<P> {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            h_0: None,
            hs: Vec::with_capacity(capacity),
            alphas: Vec::with_capacity(capacity + 1),
            alpha: None,
        }
    }

    pub(crate) fn set_alpha(&mut self, alpha: Scalar<P>) {
        self.alphas = construct_powers::<P>(&alpha, self.alphas.capacity());
        self.alpha = Some(alpha)
    }

    // WARNING: This will panic if alphas has not been initialized, but should be fine since this is private
    pub(crate) fn get_poly(&self) -> Poly<P> {
        let mut h = Poly::<P>::zero();
        if let Some(h_0) = &self.h_0 {
            h += h_0;
        }
        for i in 0..self.hs.len() {
            h += &(self.hs[i].get_poly() * self.alphas[i + 1]);
        }
        h
    }

    // WARNING: This will panic if alphas has not been initialized, but should be fine since this is private
    pub(crate) fn eval(&self, z: &Scalar<P>) -> Scalar<P> {
        let mut v = Scalar::<P>::zero();
        if let Some(h_0) = &self.h_0 {
            v += h_0.evaluate(z);
        }
        for i in 0..self.hs.len() {
            v += self.hs[i].eval(z) * self.alphas[i + 1];
        }
        v
    }

    pub(crate) fn get_scalars(&self) -> Vec<Scalar<P>> {
        let mut vec: Vec<_> = self.hs.iter().flat_map(|x| x.xis.clone()).collect();
        if let Some(alpha) = self.alpha {
            vec.push(alpha)
        }
        vec
    }
}

impl<P: PastaConfig> From<Accumulator<P>> for Instance<P> {
    fn from(acc: Accumulator<P>) -> Instance<P> {
        acc.q
    }
}

// -------------------- Accumulation Functions --------------------

/// Setup
pub fn setup(n: usize) -> Result<()> {
    PublicParams::<PallasConfig>::set_pp(n)
}

/// D: Degree of the underlying polynomials
/// pi_V: Used for hiding
#[allow(clippy::type_complexity)]
pub fn common_subroutine<P: PastaConfig>(
    qs: &[Instance<P>],
    pi_V: &AccumulatorHiding<P>,
) -> Result<(Point<P>, usize, Scalar<P>, AccumulatedHPolys<P>)> {
    let m = qs.len();
    let d = qs.first().context("No instances given")?.d;
    let pp = PublicParams::get_pp();

    let mut transcript = Sponge::new(Protocols::ASDL);

    // 1. Parse avk as (rk, ck^(1)_(PC)), and rk as (⟨group⟩ = (G, q, G), S, H, D).
    let mut hs = AccumulatedHPolys::with_capacity(m);
    let mut Us = Vec::with_capacity(m);

    // (2). Parse π_V as (h_0, U_0, ω), where h_0(X) = aX + b ∈ F_q[X], U_0 ∈ G, and ω ∈ F_q.
    let AccumulatorHiding { h: h_0, U: U_0, w } = pi_V;
    hs.h_0 = Some(h_0.clone());
    Us.push(*U_0);

    // (3). Check that U_0 is a deterministic commitment to h_0: U_0 = PCDL.Commit_ρ0(ck^(1)_PC, h; ω = ⊥).
    ensure!(
        *U_0 == pcdl::commit(h_0, 1, None),
        "U_0 ≠ PCDL.Commit_ρ0(ck^(1)_PC, h_0; ω = ⊥)"
    );

    // 4. For each i ∈ [m]:
    for q in qs {
        // 4.a Parse q_i as a tuple ((C_i, d_i, z_i, v_i), π_i).
        // 4.b Compute (h_i(X), U_i) := PCDL.SuccinctCheckρ0(rk, C_i, z_i, v_i, π_i) (see Figure 2).
        let (h_i, U_i) = q.succinct_check()?;
        hs.hs.push(h_i);
        Us.push(U_i);

        // 5. For each i in [n], check that d_i = D. (We accumulate only the degree bound D.)
        ensure!(q.d == d, "d_i ≠ d");
    }

    // 6. Compute the challenge α := ρ1([h_i, U_i]^n_(i=0)) ∈ F_q.
    transcript.absorb_fr(&hs.get_scalars());
    transcript.absorb_g(&Us);
    let alpha = transcript.challenge();
    hs.set_alpha(alpha);

    // 7. Set the polynomial h(X) := Σ^n_(i=0) α^i · h_i(X) ∈ Fq[X].

    // 8. Compute the accumulated commitment C := Σ^n_(i=0) α^i · U_i.
    let C = point_dot(&hs.alphas, &Us);

    // 9. Compute the challenge z := ρ1(C, h) ∈ F_q.
    let z = transcript.challenge();

    // 10. Randomize C : C_bar := C + ω · S ∈ G.
    let C_bar = C + pp.S * w;

    // 11. Output (C_bar, d, z, h(X)).
    Ok((C_bar, d, z, hs))
}

pub fn prover<R: Rng, P: PastaConfig>(rng: &mut R, qs: &[Instance<P>]) -> Result<Accumulator<P>> {
    // 1. Sample a random linear polynomial h_0 ∈ F_q[X],
    let h_0 = DenseUVPolynomial::rand(1, rng);

    // 2. Then compute a deterministic commitment to h_0: U_0 := PCDL.Commit_ρ0(ck_PC, h_0, d; ω = ⊥).
    let U_0 = pcdl::commit(&h_0, 1, None);

    // 3. Sample commitment randomness ω ∈ Fq, and set π_V := (h_0, U_0, ω).
    let w = Scalar::<P>::rand(rng);
    let pi_V = AccumulatorHiding { h: h_0, U: U_0, w };

    // 4. Then, compute the tuple (C_bar, d, z, h(X)) := T^ρ(avk, [qi]^n_(i=1), π_V).
    let (C_bar, d, z, h) = common_subroutine(qs, &pi_V)?;

    // 5. Compute the evaluation v := h(z)
    let v = h.eval(&z);

    // 6. Generate the hiding evaluation proof π := PCDL.Open_ρ0(ck_PC, h(X), C_bar, d, z; ω).
    let pi = pcdl::open(rng, h.get_poly(), C_bar, d, &z, Some(&w));

    // 7. Finally, output the accumulator acc = ((C_bar, d, z, v), π) and the accumulation proof π_V.
    let q = Instance::new(C_bar, d, z, v, pi);
    let acc = Accumulator { q, pi_V };
    Ok(acc)
}

pub fn verifier<P: PastaConfig>(qs: &[Instance<P>], acc: Accumulator<P>) -> Result<()> {
    let Instance { C, d, z, v, pi: _ } = acc.q;
    let pi_V = acc.pi_V;

    // 1. The accumulation verifier V computes (C_bar', d', z', h(X)) := T^ρ(avk, [qi]^n_(i=1), π_V)
    let (C_bar_prime, d_prime, z_prime, h) = common_subroutine(qs, &pi_V)?;

    // 2. Then checks that C_bar' = C_bar, d' = d, z' = z, and h(z) = v.
    ensure!(C_bar_prime == C, "C_bar' ≠ C_bar");
    ensure!(z_prime == z, "z' = z");
    ensure!(d_prime == d, "d' = d");
    ensure!(h.eval(&z) == v, "h(z) = v");

    Ok(())
}

pub fn decider<P: PastaConfig>(acc: Accumulator<P>) -> Result<()> {
    acc.q.check()
}

// -------------------- Tests --------------------

#[cfg(test)]
mod tests {
    use rand::distributions::Uniform;

    use super::*;

    fn accumulate_random_instance<R: Rng>(
        rng: &mut R,
        n: usize,
        acc: Option<Accumulator<PallasConfig>>,
    ) -> Result<Accumulator<PallasConfig>> {
        let q = Instance::rand(rng, n);
        let qs = if acc.is_some() {
            vec![acc.unwrap().into(), q]
        } else {
            vec![q]
        };

        let acc = prover(rng, &qs)?;
        verifier(&qs, acc.clone())?;

        Ok(acc)
    }

    #[test]
    fn test_acc_scheme() -> Result<()> {
        let mut rng = rand::thread_rng();
        let n_range = Uniform::new(2, 8);
        let n = (2 as usize).pow(rng.sample(&n_range));

        let m = rng.sample(&n_range);
        let mut acc = None;
        for _ in 0..m {
            acc = Some(accumulate_random_instance(&mut rng, n, acc)?);
        }

        decider(acc.unwrap())?;

        Ok(())
    }
}

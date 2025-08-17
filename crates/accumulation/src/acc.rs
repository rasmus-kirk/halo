#![allow(non_snake_case)]

//! Accumulation scheme based on the Discrete Log assumption, using bulletproofs-style IPP

use anyhow::{ensure, Context, Result};
use ark_ec::short_weierstrass::Affine;
use ark_pallas::PallasConfig;
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::{One, UniformRand, Zero};
use educe::Educe;
use rand::{thread_rng, Rng};

use halo_group::{construct_powers, point_dot, PastaConfig, Point, Poly, PublicParams, Scalar};
use halo_poseidon::{Protocols, Sponge};

use crate::pcdl::{self, Instance};

// -------------------- Accumulation Data Structures --------------------

/// acc in the paper
#[derive(Educe, CanonicalSerialize, CanonicalDeserialize)]
#[educe(Debug, Clone, PartialEq, Eq)]
pub struct Accumulator<P: PastaConfig> {
    pub q: Instance<P>,
}

impl<P: PastaConfig> Accumulator<P> {
    pub fn new<R: Rng>(rng: &mut R, qs: &[Instance<P>]) -> Result<Self> {
        prover(rng, qs)
    }

    pub const fn from_instance(q: Instance<P>) -> Self {
        Accumulator { q }
    }

    pub fn zero(n: usize, k: usize) -> Self {
        let rng = &mut thread_rng();
        let qs = vec![Instance::<P>::zero(n); k];
        prover(rng, &qs).unwrap()
    }

    pub fn zero_invalid(n: usize) -> Self {
        Self {
            q: Instance::zero_invalid(n),
        }
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
    pub(crate) U: Point<P>,
}

#[derive(Clone, CanonicalSerialize)]
pub struct AccumulatedHPolys<P: PastaConfig> {
    pub(crate) hs: Vec<pcdl::HPoly<P>>,
    alpha: Option<Scalar<P>>,
    alphas: Vec<Scalar<P>>,
}

impl<P: PastaConfig> AccumulatedHPolys<P> {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            hs: Vec::with_capacity(capacity),
            alphas: Vec::with_capacity(capacity),
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
        for i in 0..self.hs.len() {
            h += &(self.hs[i].get_poly() * self.alphas[i]);
        }
        h
    }

    // WARNING: This will panic if alphas has not been initialized, but should be fine since this is private
    pub(crate) fn eval(&self, z: &Scalar<P>) -> Scalar<P> {
        let mut v = Scalar::<P>::zero();
        for i in 0..self.hs.len() {
            v += self.hs[i].eval(z) * self.alphas[i];
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
) -> Result<(Point<P>, usize, Scalar<P>, AccumulatedHPolys<P>)> {
    let m = qs.len();
    let d = qs.first().context("No instances given")?.d;
    // let pp = PublicParams::get_pp();

    let mut transcript = Sponge::new(Protocols::ASDL);

    // 1. Parse avk as (rk, ck^(1)_(PC)), and rk as (⟨group⟩ = (G, q, G), S, H, D).
    let mut hs = AccumulatedHPolys::with_capacity(m);
    let mut Us = Vec::with_capacity(m);

    // (2). Parse π_V as (h_0, U_0, ω), where h_0(X) = aX + b ∈ F_q[X], U_0 ∈ G, and ω ∈ F_q.

    // (3). Check that U_0 is a deterministic commitment to h_0: U_0 = PCDL.Commit_ρ0(ck^(1)_PC, h; ω = ⊥).

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
    let C_bar = C;

    // 11. Output (C_bar, d, z, h(X)).
    Ok((C_bar, d, z, hs))
}

pub fn prover<R: Rng, P: PastaConfig>(rng: &mut R, qs: &[Instance<P>]) -> Result<Accumulator<P>> {
    // 1. Sample a random linear polynomial h_0 ∈ F_q[X],

    // 2. Then compute a deterministic commitment to h_0: U_0 := PCDL.Commit_ρ0(ck_PC, h_0, d; ω = ⊥).

    // 3. Sample commitment randomness ω ∈ Fq, and set π_V := (h_0, U_0, ω).

    // 4. Then, compute the tuple (C_bar, d, z, h(X)) := T^ρ(avk, [qi]^n_(i=1), π_V).
    let (C_bar, d, z, h) = common_subroutine(qs)?;

    // 5. Compute the evaluation v := h(z)
    let v = h.eval(&z);

    // 6. Generate the hiding evaluation proof π := PCDL.Open_ρ0(ck_PC, h(X), C_bar, d, z; ω).
    let pi = pcdl::open(rng, h.get_poly(), C_bar, d, &z, None);

    // 7. Finally, output the accumulator acc = ((C_bar, d, z, v), π) and the accumulation proof π_V.
    let q = Instance::new(C_bar, d, z, v, pi);
    let acc = Accumulator { q };
    Ok(acc)
}

pub fn verifier<P: PastaConfig>(qs: &[Instance<P>], acc: Accumulator<P>) -> Result<()> {
    let Instance { C, d, z, v, pi: _ } = acc.q;

    // 1. The accumulation verifier V computes (C_bar', d', z', h(X)) := T^ρ(avk, [qi]^n_(i=1), π_V)
    let (C_bar_prime, d_prime, z_prime, h) = common_subroutine(qs)?;

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
        let n_range = Uniform::new(2, 4);
        let n = 2_usize.pow(rng.sample(n_range));

        let m = rng.sample(n_range);
        let mut acc = None;
        for _ in 0..m {
            acc = Some(accumulate_random_instance(&mut rng, n, acc)?);
        }

        decider(acc.unwrap())?;

        Ok(())
    }

    fn accumulate_random_instance_without_hiding<R: Rng>(
        rng: &mut R,
        n: usize,
        acc: Accumulator<PallasConfig>,
    ) -> Result<Accumulator<PallasConfig>> {
        let q_1 = Instance::rand_without_hiding(rng, n);
        let q_2 = Instance::rand_without_hiding(rng, n);
        q_1.check()?;
        q_2.check()?;
        let qs = vec![acc.into(), q_1, q_2];

        let acc = prover(rng, &qs)?;
        verifier(&qs, acc.clone())?;

        Ok(acc)
    }

    #[test]
    fn test_acc_scheme_zero() -> Result<()> {
        let mut rng = rand::thread_rng();
        let n_range = Uniform::new(2, 4);
        let n = 2_usize.pow(rng.sample(n_range));

        let m = rng.sample(n_range);
        let mut acc = Accumulator::zero(n, 4);
        for _ in 0..m {
            acc = accumulate_random_instance_without_hiding(&mut rng, n, acc)?;
        }

        decider(acc)?;

        Ok(())
    }

    #[test]
    fn test_acc_zero() -> Result<()> {
        let mut rng = rand::thread_rng();
        let n_range = Uniform::new(2, 4);
        let n = 2_usize.pow(rng.sample(n_range));
        let k = rng.sample(Uniform::new(2, 4));

        let qs = vec![Instance::zero(n); k];
        let acc = Accumulator::<PallasConfig>::zero(n, k);
        verifier(&qs, acc.clone())?;
        decider(acc)?;

        Ok(())
    }
}

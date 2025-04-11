#![allow(non_snake_case)]

//! Accumulation scheme based on the Discrete Log assumption, using bulletproofs-style IPP

use anyhow::ensure;
use anyhow::Context;
use anyhow::Result;
use ark_ff::PrimeField;
use ark_pallas::PallasConfig;
use ark_poly::DenseUVPolynomial;
use ark_poly::Polynomial;
use ark_serialize::CanonicalDeserialize;
use ark_serialize::CanonicalSerialize;
use ark_std::{UniformRand, Zero};
use rand::Rng;

use crate::pp::PublicParams;
use crate::{
    group::{construct_powers, point_dot, rho_1, PallasPoint, PallasPoly, PallasScalar},
    pcdl::{self, Instance},
};

// -------------------- Accumulation Data Structures --------------------

/// acc in the paper
#[derive(Debug, Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct Accumulator {
    pub(crate) q: Instance,
    pub pi_V: AccumulatorHiding,
}

impl Accumulator {
    pub fn new<R: Rng>(rng: &mut R, qs: &[Instance]) -> Result<Self> {
        prover(rng, qs)
    }

    pub fn verifier(self, qs: &[Instance]) -> Result<()> {
        verifier(qs, self)
    }

    pub fn decider(self) -> Result<()> {
        decider(self)
    }
}

/// pi_V in the paper, used for hiding only
#[derive(Debug, Clone, CanonicalSerialize, CanonicalDeserialize)]
pub struct AccumulatorHiding {
    pub(crate) h: PallasPoly,
    pub(crate) U: PallasPoint,
    pub(crate) w: PallasScalar,
}

#[derive(Clone, CanonicalSerialize)]
pub struct AccumulatedHPolys {
    h_0: Option<PallasPoly>,
    pub hs: Vec<pcdl::HPoly>,
    alpha: Option<PallasScalar>,
    alphas: Vec<PallasScalar>,
}

impl AccumulatedHPolys {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            h_0: None,
            hs: Vec::with_capacity(capacity),
            alphas: Vec::with_capacity(capacity + 1),
            alpha: None,
        }
    }

    pub fn set_alpha(&mut self, alpha: PallasScalar) {
        self.alphas = construct_powers(&alpha, self.alphas.capacity());
        self.alpha = Some(alpha)
    }

    // WARNING: This will panic if alphas has not been initialized, but should be fine since this is private
    pub fn get_poly(&self) -> PallasPoly {
        let mut h = PallasPoly::zero();
        if let Some(h_0) = &self.h_0 {
            h += h_0;
        }
        for i in 0..self.hs.len() {
            h += &(self.hs[i].get_poly() * self.alphas[i + 1]);
        }
        h
    }

    // WARNING: This will panic if alphas has not been initialized, but should be fine since this is private
    pub fn eval(&self, z: &PallasScalar) -> PallasScalar {
        let mut v = PallasScalar::zero();
        if let Some(h_0) = &self.h_0 {
            v += h_0.evaluate(z);
        }
        for i in 0..self.hs.len() {
            v += self.hs[i].eval(z) * self.alphas[i + 1];
        }
        v
    }
}

impl From<Accumulator> for Instance {
    fn from(acc: Accumulator) -> Instance {
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
pub fn common_subroutine(
    qs: &[Instance],
    pi_V: &AccumulatorHiding,
) -> Result<(PallasPoint, usize, PallasScalar, AccumulatedHPolys)> {
    let m = qs.len();
    let d = qs.first().context("No instances given")?.d;

    let pp = PublicParams::get_pp();

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
    hs.set_alpha(rho_1!(hs));

    // 7. Set the polynomial h(X) := Σ^n_(i=0) α^i · h_i(X) ∈ Fq[X].

    // 8. Compute the accumulated commitment C := Σ^n_(i=0) α^i · U_i.
    let C = point_dot(&hs.alphas, &Us);

    // 9. Compute the challenge z := ρ1(C, h) ∈ F_q.
    let z = rho_1![C, hs.alpha.unwrap()];

    // 10. Randomize C : C_bar := C + ω · S ∈ G.
    let C_bar = C + pp.S * w;

    // 11. Output (C_bar, d, z, h(X)).
    Ok((C_bar, d, z, hs))
}

pub fn prover<R: Rng>(rng: &mut R, qs: &[Instance]) -> Result<Accumulator> {
    // 1. Sample a random linear polynomial h_0 ∈ F_q[X],
    let h_0 = PallasPoly::rand(1, rng);

    // 2. Then compute a deterministic commitment to h_0: U_0 := PCDL.Commit_ρ0(ck_PC, h_0, d; ω = ⊥).
    let U_0 = pcdl::commit(&h_0, 1, None);

    // 3. Sample commitment randomness ω ∈ Fq, and set π_V := (h_0, U_0, ω).
    let w = PallasScalar::rand(rng);
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

pub fn verifier(qs: &[Instance], acc: Accumulator) -> Result<()> {
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

pub fn decider(acc: Accumulator) -> Result<()> {
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
        acc: Option<Accumulator>,
    ) -> Result<Accumulator> {
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

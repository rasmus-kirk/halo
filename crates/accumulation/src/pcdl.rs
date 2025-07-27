#![allow(non_snake_case, clippy::let_and_return)]

//! Bulletproofs-style polynomial commitments based on the Discrete Log assumption
use anyhow::{ensure, Result};
use ark_ec::CurveGroup;
use ark_ff::{AdditiveGroup, Field};
use ark_pallas::PallasConfig;
use ark_poly::{univariate::DensePolynomial, DenseUVPolynomial, Polynomial};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_std::{One, UniformRand};
use educe::Educe;
use rand::Rng;
use rayon::prelude::*;

use halo_group::{
    construct_powers, point_dot_affine, scalar_dot, PastaConfig, Point, Poly, PublicParams, Scalar,
};
use halo_poseidon::{Protocols, Sponge};

use crate::pedersen;

// -------------------- PCS Data Structures --------------------

/// The instances from the report. These represent polynomial commitment openings.
//#[derive(Debug, Clone, PartialEq, Eq, CanonicalSerialize, CanonicalDeserialize)]
#[derive(Educe, CanonicalSerialize, CanonicalDeserialize)]
#[educe(Debug, Clone, PartialEq, Eq)]
pub struct Instance<P: PastaConfig> {
    pub(crate) C: Point<P>,      // Commitment to the coefficints of a polynomial p
    pub(crate) d: usize,         // The degree of p
    pub(crate) z: Scalar<P>,     // The point to evaluate p at
    pub(crate) v: Scalar<P>,     // The evaluation of p(z) = v
    pub(crate) pi: EvalProof<P>, // The proof that p(z) = v
}

impl<P: PastaConfig> Instance<P> {
    pub fn new(C: Point<P>, d: usize, z: Scalar<P>, v: Scalar<P>, pi: EvalProof<P>) -> Self {
        Self { C, d, z, v, pi }
    }

    pub fn open<R: Rng>(
        rng: &mut R,
        p: Poly<P>,
        d: usize,
        z: &Scalar<P>,
        w: Option<&Scalar<P>>,
    ) -> Self {
        let C = commit(&p, d, w);
        let v = p.evaluate(z);
        let pi = open(rng, p, C, d, z, w);
        Self { C, d, z: *z, v, pi }
    }

    pub fn rand<R: Rng>(rng: &mut R, n: usize) -> Self {
        assert!(n.is_power_of_two(), "n ({n}) is not a power of two");
        let d = n - 1;
        let w = Some(Scalar::<P>::rand(rng));
        let p: Poly<P> = DenseUVPolynomial::<Scalar<P>>::rand(d, rng);
        let z = &Scalar::<P>::rand(rng);
        assert!(p.degree() == n - 1);
        Self::open(rng, p, d, z, w.as_ref())
    }

    pub fn check(&self) -> Result<()> {
        check(&self.C, self.d, &self.z, &self.v, self.pi.clone())
    }

    pub fn succinct_check(&self) -> Result<(HPoly<P>, Point<P>)> {
        succinct_check(self.C, self.d, &self.z, &self.v, self.pi.clone())
    }

    pub fn tuple(&self) -> (&Point<P>, &usize, &Scalar<P>, &Scalar<P>, &EvalProof<P>) {
        (&self.C, &self.d, &self.z, &self.v, &self.pi)
    }

    pub fn C(&self) -> &Point<P> {
        &self.C
    }

    pub fn d(&self) -> &usize {
        &self.d
    }

    pub fn z(&self) -> &Scalar<P> {
        &self.z
    }

    pub fn v(&self) -> &Scalar<P> {
        &self.v
    }

    pub fn pi(&self) -> &EvalProof<P> {
        &self.pi
    }

    pub fn into_tuple(self) -> (Point<P>, usize, Scalar<P>, Scalar<P>, EvalProof<P>) {
        (self.C, self.d, self.z, self.v, self.pi)
    }
}

// impl PartialOrd for Instance {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.z.cmp(&other.z))
//     }
// }

// impl Ord for Instance {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.z.cmp(&other.z)
//     }
// }

#[derive(Educe, CanonicalSerialize, CanonicalDeserialize)]
#[educe(Debug, Clone, PartialEq, Eq)]
pub struct EvalProof<P: PastaConfig> {
    pub(crate) Ls: Vec<Point<P>>,
    pub(crate) Rs: Vec<Point<P>>,
    pub(crate) U: Point<P>,
    pub(crate) c: Scalar<P>,
    pub(crate) C_bar: Option<Point<P>>,
    pub(crate) w_prime: Option<Scalar<P>>,
}

/// Special struct to denote the polynomial h(X). The struct is needed in order to evaluate h(X) in sub-linear time
#[derive(Clone, CanonicalSerialize)]
pub struct HPoly<P: PastaConfig> {
    pub(crate) xis: Vec<Scalar<P>>,
}

// TODO: Privacy
impl<P: PastaConfig> HPoly<P> {
    pub(crate) fn new(xis: Vec<Scalar<P>>) -> Self {
        Self { xis }
    }

    /// Constructs the polynomial h(X) based on the formula:
    /// h(X) := π^(lg(n)-1)_(i=0) (1 + ξ_(lg(n)−i) · X^(2^i)) ∈ F_q[X]
    pub(crate) fn get_poly(&self) -> DensePolynomial<Scalar<P>> {
        let mut h = DensePolynomial::from_coefficients_slice(&[Scalar::<P>::one()]); // Start with 1
        let lg_n = self.xis.len() - 1;

        for i in 0..lg_n {
            // Compute 2^i
            let power = 1 << i;

            // Create coefficients for 1 + ξ_(lg(n)-i) * X^(2^i)
            let mut term = vec![Scalar::<P>::ZERO; power + 1];
            term[0] = Scalar::<P>::one(); // Constant term 1
            term[power] = self.xis[lg_n - i]; // Coefficient for X^(2^i)

            // Create polynomial for this term
            let poly = DensePolynomial::from_coefficients_vec(term);

            // Multiply the current h(X) with the new term
            h = h * poly;
        }

        h
    }

    pub(crate) fn eval(&self, z: &Scalar<P>) -> Scalar<P> {
        let lg_n = self.xis.len() - 1;
        let one = Scalar::<P>::one();

        let mut v = one + self.xis[lg_n] * z;
        let mut z_i = *z;

        for i in 1..lg_n {
            z_i.square_in_place();
            v *= one + self.xis[lg_n - i] * z_i;
        }
        v
    }

    #[allow(dead_code)]
    pub(crate) fn rand<R: Rng>(rng: &mut R, n: usize) -> Self {
        let lg_n = n.ilog2() as usize;
        let mut xis = Vec::with_capacity(lg_n + 1);
        for _ in 0..(lg_n + 1) {
            xis.push(Scalar::<P>::rand(rng))
        }

        HPoly::new(xis)
    }
}

// TODO: Maybe move this?
// -------------------- Helper Traits -------------------- //
pub(crate) trait VecPushOwn<T> {
    fn push_own(self, value: T) -> Self;
}

impl<T> VecPushOwn<T> for Vec<T> {
    fn push_own(mut self, value: T) -> Self {
        self.push(value);
        self
    }
}

// -------------------- PCS Functions -------------------- //

/// Setup
///
/// Sets the public parameters. If not called, public parameters will be set to max.
/// This will degrade performance, but never fail.
pub fn setup(n: usize) -> Result<()> {
    PublicParams::<PallasConfig>::set_pp(n)
}

/// Creates a commitment to the coefficients of the polynomial $p$ of degree $d' < d$, with optional hiding $\o$, using pedersen commitments.
///
/// p: A univariate polynomial p(X),
/// d: A degree bound for p, we require that p.degree() <= d,
/// w: Optional hiding to pass to the underlying Pederson Commitment
pub fn commit<P: PastaConfig>(p: &Poly<P>, d: usize, w: Option<&Scalar<P>>) -> Point<P> {
    let pp = PublicParams::get_pp();
    let n = d + 1;
    let p_deg = p.degree();
    let pp_len = pp.len();
    let D = pp.D;

    assert!(n.is_power_of_two(), "n ({n}) is not a power of two");
    assert!(p_deg <= d, "p_deg ({p_deg}) <= d ({d})");
    assert!(d <= D, "d ({d}) <= D ({D}) (pp_len = {pp_len})",);

    pedersen::commit(w, &pp.Gs[0..n], &p.coeffs)
}

/// Creates a commitment to the coefficients of the polynomial $p$ of degree $d' < d$, with optional hiding $\o$, using pedersen commitments.
///
/// p: A univariate polynomial p(X),
/// d: A degree bound for p, we require that p.degree() <= d,
/// w: Optional hiding to pass to the underlying Pederson Commitment
pub fn chunked_commit<P: PastaConfig>(
    p: &Poly<P>,
    d: usize,
    w: Option<&Scalar<P>>,
    chunk_size: usize,
) -> Vec<Point<P>> {
    let pp = PublicParams::get_pp();
    let n = d + 1;
    // let p_deg = p.degree();
    let pp_len = pp.len();
    let D = pp.D;

    assert!(n.is_power_of_two(), "n ({n}) is not a power of two");
    // assert!(p_deg <= d, "p_deg ({p_deg}) <= d ({d})");
    assert!(d <= D, "d ({d}) <= D ({D}) (pp_len = {pp_len})",);

    p.coeffs()
        .chunks(chunk_size)
        .map(|x| pedersen::commit(w, &pp.Gs[0..n], x))
        .collect()
}

/// Creates a proof that states: "I know a polynomial p of degree d' less than d, with commitment C s.t. p(z) = v" where p is private and d, z, v are public.
/// Without evaluating p(X)
///
/// rng: Required since the function uses randomness
/// p: A univariate polynomial p(X)
/// C: A commitment to p,
/// d: A degree bound for p, we require that p.degree() <= d,
/// z: An evaluation point z
/// v: The evaluation of p(z)
/// w: Commitment randomness ω for the Pedersen Commitment C
pub fn open_without_eval<R: Rng, P: PastaConfig>(
    rng: &mut R,
    p: Poly<P>,
    C: Point<P>,
    d: usize,
    z: &Scalar<P>,
    v: &Scalar<P>,
    w: Option<&Scalar<P>>,
) -> EvalProof<P> {
    let pp = PublicParams::get_pp();
    let mut transcript = Sponge::<P>::new(Protocols::PCDL);
    let n = d + 1;
    let lg_n = n.ilog2() as usize;
    assert!(n > 1);
    assert!(n.is_power_of_two(), "n ({n}) is not a power of two");
    assert!(p.degree() <= d);
    assert!(d <= pp.D);

    let (p_prime, C_prime, w_prime, C_bar) = if let Some(w) = w {
        // (2). Sample a random polynomial p_bar ∈ F^(≤d)_q[X] such that p_bar(z) = 0.
        // p_bar(X) = (X - z) * q(X), where q(X) is a uniform random polynomial
        let z_poly = Poly::<P>::from_coefficients_vec(vec![-*z, Scalar::<P>::ONE]);
        let q: Poly<P> = DenseUVPolynomial::<Scalar<P>>::rand(d - 1, rng);
        let p_bar = q * z_poly;

        // (3). Sample corresponding commitment randomness ω_bar ∈ Fq.
        let w_bar = Scalar::<P>::rand(rng);

        // (4). Compute a hiding commitment to p_bar: C_bar ← CM.Commit^(ρ0)(ck, p_bar; ω_bar) ∈ G.
        let C_bar = commit(&p_bar, d, Some(&w_bar));

        // (5). Compute the challenge α := ρ(C, z, v, C_bar) ∈ F^∗_q.
        transcript.absorb_g(&[C, C_bar]);
        transcript.absorb_fr(&[*z, *v]);
        let a = transcript.challenge();

        // 6. Compute the polynomial p' := p + α ⋅ p_bar = Σ^d_(i=0) c_i ⋅ X_i ∈ Fq[X].
        let p_prime = p + &p_bar * a;

        // 7. Compute commitment randomness ω' := ω + α ⋅ ω_bar ∈ Fq.
        let w_prime = w_bar * a + w;

        // 8. Compute a non-hiding commitment to p' : C' := C + α ⋅ C_bar - ω' ⋅ S ∈ G.
        let C_prime = C + C_bar * a - pp.S * w_prime;

        (p_prime, C_prime, Some(w_prime), Some(C_bar))
    } else {
        // 6. Compute the polynomial p' := p + α ⋅ p_bar = Σ^d_(i=0) c_i ⋅ X_i ∈ Fq[X].
        let p_prime = p;

        // 8. Compute a non-hiding commitment to p' : C' := C + α ⋅ C_bar - ω' ⋅ S ∈ G.
        let C_prime = C;

        (p_prime, C_prime, None, None)
    };

    // Compute the 0-th challenge field element ξ_0 := ρ0(C', z, v) ∈ F_q, and use it to compute the group element
    // H' := ξ_0 ⋅ H ∈ G. Initialize the following vectors:
    // c_0 := (c_0, c_1, . . . , c_d) ∈ F^(d+1)_q
    // z_0 := (1, z, . . . , z^d) ∈ F^(d+1)_q
    // G_0 := (G_0, G_1, . . . , G_d) ∈ G_(d+1)
    transcript.absorb_g(&[C_prime]);
    transcript.absorb_fr(&[*z, *v]);
    let mut xi_i = transcript.challenge();
    let H_prime = pp.H * xi_i;

    let mut cs = p_prime.coeffs;
    cs.resize(n, Scalar::<P>::ZERO);
    let mut gs = pp.Gs[0..n].to_vec();
    let mut zs: Vec<P::ScalarField> = construct_powers::<P>(z, n);

    let mut Ls = Vec::with_capacity(lg_n);
    let mut Rs = Vec::with_capacity(lg_n);

    let mut m = n / 2;

    // NOTE: i is zero-indexed here, but one-indexed in spec,
    // and that i has been corrected in below comments.
    for _ in 0..lg_n {
        // 1&2. Setting Σ_L := l(G_i) || H', Σ_R := r(G i) || H', compute:
        // L_(i+1) := CM.Commit_(Σ_L)(r(c_i) || ⟨r(c_i), l(z_i)⟩)
        // R_(i+1) := CM.Commit_(Σ_R)(l(c_i) || ⟨l(c_i), r(z_i)⟩)
        let (gs_l, gs_r) = gs.split_at_mut(m);
        let (cs_l, cs_r) = cs.split_at_mut(m);
        let (zs_l, zs_r) = zs.split_at_mut(m);

        let dot_l = scalar_dot::<P>(cs_r, zs_l);
        let L = point_dot_affine(cs_r, gs_l) + H_prime * dot_l;
        Ls.push(L);

        let dot_r = scalar_dot::<P>(cs_l, zs_r);
        let R = point_dot_affine(cs_l, gs_r) + H_prime * dot_r;
        Rs.push(R);

        // 3. Generate the (i+1)-th challenge ξ_(i+1) := ρ_0(ξ_i, L_(i+1), R_(i+1)) ∈ F_q.
        transcript.absorb_fr(&[xi_i]);
        transcript.absorb_g(&[L, R]);
        let xi_next = transcript.challenge();
        let xi_next_inv = xi_next.inverse().unwrap();
        xi_i = xi_next;

        gs_l.par_iter_mut().take(m).enumerate().for_each(|(j, g)| {
            *g = (*g + gs_r[j] * xi_next).into_affine();
        });
        cs_l.par_iter_mut().take(m).enumerate().for_each(|(j, c)| {
            *c += cs_r[j] * xi_next_inv;
        });
        zs_l.par_iter_mut().take(m).enumerate().for_each(|(j, z)| {
            *z += zs_r[j] * xi_next;
        });

        m /= 2;
    }

    // Finally, set U := G_(log_n), c := c_(log_n), and output the evaluation proof π := (L, R, U, c, C_bar, ω').
    let U = gs[0].into();
    let c = cs[0];
    let pi = EvalProof {
        Ls,      // L
        Rs,      // R
        c,       // a[0]
        U,       // G[0]
        C_bar,   // For constructing C_prime
        w_prime, // For constructing C_prime
    };

    pi
}

/// Creates a proof that states: "I know a polynomial p of degree d' less than d, with commitment C s.t. p(z) = v" where p is private and d, z, v are public.
///
/// rng: Required since the function uses randomness
/// p: A univariate polynomial p(X)
/// C: A commitment to p,
/// d: A degree bound for p, we require that p.degree() <= d,
/// z: An evaluation point z
/// w: Commitment randomness ω for the Pedersen Commitment C
pub fn open<R: Rng, P: PastaConfig>(
    rng: &mut R,
    p: Poly<P>,
    C: Point<P>,
    d: usize,
    z: &Scalar<P>,
    w: Option<&Scalar<P>>,
) -> EvalProof<P> {
    let v = p.evaluate(z);
    open_without_eval(rng, p, C, d, z, &v, w)
}

/// Cheaply checks that a proof, pi, is correct. It is not a full check
/// however, since an expensive part of the check is deferred until a later point.
///
/// C: A commitment to p,
/// d: A degree bound for p, we require that p.degree() <= d,
/// z: An evaluation point z
/// v: v = p(z)
/// pi: The evaluation proof
pub fn succinct_check<P: PastaConfig>(
    C: Point<P>,
    d: usize,
    z: &Scalar<P>,
    v: &Scalar<P>,
    pi: EvalProof<P>,
) -> Result<(HPoly<P>, Point<P>)> {
    let pp = PublicParams::get_pp();
    let n = d + 1;
    let lg_n = n.ilog2() as usize;
    assert!(n.is_power_of_two(), "n ({n}) is not a power of two");
    ensure!(d <= pp.D, "d was larger than D!");

    let mut transcript = Sponge::new(Protocols::PCDL);

    // 1. Parse rk as (⟨group⟩, S, H, d'), and π as (L, R, U, c, C_bar, ω').
    #[rustfmt::skip]
    let EvalProof { Ls, Rs, U, c, C_bar, w_prime } = pi;

    // 2. Check that d = d'. Irrelevant, we just removed d'
    //ensure!(d == d_prime, "d ≠ d'");

    // 4. Compute the non-hiding commitment C' := C + α · C_bar − ω'· S ∈ G.
    let C_prime = if let Some(C_bar) = C_bar {
        // (3). Compute the challenge α := ρ_0(C, z, v, C_bar) ∈ F^∗_q.
        transcript.absorb_g(&[C, C_bar]);
        transcript.absorb_fr(&[*z, *v]);
        let a = transcript.challenge();

        C + C_bar * a - pp.S * w_prime.unwrap()
    } else {
        C
    };

    // 5. Compute the 0-th challenge ξ_0 := ρ_0(C', z, v), and set H' := ξ_0 · H ∈ G.
    transcript.absorb_g(&[C_prime]);
    transcript.absorb_fr(&[*z, *v]);
    let xi_0 = transcript.challenge();
    let mut xis = Vec::with_capacity(lg_n + 1).push_own(xi_0);

    let H_prime = pp.H * xi_0;

    // 6. Compute the group element C_0 := C' + v · H' ∈ G.
    let mut C_i = C_prime + H_prime * v;

    // 7. For each i ∈ [log_n]:
    for i in 0..lg_n {
        // 7.a Generate the (i+1)-th challenge: ξ_(i+1) := ρ_0(ξ_i, L_i, R_i) ∈ F_q.
        transcript.absorb_fr(&[xis[i]]);
        transcript.absorb_g(&[Ls[i], Rs[i]]);
        let xi_next = transcript.challenge();
        xis.push(xi_next);

        // 7.b Compute the (i+1)-th commitment: C_(i+1) := C_i + ξ^(−1)_(i+1) · L_i + ξ_(i+1) · R_i ∈ G.
        C_i += Ls[i] * xi_next.inverse().unwrap() + Rs[i] * xi_next;
    }

    // 8. Define the univariate polynomial h(X) := π^(lg(n))_(i=0) (1 + ξ_(lg(n)−i) · X^(2^i)) ∈ F_q[X].
    let h = HPoly::new(xis);

    // 9. Compute the evaluation v' := c · h(z) ∈ F_q.
    let v_prime = c * h.eval(z);

    // 10. Check that C_(log_n) = CM.Commit_Σ(c || v'), where Σ = (U || H').
    ensure!(
        C_i == U * c + H_prime * v_prime,
        "C_(log_n) ≠ CM.Commit_Σ(c || v')"
    );

    // 11. Output (h, U).
    Ok((h, U))
}

/// The full check on the evaluation proof, pi
///
/// C: A commitment to p,
/// d: A degree bound for p, we require that p.degree() <= d,
/// z: An evaluation point z
/// v: v = p(z)
/// pi: The evaluation proof
pub fn check<P: PastaConfig>(
    C: &Point<P>,
    d: usize,
    z: &Scalar<P>,
    v: &Scalar<P>,
    pi: EvalProof<P>,
) -> Result<()> {
    let pp = PublicParams::get_pp();
    // 1. Parse ck as (⟨group⟩, hk, S).
    // 2. Set d' := |hk| - 1.
    // 3. Set rk := (⟨group⟩, S, H, d').

    // 4. Check that PC_DL.SuccinctCheck_ρ0(rk, C, d, z, v, π) accepts and outputs (h, U).
    let (h, U) = succinct_check(*C, d, z, v, pi)?;

    // 5. Check that U = CM.Commit(ck, h_vec), where h_vec is the coefficient vector of the polynomial h.
    let comm = pedersen::commit(None, &pp.Gs[0..(d + 1)], &h.get_poly().coeffs);
    ensure!(U == comm, "U ≠ CM.Commit(ck, h_vec)");

    Ok(())
}

// -------------------- Tests -------------------- //

#[cfg(test)]
mod tests {
    use ark_std::UniformRand;
    use rand::distributions::Uniform;

    use halo_group::{PallasPoint, PallasPoly, PallasScalar};

    use super::*;

    #[test]
    fn test_z() {
        let mut rng = rand::thread_rng();
        let n_range = Uniform::new(2, 10);
        let n = 2_usize.pow(rng.sample(n_range));
        let lg_n = n.ilog2() as usize;
        let one = PallasScalar::one();

        let z = PallasScalar::rand(&mut rng);
        let mut xis = Vec::with_capacity(lg_n + 1);
        for _ in 0..(lg_n + 1) {
            xis.push(PallasScalar::rand(&mut rng));
        }

        let mut v_1 = one + xis[lg_n] * z;
        let mut z_i = z;
        for i in 1..lg_n {
            z_i.square_in_place();
            v_1 *= one + xis[lg_n - i] * z_i;
        }

        let mut v_2 = one;
        for i in 0..lg_n {
            let power: u64 = 1 << i;
            v_2 *= one + xis[lg_n - i] * z.pow([power]);
        }

        assert_eq!(v_1, v_2);
    }

    #[test]
    fn test_u_check() -> Result<()> {
        let n = 2_usize.pow(3);
        let lg_n = n.ilog2() as usize;

        let pp = PublicParams::get_pp();

        // Generate fake values
        let xis: Vec<PallasScalar> = vec![0, 1, 2, 3]
            .into_iter()
            .map(PallasScalar::from)
            .collect();

        let gs_affine = &pp.Gs[0..n];
        let gs: Vec<PallasPoint> = gs_affine.iter().map(|x| PallasPoint::from(*x)).collect();
        let mut gs_mut = gs.clone();

        for i in 0..lg_n {
            let (g_l, g_r) = gs_mut.split_at(gs_mut.len() / 2);

            let xi_next = xis[i + 1];

            let mut g = Vec::with_capacity(g_l.len());
            for j in 0..g_l.len() {
                // 4. Construct the commitment key for the next round: G_(i+1) := l(G_i) + ξ_(i+1) · r(G_i).
                g.push(g_l[j] + g_r[j] * xi_next);
            }
            gs_mut = g;
        }

        let g0_expected: PallasPoint = vec![
            gs[0],
            gs[1] * xis[3],
            gs[2] * xis[2],
            gs[3] * xis[2] * xis[3],
            gs[4] * xis[1],
            gs[5] * xis[1] * xis[3],
            gs[6] * xis[1] * xis[2],
            gs[7] * xis[1] * xis[2] * xis[3],
        ]
        .iter()
        .sum();

        assert_eq!(gs_mut.len(), 1);
        assert_eq!(g0_expected, gs_mut[0]);

        let h = HPoly::<PallasConfig>::new(xis.clone());
        let h_coeffs = h.get_poly().coeffs;
        let U = gs_mut[0];
        let U_prime = pedersen::commit(None, gs_affine, &h_coeffs);

        let mut xs = Vec::with_capacity(gs.len());
        let mut acc = PallasPoint::ZERO;
        for i in 0..gs.len() {
            acc += gs[i] * h_coeffs[i];
            xs.push(gs[i] * h_coeffs[i])
        }

        assert_eq!(U, U_prime);

        Ok(())
    }

    #[test]
    fn test_check() -> Result<()> {
        let rng = &mut rand::thread_rng();
        let n = 2_usize.pow(rng.sample(Uniform::new(2, 10)));

        // Verify that check works
        Instance::<PallasConfig>::rand(rng, n).check()?;

        Ok(())
    }

    #[test]
    fn test_check_no_hiding() -> Result<()> {
        let mut rng = rand::thread_rng();
        let n = 2_usize.pow(rng.sample(Uniform::new(2, 10)));
        let d = n - 1;
        let d_prime = rng.sample(Uniform::new(1, d));

        // Commit to a random polynomial
        let p = PallasPoly::rand(d_prime, &mut rng);
        let C = commit::<PallasConfig>(&p, d, None);

        // Generate an evaluation proof
        let z = PallasScalar::rand(&mut rng);
        let v = p.evaluate(&z);
        let pi = open(&mut rng, p, C, d, &z, None);

        // Verify that check works
        check(&C, d, &z, &v, pi)?;

        Ok(())
    }

    #[test]
    fn test_construct_h_with_degree_7() {
        let mut rng = rand::thread_rng();
        let n = 2_usize.pow(3);
        let lg_n = n.ilog2() as usize;
        let xis_len = lg_n + 1;

        let xis: Vec<PallasScalar> = vec![PallasScalar::ZERO; xis_len]
            .iter()
            .map(|_| PallasScalar::rand(&mut rng))
            .collect();
        let coeffs = vec![
            PallasScalar::one(),
            xis[3],
            xis[2],
            xis[2] * xis[3],
            xis[1],
            xis[1] * xis[3],
            xis[1] * xis[2],
            xis[1] * xis[2] * xis[3],
        ];
        let h = HPoly::<PallasConfig>::new(xis);

        assert_eq!(h.get_poly().coeffs, coeffs);
    }
}

use anyhow::{Result, ensure};
use halo_accumulation::pcdl::{self, EvalProof};
use halo_group::{
    PastaConfig, Point, Poly,
    ark_ff::Field,
    ark_poly::{EvaluationDomain, Polynomial, Radix2EvaluationDomain},
    ark_std::{Zero, rand::Rng},
};
use halo_poseidon::Sponge;

pub struct VanishingArgumentProof<P: PastaConfig> {
    n: usize,
    C_t: Point<P>,
    v_t: P::ScalarField,
    pi_t: EvalProof<P>,
    C_fs: Vec<Point<P>>,
    v_fs: Vec<P::ScalarField>,
    pi_fs: Vec<EvalProof<P>>,
}

impl<P: PastaConfig> VanishingArgumentProof<P> {
    pub fn prove<R: Rng>(
        rng: &mut R,
        transcript: &mut Sponge<P>,
        fs: &[Poly<P>],
        n: usize,
    ) -> Self {
        assert!(n.is_power_of_two());
        let d = n - 1;

        // 1. The prover commits to each f_i and sends each commitment to the verifier
        let C_fs: Vec<Point<P>> = fs.iter().map(|f| pcdl::commit(f, d, None)).collect();
        transcript.absorb_g(&C_fs);

        // 2. The verifier sends a random challenge α to the prover.
        let alpha = transcript.challenge(); // Verifier sends challenge alpha

        // 3. The prover constructs t(X):
        let mut sum = Poly::<P>::zero();
        for (i, f) in fs.iter().enumerate() {
            sum = sum + f * alpha.pow([i as u64])
        }
        let domain = Radix2EvaluationDomain::new(n).unwrap();
        let t = sum.mul_by_vanishing_poly(domain);

        // 4. The prover sends a commitment to t(X)
        let C_t: Point<P> = pcdl::commit(&t, d, None);
        transcript.absorb_g(&[C_t]);

        // 5. The verifier sends challenge ξ to the prover.
        let xi = transcript.challenge();

        // 6. The prover sends (f_i(ξ) = vfi, πfi, t(ξ) = vt, πf) to the verifier.
        let mut pi_fs = Vec::with_capacity(fs.len());
        let mut v_fs = Vec::with_capacity(fs.len());
        for i in 0..fs.len() {
            let v = fs[i].evaluate(&xi);
            pi_fs.push(pcdl::open_without_eval(
                rng,
                fs[i].clone(),
                C_fs[i],
                d,
                &xi,
                &v,
                None,
            ));
            v_fs.push(v);
        }

        let v_t = t.evaluate(&xi);
        let pi_t = pcdl::open_without_eval(rng, t, C_t, d, &xi, &v_t, None);

        VanishingArgumentProof {
            n,
            C_t,
            v_t,
            pi_t,
            C_fs,
            v_fs,
            pi_fs,
        }
    }

    pub fn verify(transcript: &mut Sponge<P>, pi: Self) -> Result<()> {
        ensure!(pi.n.is_power_of_two());
        let d = pi.n - 1;

        transcript.absorb_g(&pi.C_fs);
        let alpha = transcript.challenge();
        transcript.absorb_g(&[pi.C_t]);
        let xi = transcript.challenge();

        let domain = Radix2EvaluationDomain::new(pi.n).unwrap();
        let z_S = domain.vanishing_polynomial();

        let mut sum = P::ScalarField::zero();
        for (i, v_f) in pi.v_fs.iter().enumerate() {
            sum += alpha.pow([i as u64]) * v_f
        }

        ensure!(sum == pi.v_t * z_S.evaluate(&xi));
        for ((v_f, C_f), pi_f) in pi.v_fs.iter().zip(pi.C_fs).zip(pi.pi_fs) {
            pcdl::check(&C_f, d, &xi, &v_f, pi_f)?
        }
        pcdl::check(&pi.C_t, d, &xi, &pi.v_t, pi.pi_t)?;

        Ok(())
    }
}

pub struct BatchedEvaluationProofs<P: PastaConfig> {
    n: usize,
    C_fs: Vec<Point<P>>,
    v_fs: Vec<P::ScalarField>,
    pi_w: EvalProof<P>,
}

impl<P: PastaConfig> BatchedEvaluationProofs<P> {
    pub fn prove<R: Rng>(
        rng: &mut R,
        transcript: &mut Sponge<P>,
        fs: &[Poly<P>],
        n: usize,
    ) -> Self {
        assert!(n.is_power_of_two());
        let d = n - 1;

        let C_fs: Vec<_> = fs.iter().map(|f| pcdl::commit(&f, d, None)).collect();
        transcript.absorb_g(&C_fs);
        let xi = transcript.challenge();
        let v_fs: Vec<_> = fs.iter().map(|f| f.evaluate(&xi)).collect();

        transcript.absorb_fr(&v_fs);
        let alpha = transcript.challenge();

        let mut w = Poly::<P>::zero();
        for (i, f) in fs.iter().enumerate() {
            w = w + f * alpha.pow([i as u64])
        }
        let C_w = C_fs.iter().sum();

        let pi_w = pcdl::open(rng, w, C_w, d, &xi, None);

        Self {
            n,
            C_fs,
            v_fs,
            pi_w,
        }
    }

    pub fn verify(transcript: &mut Sponge<P>, pi: Self, n: usize) -> Result<()> {
        assert!(n.is_power_of_two());
        let d = n - 1;

        transcript.absorb_g(&pi.C_fs);
        let xi = transcript.challenge();
        transcript.absorb_fr(&pi.v_fs);
        let alpha = transcript.challenge();

        let C_w: Point<P> = pi
            .C_fs
            .iter()
            .enumerate()
            .map(|(i, C_f)| *C_f * alpha.pow([i as u64]))
            .sum();
        let v_w: P::ScalarField = pi
            .v_fs
            .iter()
            .enumerate()
            .map(|(i, v_f)| *v_f * alpha.pow([i as u64]))
            .sum();

        pcdl::check(&C_w, d, &xi, &v_w, pi.pi_w)?;

        Ok(())
    }
}

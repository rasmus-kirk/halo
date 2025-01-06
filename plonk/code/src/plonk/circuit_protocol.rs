use super::{
    circuit_abstract::{ConstVec, EqClasses, GateVec},
    circuit_snark::SnarkProof,
    transcript::TranscriptProtocol,
    utils::{check, commit, get_omega, interpolate, open, pu, x, Evals, Poly, Scalar},
};
use ark_ff::{AdditiveGroup, Field};
use halo_accumulation::group::PallasScalar;
use merlin::Transcript;
use rand::rngs::ThreadRng;
use rand::Rng;

#[derive(Debug)]
pub struct Circuit<const M: usize> {
    pub n: u64,
    pub order: u64,
    pub classes: EqClasses,
    pub gates: GateVec,
    pub consts: ConstVec,
    pub evals: Evals<8>,
    pub inputs: [PallasScalar; M],
    w1: Scalar,
}

impl<const M: usize> Circuit<M> {
    pub fn new(
        // public data
        n: u64,
        order: u64,
        classes: EqClasses,
        gates: GateVec,
        // private data
        consts: ConstVec,
        evals: Evals<8>,
        inputs: [PallasScalar; M],
    ) -> Self {
        Self {
            n,
            order,
            classes,
            gates,
            consts,
            evals,
            inputs,
            w1: get_omega(order),
        }
    }

    /// Interpolate the points to a polynomial
    fn fft(&self, points: &[Scalar]) -> Poly {
        // println!("points len: {}", points.len());
        let ppoints: Vec<PallasScalar> = points.iter().map(|&x| x.into()).collect();
        interpolate(self.order, &ppoints).into()
    }

    fn w(&self, i: u64) -> Scalar {
        self.w1.pow(i)
    }

    /// Lagrange basis polynomial
    ///  L_x(X) = c_x (X^n - 1)/(X - x)
    fn l(&self, i: u64) -> Poly {
        let one: Scalar = PallasScalar::ONE.into();
        (self.w(i) * (x(self.order) - one)) / (pu(&self.order) * (x(1) - self.w(i)))
    }

    /// Compute wire polynomials
    fn wire_polynomials(&self) -> [Poly; 8] {
        let mut polys = Vec::with_capacity(8);
        for e in &self.evals {
            let points = e.iter().map(|&x| x.into()).collect::<Vec<Scalar>>();
            polys.push(self.fft(&points));
        }
        polys.try_into().unwrap()
    }

    fn public_wire_polynomials(&self) -> [Poly; 5] {
        let [_, _, _, ql, qr, qo, qm, qc] = self.wire_polynomials();
        [ql, qr, qo, qm, qc]
    }

    fn compute_coset(&self, rng: &mut ThreadRng) -> (Scalar, Scalar) {
        // compute coset
        // k1 |-> $ ∉ { ω^i }_i∈[N]
        let k1 = loop {
            let k1 = rng.gen::<Scalar>();
            if (0..self.order).all(|i| k1 != self.w(i)) {
                break k1;
            }
        };
        // k2 |-> $ ∉ { ω^i, k1 ω^i }_i∈[N]
        let k2 = loop {
            let k2 = rng.gen::<Scalar>();
            if (0..self.order).all(|i| k2 != self.w(i) && k2 != k1 * self.w(i)) {
                break k2;
            }
        };
        (k1, k2)
    }

    /// Copy constraint polynomials points wires to wires
    /// let i,l ∈ { a, b, c } and j,m ∈ [n]
    /// σi_j(ω^j) = kl ω^m => i_j |-> l_m
    /// to form cycles of wires in the equivalence classes
    /// [ x, y, z ] => x |-> y, y |-> z, z |-> x
    fn copy_constraints_evals(&self) -> [Vec<Scalar>; 3] {
        let (k1, k2) = self.compute_coset(&mut ThreadRng::default());
        let ks = [PallasScalar::ONE.into(), k1, k2];
        let mut sabce = vec![vec![PallasScalar::ZERO.into(); self.order as usize]; 3];
        for class in self.classes.iter() {
            let l = class.len();
            for i in 0..l {
                let (v1, i1) = class[i];
                let (v2, i2) = class[if i == l - 1 { 0 } else { i + 1 }];
                // println!("{} {} -> {} {} [{}]", v1, i1, v2, i2, self.n * v2 as u64 + (i2 as u64 + 1));
                // sabce[v1][i1] = ks[v2] * self.w(i2 as u64);
                sabce[v1][i1] = Scalar::from(self.order * v2 as u64 + (i2 as u64 + 1));
            }
        }
        sabce.try_into().unwrap()
    }

    fn id_permutation(&self) -> [Poly; 3] {
        let mut sabce = vec![vec![PallasScalar::ZERO.into(); self.order as usize]; 3];
        for i in 0..self.order {
            sabce[0][i as usize] = Scalar::from(i + 1);
            sabce[1][i as usize] = Scalar::from(self.order + i + 1);
            sabce[2][i as usize] = Scalar::from(self.order * 2 + i + 1);
        }
        [
            self.fft(&sabce[0]),
            self.fft(&sabce[1]),
            self.fft(&sabce[2]),
        ]
    }

    fn copy_constraints(&self) -> [Poly; 3] {
        let sabce = self.copy_constraints_evals();
        [
            self.fft(&sabce[0]),
            self.fft(&sabce[1]),
            self.fft(&sabce[2]),
        ]
    }

    pub fn prove(&self, rng: &mut ThreadRng) -> SnarkProof {
        println!("---");
        println!("Plonk: Prover is computing the SNARK proof...\n");
        // convenience definitions
        // --------------------------------------------------------------------

        // `PallasScalar` constants
        let one: Scalar = PallasScalar::ONE.into();
        let zero: Scalar = PallasScalar::ZERO.into();

        // Round: Precomputation
        // --------------------------------------------------------------------
        // assert!(self.n <= self.order);

        // wire polynomials
        let [fa, fb, fc, ql, qr, qo, qm, qc] = &self.wire_polynomials();

        // copy constraint polynomials
        let [sae, sbe, sce] = &self.copy_constraints_evals();
        let [sida, sidb, sidc] = &self.id_permutation();
        // println!("IM FFTING SA !!!!!!!!");
        let [sa, sb, sc] = &self.copy_constraints();

        // transcript
        let mut transcript = Transcript::new(b"protocol");
        transcript.domain_sep();

        // zh(x) = x^N - 1 where zh(ω^i) = 0 for i∈[N]
        let zh = &(x(self.order) - one);
        for i in 0..self.order {
            assert!(zh.evaluate_(self.w(i)) == zero);
        }

        // Round: Permutation Polynomial
        // --------------------------------------------------------------------

        // β = H(transcript, 0)
        let beta = &transcript.challenge_scalar_augment(0, b"beta");
        // γ = H(transcript, 1)
        let gamma = &transcript.challenge_scalar_augment(1, b"gamma");
        // println!("beta: {}", beta);
        // println!("gamma: {}", gamma);

        let ae = |i: u64| Scalar::from(self.evals[0][i as usize]);
        let be = |i: u64| Scalar::from(self.evals[1][i as usize]);
        let ce = |i: u64| Scalar::from(self.evals[2][i as usize]);
        // accumulator polynomial
        // // a_i + β (i+1) + γ
        // let f_a = |i: u64| (fa.evaluate(&self.w(i)) + beta * sida.evaluate(&self.w(i)) + gamma);
        // // b_i + β n (i+1) + γ
        // let f_b = |i: u64| (fb.evaluate(&self.w(i)) + beta * sidb.evaluate(&self.w(i)) + gamma);
        // // c_i + β 2n (i+1) + γ
        // let f_c = |i: u64| (fc.evaluate(&self.w(i)) + beta * sidc.evaluate(&self.w(i)) + gamma);
        // // a_i + β σa(ω^i) + γ
        // let g_a = |i: u64| (fa.evaluate(&self.w(i)) + beta * sae[i as usize] + gamma);
        // // b_i + β σb(ω^i) + γ
        // let g_b = |i: u64| (fb.evaluate(&self.w(i)) + beta * sbe[i as usize] + gamma);
        // // c_i + β σc(ω^i) + γ
        // let g_c = |i: u64| (fc.evaluate(&self.w(i)) + beta * sce[i as usize] + gamma);

        let f_a = |i: u64| (ae(i) + beta * Scalar::from(i + 1) + gamma);
        // b_i + β n (i+1) + γ
        let f_b = |i: u64| (be(i) + beta * Scalar::from(self.order + i + 1) + gamma);
        // c_i + β 2n (i+1) + γ
        let f_c = |i: u64| (ce(i) + beta * Scalar::from(self.order * 2 + i + 1) + gamma);
        // a_i + β σa(ω^i) + γ
        let g_a = |i: u64| (ae(i) + beta * sae[i as usize] + gamma);
        // b_i + β σb(ω^i) + γ
        let g_b = |i: u64| (be(i) + beta * sbe[i as usize] + gamma);
        // c_i + β σc(ω^i) + γ
        let g_c = |i: u64| (ce(i) + beta * sce[i as usize] + gamma);

        // fp(ω^i) = (a_i + β (i+1) + γ) (b_i + β n (i+1) + γ) (c_i + β 2n (i+1) + γ)
        let mut fp_evals = vec![zero; (self.order) as usize];
        for i in 0..self.order {
            fp_evals[i as usize] = f_a(i) * f_b(i) * f_c(i);
        }
        let fp = &self.fft(&fp_evals);

        // gp(ω^i) = (a_i + β σa(ω^i) + γ) (b_i + β σb(ω^i) + γ) (c_i + β σc(ω^i) + γ)
        let mut gp_evals = vec![zero; (self.order) as usize];
        for i in 0..self.order {
            gp_evals[i as usize] = g_a(i) * g_b(i) * g_c(i);
        }
        let gp = &self.fft(&gp_evals);

        // ATTEMPT AT DEFINING Z VIA INTERPOLATION
        // z(ω^0) = 1
        // z(ω^i) = Π_{j=0}^{i} f(ω^j) / g(ω^j)
        let mut z_evals = vec![zero; (self.order + 1) as usize];
        for (i, eval) in z_evals.iter_mut().enumerate().take((self.order + 1) as usize) {
            *eval = if i == 0 {
                one
            } else {
                let mut result = one;
                for j in 0..i {
                    result = result * fp_evals[j] / gp_evals[j];
                }
                result
            };
        }
        let z = &self.fft(&z_evals);

        // ATTEMPT AT DEFINING Z VIA POLYNOMIAL COMPOSITON ALA PLONK PAPER
        // let mut prod_terms = vec![zero; self.n as usize];
        // for i in 0..self.n {
        //     let mut result = one;
        //     for j in 0..i {
        //         result = result * fp_evals[j as usize] / gp_evals[j as usize];
        //     }
        //     prod_terms[i as usize] = result;
        // }
        // let mut z_ = self.l(0);
        // for i in 0..self.n {
        //     let zp = z_.clone();
        //     let li = &self.l(i + 1);
        //     let pt = &prod_terms[i as usize];
        //     z_ = zp + li * pt;
        // }
        // let z = &z_;

        // Round: Challenge
        // --------------------------------------------------------------------

        // 𝔷 = H(transcript)
        let xi = &transcript.challenge_scalar(b"xi");
        // println!("xi: {}", xi);

        // Round: Gate Check
        // --------------------------------------------------------------------

        // α = H(transcript)
        let alpha = &transcript.challenge_scalar(b"alpha");

        let a_xi = &fa.evaluate(xi);
        let a_com = &commit(fa);
        let a_pi = &open(rng, fa, a_com, xi);

        let b_xi = &fb.evaluate(xi);
        let b_com = &commit(fb);
        let b_pi = &open(rng, fb, b_com, xi);

        let c_xi = &fc.evaluate(xi);
        let c_com = &commit(fc);
        let c_pi = &open(rng, fc, c_com, xi);

        // f_gc(X) = A(X) Q_L(X) + B(X) Q_R(X) + C(X) Q_O(X) + A(X) B(X) Q_M(X) + Q_C(X)
        let f_gc = &(fa * ql + fb * qr + fc * qo + fa * fb * qm + qc);
        let f_gc_com = &commit(f_gc);
        let f_gc_pi = &open(rng, f_gc, f_gc_com, xi);

        // Round: Copy Constraint Check (Base Case)
        // --------------------------------------------------------------------

        // lagrange basis polynomial
        let l1: &Poly = &self.l(0);

        let z_xi = &z.evaluate(xi);
        let z_com = &commit(z);
        let z_pi = &open(rng, z, z_com, xi);

        // f_cc1(X) = (Z(X) - 1) L_1(X)
        let f_cc1: &Poly = &((z - one) * l1);
        // println!("f_cc1_xi: {}", f_cc1.evaluate(xi));

        // Round: Copy Constraint Check (Inductive Case)
        // --------------------------------------------------------------------

        // zw(x) = z(ωx)
        let mut zw_evals = vec![zero; self.order as usize];
        for i in 0..self.order {
            zw_evals[i as usize] = z.evaluate(&self.w(i + 1));
        }
        let zw = &self.fft(&zw_evals);
        for i in 0..self.order {
            assert!(z.evaluate(&self.w(i + 1)) == zw.evaluate(&self.w(i)));
        }
        assert!(z.evaluate(&(self.w1 * xi)) == zw.evaluate(xi));

        let zw_xi = &z.evaluate(&(xi * self.w1));
        let zw_pi = &open(rng, z, z_com, &(xi * self.w1));

        // f_cc2(X) = Z(X) f'(X) - g'(X) Z(ω X)
        let f_cc2 = &((z * fp) - (gp * zw));
        let f_cc2_xi = &f_cc2.evaluate(xi);
        let f_cc2_com = &commit(f_cc2);
        let f_cc2_pi = &open(rng, f_cc2, f_cc2_com, xi);

        let fp_xi = &fp.evaluate(xi);
        let fp_com = &commit(fp);
        let fp_pi = &open(rng, fp, fp_com, xi);

        let gp_xi = &gp.evaluate(xi);
        let gp_com = &commit(gp);
        let gp_pi = &open(rng, gp, gp_com, xi);

        let fp_v = &(fp - (fa + beta * sida + gamma) * (fb + beta * sidb + gamma) * (fc + beta * sidc + gamma));
        println!("fp_v_xi AGAIN!: {}", fp_v.evaluate(xi));
        let gp_v = &(gp - (fa + beta * sa + gamma) * (fb + beta * sb + gamma) * (fc + beta * sc + gamma));
        


        println!("a_xi: {}", fa.evaluate(xi));
        println!("b_xi: {}", fb.evaluate(xi));
        println!("c_xi: {}", fc.evaluate(xi));
        println!("sida_xi: {}", sida.evaluate(xi));
        println!("sidb_xi: {}", sidb.evaluate(xi));
        println!("sidc_xi: {}", sidc.evaluate(xi));
        println!("f_cc2_xi: {}", f_cc2.evaluate(xi));
        println!("z_xi: {}", z.evaluate(xi));
        println!("fp_xi: {}", fp.evaluate(xi));
        println!("gp_xi: {}", gp.evaluate(xi));
        println!("zw_xi: {}, {}", zw.evaluate(xi), z.evaluate(&(xi * self.w1)));

        let t = &((f_gc + alpha * f_cc1 + alpha.pow(2) * f_cc2) / zh);
        let t_xi = &t.evaluate(xi);
        let t_com = &commit(t);
        let t_pi = &open(rng, t, t_com, xi);

        println!("alpha: {}", alpha);
        println!("fp_v_xi!!!!!!!: {}", fp_v.evaluate(xi));
        println!("gp_v_xi: {}", gp_v.evaluate(xi));
        println!("f_gc_xi: {}", f_gc.evaluate(xi));
        println!("f_cc1_xi: {}", f_cc1.evaluate(xi));
        println!("f_cc2_xi: {}", f_cc2.evaluate(xi));
        println!("t_xi: {}", t.evaluate(xi));
        println!("zh_xi: {}", zh.evaluate(xi));
        let f_gc_eval = f_gc.evaluate(xi);
        let f_cc1_eval = f_cc1.evaluate(xi);
        let f_cc2_eval = f_cc2.evaluate(xi);
        let fp_v_eval = fp_v.evaluate(xi);
        let gp_v_eval = gp_v.evaluate(xi);
        let t_eval = t.evaluate(xi);
        let zh_eval = zh.evaluate(xi);

        let t_fp = fp_v / zh;
        let t_gc = f_gc / zh;
        let t_gp = gp_v / zh;
        for i in 0..self.order*50 {
            assert!(t_gc.evaluate(&self.w(i)) * zh.evaluate(&self.w(i)) - f_gc.evaluate(&self.w(i)) == zero);
            assert!(t_fp.evaluate(&self.w(i)) * zh.evaluate(&self.w(i)) - fp_v.evaluate(&self.w(i)) == zero);
            assert!(t_gp.evaluate(&self.w(i)) * zh.evaluate(&self.w(i)) - gp_v.evaluate(&self.w(i)) == zero);
            let gp_v_xi = gp.evaluate(&self.w(i)) - (fa.evaluate(&self.w(i)) + beta * sa.evaluate(&self.w(i)) + gamma) * (fb.evaluate(&self.w(i)) + beta * sb.evaluate(&self.w(i)) + gamma) * (fc.evaluate(&self.w(i)) + beta * sc.evaluate(&self.w(i)) + gamma);
            assert!(t_gp.evaluate(&self.w(i)) * zh.evaluate(&self.w(i)) - gp_v_xi == zero);
        }
        assert!(t_gp.evaluate(xi) * zh.evaluate(xi) - gp_v.evaluate(xi) == zero);
        let gp_v_xi = gp_xi - (a_xi + beta * sa.evaluate(xi) + gamma) * (b_xi + beta * sb.evaluate(xi) + gamma) * (c_xi + beta * sc.evaluate(xi) + gamma);
        assert!(t_gp.evaluate(xi) * zh.evaluate(xi) - gp_v_xi == zero);
        

        // fp - (fa + beta * sida + gamma) * (fb + beta * sidb + gamma) * (fc + beta * sidc + gamma)
        //let fp_a_xi = fa.evaluate(xi) + beta * sida.evaluate(xi) + gamma;
        //let fp_b_xi = fb.evaluate(xi) + beta * sidb.evaluate(xi) + gamma;
        //let fp_c_xi = fc.evaluate(xi) + beta * sidc.evaluate(xi) + gamma;
        //let fp_xi = &(fp_a_xi * fp_b_xi * fp_c_xi);
        let fp_xi = fp.evaluate(xi);
        // assert!(fp.evaluate(xi) == fp_xi.clone());
        let fp_v = &(fp - (fa + beta * sida + gamma) * (fb + beta * sidb + gamma) * (fc + beta * sidc + gamma));
        assert_eq!(fp.evaluate(xi), fp_xi);
        assert!(fp_v.evaluate(xi) - t_fp.evaluate(xi) * zh.evaluate(xi) == zero);
        assert!(gp_v.evaluate(xi) - t_gp.evaluate(xi) * zh.evaluate(xi) == zero);

        //let fp_v = &(fp - (fa + beta * sida + gamma) * (fb + beta * sidb + gamma) * (fc + beta * sidc + gamma));
        // let gp_v = &(gp - (fa + beta * sa + gamma) * (fb + beta * sb + gamma) * (fc + beta * sc + gamma));

        let fp_v_xi = fp_xi - (a_xi + beta * sida.evaluate(xi) + gamma) * (b_xi + beta * sidb.evaluate(xi) + gamma) * (c_xi + beta * sidc.evaluate(xi) + gamma);
        let gp_v_xi = gp_xi - (a_xi + beta * sa.evaluate(xi) + gamma) * (b_xi + beta * sb.evaluate(xi) + gamma) * (c_xi + beta * sc.evaluate(xi) + gamma);
        let gp_v_xi2 = gp_xi - gp_v.evaluate(xi);
        // assert_eq!(gp_v_xi, gp_v_xi2);
        assert_eq!(fp_v.evaluate(xi), fp_v_xi);
        // assert_eq!(gp_v.evaluate(xi), gp_v_xi);

        let f_gc_xi = f_gc.evaluate(xi);
        let f_cc1_xi = f_cc1.evaluate(xi);
        let f_cc2_xi = f_cc2.evaluate(xi);
        let t_xi = t.evaluate(xi);
        let zh_xi = zh.evaluate(xi);

        // assert!(f_gc_eval + f_cc1_eval * alpha + alpha.pow(2) * f_cc2_eval + alpha.pow(3) * fp_v_eval + alpha.pow(4) * gp_v_eval - t_eval * zh_eval == zero);

        //&((z * fp) - (gp * zw));
        
        for i in 0..self.n*10 {
            let xi = &self.w(i);
            let fp_a_xi = fa.evaluate(xi) + beta * sida.evaluate(xi) + gamma;
            let fp_b_xi = fb.evaluate(xi) + beta * sidb.evaluate(xi) + gamma;
            let fp_c_xi = fc.evaluate(xi) + beta * sidc.evaluate(xi) + gamma;
            let fp_xi = fp_a_xi * fp_b_xi * fp_c_xi;
            let expcc2 = z.evaluate(xi) * fp_xi - gp.evaluate(xi) * zw.evaluate(xi);
            //assert!(f_cc1.evaluate(xi) + alpha * expcc2- t_cc_xi * zh.evaluate(xi) == zero);
            let j = &Scalar { val: i.into() };
            let lol = (a_xi + beta * sa.evaluate(xi) + gamma) * (b_xi + beta * sb.evaluate(xi) + gamma) * (c_xi + beta * sc.evaluate(xi) + gamma);
            println!();
            println!("i = {}", i);
            println!(
                "  sa(i) = {},\n  sb(i) = {},\n  sc(i) = {},\n  a(i) = {},\n  sida(i) = {},\n  sidb(i) = {},\n  sidc(i) = {},\n  lol = {}", 
                sa.evaluate(xi),
                sb.evaluate(xi),
                sc.evaluate(xi),
                fa.evaluate(xi),
                sida.evaluate(xi),
                sidb.evaluate(xi),
                sidc.evaluate(xi),
                lol - gp.evaluate(xi),
            );
            //assert_eq!(f_gc.evaluate(xi), zero);
            //assert_eq!(f_cc1.evaluate(xi), zero);
            //assert_eq!(f_cc2.evaluate(xi), zero);
            for i in 0..self.n {
                //assert!(f_cc1.evaluate(&self.w(i)) + alpha * f_cc2.evaluate(&self.w(i)) - t_cc.evaluate(&self.w(i)) * zh.evaluate(&self.w(i)) == zero);
            }
        }

        //assert_eq!(f_cc1.evaluate(xi), zero);
        //assert!(f_gc_xi + alpha * f_cc1_xi + alpha.pow(2) * f_cc2_xi - t_xi * zh_xi == zero);
        // let fp_a_xi = a_xi + beta * sida.evaluate(xi) + gamma;
        // let fp_b_xi = b_xi + beta * sidb.evaluate(xi) + gamma;
        // let fp_c_xi = c_xi + beta * sidc.evaluate(xi) + gamma;
        // let fp_xi = fp_a_xi * fp_b_xi * fp_c_xi;
        // let expcc2 = z.evaluate(xi) * fp_xi - gp.evaluate(xi) * zw.evaluate(xi);
        // assert!(f_cc1.evaluate(xi) + alpha * expcc2- t_cc_xi * zh.evaluate(xi) == zero);
        // for i in 0..self.n {
        //     assert!(f_cc1.evaluate(&self.w(i)) + alpha * f_cc2.evaluate(&self.w(i)) - t_cc.evaluate(&self.w(i)) * zh.evaluate(&self.w(i)) == zero);
        // }
        // println!("zh_xi: {}", zh.evaluate(xi));
        // println!("t_cc_xi: {}", t_cc_xi);
        // println!("alpha: {}", alpha);
        // assert!(fp.evaluate(xi) == (fa.evaluate(xi) + beta * sida.evaluate(xi) + gamma) * (fb.evaluate(xi) + beta * sidb.evaluate(xi) + gamma) * (fc.evaluate(xi) + beta * sidc.evaluate(xi) + gamma));
        // for i in 0..self.n {
        //     assert!(fp.evaluate(&self.w(i)) == (fa.evaluate(&self.w(i)) + beta * sida.evaluate(&self.w(i)) + gamma) * (fb.evaluate(&self.w(i)) + beta * sidb.evaluate(&self.w(i)) + gamma) * (fc.evaluate(&self.w(i)) + beta * sidc.evaluate(&self.w(i)) + gamma));
        // }
        // assert!(f_cc1.evaluate(xi) + alpha * f_cc2.evaluate(xi) - t_cc.evaluate(xi) * zh.evaluate(xi) == zero);
        // for i in 0..self.n {
        //     assert!(f_cc1.evaluate(&self.w(i)) + alpha * f_cc2.evaluate(&self.w(i)) - t_cc.evaluate(&self.w(i)) * zh.evaluate(&self.w(i)) == zero);
        // }

        println!("Plonk: Sending the following to verifier:\n");
        SnarkProof {
            a_xi: *a_xi,
            a_com: *a_com,
            a_pi: a_pi.clone(),
            b_xi: *b_xi,
            b_com: *b_com,
            b_pi: b_pi.clone(),
            c_xi: *c_xi,
            c_com: *c_com,
            c_pi: c_pi.clone(),
            f_gc_pi: f_gc_pi.clone(),
            t_xi: t_xi.clone(),
            t_com: *t_com,
            t_pi: t_pi.clone(),
            z_xi: *z_xi,
            z_com: *z_com,
            z_pi: z_pi.clone(),
            zw_xi: *zw_xi,
            zw_pi: zw_pi.clone(),
            fp_xi: fp_xi,
            fp_pi: fp_pi.clone(),
            fp_com: fp_com.clone(),
            gp_xi: *gp_xi,
            gp_pi: gp_pi.clone(),
            gp_com: gp_com.clone(),
        }
    }

    pub fn verify(&self, proof: &SnarkProof) -> bool {
        println!("Plonk: Verifier is verifying the SNARK proof...\n");
        // convenience definitions
        // --------------------------------------------------------------------

        // `PallasScalar` constants
        let one: &Scalar = &PallasScalar::ONE.into();
        let zero: &Scalar = &PallasScalar::ZERO.into();

        // Round: Precomputation
        // --------------------------------------------------------------------

        // public polynomials
        let [qle, qre, qoe, qme, qce] = &self.public_wire_polynomials();
        let [sa, sb, sc] = &self.copy_constraints();
        let [sida, sidb, sidc] = &self.id_permutation();

        // Round:  Compute challenges
        // --------------------------------------------------------------------

        let mut transcript = Transcript::new(b"protocol");
        transcript.domain_sep();

        let beta = &transcript.challenge_scalar_augment(0, b"beta");
        let gamma = &transcript.challenge_scalar_augment(1, b"gamma");
        // println!("beta: {}", beta);
        // println!("gamma: {}", gamma);
        let xi = &transcript.challenge_scalar(b"xi");
        // println!("xi: {}", xi);
        let alpha = &transcript.challenge_scalar(b"alpha");

        let zh = &(x(self.order) - one);
        let zh_xi = &zh.evaluate(xi);

        // Round: Gate Constraints Check
        // --------------------------------------------------------------------

        let ql_xi = &qle.evaluate(xi);
        let qr_xi = &qre.evaluate(xi);
        let qo_xi = &qoe.evaluate(xi);
        let qm_xi = &qme.evaluate(xi);
        let qc_xi = &qce.evaluate(xi);
        let f_gc_xi = proof.a_xi * ql_xi
            + proof.b_xi * qr_xi
            + proof.c_xi * qo_xi
            + proof.a_xi * proof.b_xi * qm_xi
            + qc_xi;
        assert!(f_gc_xi != *zero);
        assert!(check(&proof.a_com, xi, &proof.a_xi, &proof.a_pi));
        assert!(check(&proof.b_com, xi, &proof.b_xi, &proof.b_pi));
        assert!(check(&proof.c_com, xi, &proof.c_xi, &proof.c_pi));

        // Round: Copy Constraints Check (Base Case)
        // --------------------------------------------------------------------

        let l1 = &self.l(0);
        let l1_xi = &l1.evaluate(xi);
        let f_cc1_xi = &((proof.z_xi - one) * l1_xi);
        // println!("f_cc1_xi: {}", f_cc1_xi);

        // Round: Copy Constraints Check (Inductive Case)
        // --------------------------------------------------------------------

        // println!("a_xi: {}", proof.a_xi);
        // println!("b_xi: {}", proof.b_xi);
        // println!("c_xi: {}", proof.c_xi);
        // println!("sida_xi: {}", sida.evaluate(xi));
        // println!("sidb_xi: {}", sidb.evaluate(xi));
        // println!("sidc_xi: {}", sidc.evaluate(xi));
        // let fp_a_xi = proof.a_xi + beta * sida.evaluate(xi) + gamma;
        // let fp_b_xi = proof.b_xi + beta * sidb.evaluate(xi) + gamma;
        // let fp_c_xi = proof.c_xi + beta * sidc.evaluate(xi) + gamma;
        // let fp_xi = fp_a_xi * fp_b_xi * fp_c_xi;
        // let gp_a_xi = proof.a_xi + beta * sa.evaluate(xi) + gamma;
        // let gp_b_xi = proof.b_xi + beta * sb.evaluate(xi) + gamma;
        // let gp_c_xi = proof.c_xi + beta * sc.evaluate(xi) + gamma;
        // let gp_xi = gp_a_xi * gp_b_xi * gp_c_xi;
        let fp_xi = (proof.a_xi + beta * sida.evaluate(xi) + gamma) * (proof.b_xi + beta * sidb.evaluate(xi) + gamma) * (proof.c_xi + beta * sidc.evaluate(xi) + gamma);
        let gp_xi = (proof.a_xi + beta * sa.evaluate(xi) + gamma) * (proof.b_xi + beta * sb.evaluate(xi) + gamma) * (proof.c_xi + beta * sc.evaluate(xi) + gamma);
        let f_cc2_xi = &(proof.z_xi * proof.fp_xi - proof.gp_xi * proof.zw_xi);
        // println!(" L R : {} {} ", proof.fp_xi,  (proof.a_xi + beta * sida.evaluate(xi) + gamma) * (proof.b_xi + beta * sidb.evaluate(xi) + gamma) * (proof.c_xi + beta * sidc.evaluate(xi) + gamma);
        let fp_v_xi = proof.fp_xi - fp_xi;
        let gp_v_xi = proof.gp_xi - gp_xi;
        // println!("f_cc2_xi: {}", f_cc2_xi);
        // println!("z_xi: {}", proof.z_xi);
        // println!("fp_xi: {}", fp_xi);
        // println!("gp_xi: {}", gp_xi);
        // println!("zw_xi: {}", proof.zw_xi);

        // assert!(f_cc1.evaluate(xi) + alpha * f_cc2.evaluate(xi) - t_cc_xi * zh.evaluate(xi) == zero);
        println!("alpha: {}", alpha);
        println!("fp_v_xi: {}", fp_v_xi);
        println!("gp_v_xi: {}", gp_v_xi);
        println!("f_gc_xi: {}", f_gc_xi);
        println!("f_cc1_xi: {}", f_cc1_xi);
        println!("f_cc2_xi: {}", f_cc2_xi);
        println!("t_xi: {}", proof.t_xi);
        println!("zh_xi: {}", zh_xi);
        //assert!((f_gc_xi + f_cc1_xi * alpha + alpha.pow(2) * f_cc2_xi) - proof.t_xi * zh_xi == *zero);
        assert!(check(&proof.t_com, xi, &proof.t_xi, &proof.t_pi));
        assert!(check(&proof.z_com, xi, &proof.z_xi, &proof.z_pi));
        assert!(check(
            &proof.z_com,
            &(self.w1 * xi),
            &proof.zw_xi,
            &proof.zw_pi
        ));

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zh_evaluation() {
        let order = 8;
        let one: Scalar = PallasScalar::ONE.into();
        let zero: Scalar = PallasScalar::ZERO.into();
        let zh_poly: &Poly = &(x(order) - one);
        let omega: Scalar = get_omega(order).into();
        for i in 0..order {
            let point = omega.pow(i);
            assert_eq!(zh_poly.evaluate(&point), zero);
        }
    }
}

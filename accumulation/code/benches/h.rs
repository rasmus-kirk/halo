#![allow(dead_code)]

use ark_poly::Polynomial;
use ark_std::{test_rng, UniformRand};
use criterion::Criterion;

use halo_accumulation::{
    acc::AccumulatedHPolys, group::PallasScalar, pcdl::HPoly
};
use seq_macro::seq;

const K: usize = 100;

macro_rules! define_h_benches {
    ($exp:literal) => {
        paste::paste! {
            pub fn [<h_get_poly_ $exp>](c: &mut Criterion) {
                let rng = &mut test_rng();
                let h = HPoly::rand(rng, $exp);
                c.bench_function(concat!("h_get_poly_", stringify!($exp)), |b| b.iter(|| h.get_poly()));
            }
            pub fn [<h_eval_ $exp>](c: &mut Criterion) {
                let rng = &mut test_rng();
                let h = HPoly::rand(rng, $exp);
                let z = PallasScalar::rand(rng);
                c.bench_function(concat!("h_eval_", stringify!($exp)), |b| b.iter(|| h.eval(&z)));
            }
            pub fn [<h_eval_naive_ $exp>](c: &mut Criterion) {
                let rng = &mut test_rng();
                let h = HPoly::rand(rng, $exp);
                let h_poly = h.get_poly();
                let z = PallasScalar::rand(rng);
                //assert_eq!(h_poly.degree(), $exp - 1);
                c.bench_function(concat!("h_eval_naive_", stringify!($exp)), |b| b.iter(|| h_poly.evaluate(&z)));
            }
            pub fn [<h_hs_get_poly_ $exp>](c: &mut Criterion) {
                let rng = &mut test_rng();
                let mut hs = AccumulatedHPolys::with_capacity(K);
                hs.set_alpha(PallasScalar::rand(rng));
                for _ in 0..K {
                    hs.hs.push(HPoly::rand(rng, $exp))
                }

                c.bench_function(concat!("h_hs_get_poly_", stringify!($exp)), |b| b.iter(|| hs.get_poly()));
            }
            pub fn [<h_hs_eval_ $exp>](c: &mut Criterion) {
                let rng = &mut test_rng();
                let mut hs = AccumulatedHPolys::with_capacity(K);
                hs.set_alpha(PallasScalar::rand(rng));
                for _ in 0..K {
                    hs.hs.push(HPoly::rand(rng, $exp))
                }

                let z = PallasScalar::rand(rng);
                c.bench_function(concat!("h_hs_eval_", stringify!($exp)), |b| b.iter(|| hs.eval(&z)));
            }
        }
    };
}

seq!(K in 10..21 {
    define_h_benches!(K);
});

#![allow(dead_code)]

use ark_ff::UniformRand;
use ark_poly::DenseUVPolynomial;
use ark_std::test_rng;
use bincode::config::standard;
use criterion::Criterion;

use halo_accumulation::{
    group::*,
    pcdl::{self, commit, Instance},
    wrappers::*,
};
use seq_macro::seq;

const PRE: &[u8] = include_bytes!("precompute/qs.bin");

/// Helper function: Gets precomputed linear-time computation dummy values.
fn get_cheap_linears(n: usize) -> [Instance; 1] {
    let (val, _): (Vec<(usize, WrappedInstance, WrappedAccumulator)>, _) =
        bincode::decode_from_slice(&PRE, standard()).unwrap();
    let q_acc = val.into_iter().find(|x| x.0 == n).unwrap();
    [q_acc.1.into()]
}

macro_rules! define_pcdl_benches {
    ($exp:literal) => {
        paste::paste! {
            pub fn [<pcdl_open_ $exp>](c: &mut Criterion) {
                let mut rng = &mut test_rng();
                let n = 2usize.pow($exp);
                let d = n - 1;
                let pp = &pcdl::setup(n);

                let w = Some(PallasScalar::rand(rng));
                let p = PallasPoly::rand(d, rng);
                let z = &PallasScalar::rand(rng);
                let comm = commit(pp, &p, d, w.as_ref());

                c.bench_function(concat!("pcdl_open_", stringify!($exp)), |b| b.iter(|| pcdl::open(&mut rng, pp, p.clone(), comm, d, z, w.as_ref())));
            }
            pub fn [<pcdl_commit_ $exp>](c: &mut Criterion) {
                let rng = &mut test_rng();
                let n = 2usize.pow($exp);
                let d = n - 1;
                let pp = &pcdl::setup(n);

                let w = Some(PallasScalar::rand(rng));
                let p = PallasPoly::rand(d, rng);

                c.bench_function(concat!("pcdl_commit_", stringify!($exp)), |b| b.iter(|| commit(pp, &p, d, w.as_ref())));
            }
            pub fn [<pcdl_check_ $exp>](c: &mut Criterion) {
                let n = 2usize.pow($exp);
                let pp = &pcdl::setup(n);
                let qs = get_cheap_linears(n);

                c.bench_function(concat!("pcdl_check_", stringify!($exp)), |b| b.iter(|| qs[0].check(pp).unwrap()));
            }
            pub fn [<pcdl_succinct_check_ $exp>](c: &mut Criterion) {
                let n = 2usize.pow($exp);
                let pp = &pcdl::setup(n);
                let qs = get_cheap_linears(n);

                c.bench_function(concat!("pcdl_succinct_check_", stringify!($exp)), |b| b.iter(|| qs[0].succinct_check(pp).unwrap()));
            }
        }
    };
}

seq!(K in 5..21 {
    define_pcdl_benches!(K);
});

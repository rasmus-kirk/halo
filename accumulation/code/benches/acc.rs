#![allow(dead_code)]

use ark_std::test_rng;
use bincode::config::standard;
use criterion::Criterion;

use halo_accumulation::{
    acc::{self, Accumulator},
    pcdl::Instance,
    pp::PublicParams,
    wrappers::*,
};
use seq_macro::seq;

const PRE: &[u8] = include_bytes!("precompute/qs.bin");

/// Helper function: Gets precomputed linear-time computation dummy values.
fn get_cheap_linears(n: usize) -> ([Instance; 1], Accumulator) {
    let (val, _): (Vec<(usize, WrappedInstance, WrappedAccumulator)>, _) =
        bincode::decode_from_slice(&PRE, standard()).unwrap();
    let q_acc = val.into_iter().find(|x| x.0 == n).unwrap();
    ([q_acc.1.into()], q_acc.2.into())
}

pub fn public_parameters(c: &mut Criterion) {
    let n = 2usize.pow(20);
    c.bench_function("public_parameters", |b| b.iter(|| PublicParams::new(n)));
}

macro_rules! define_acc_benches {
    ($exp:literal) => {
        paste::paste! {
            pub fn [<acc_common_subroutine_ $exp>](c: &mut Criterion) {
                let n = 2usize.pow($exp);
                let pp = &acc::setup(n);
                let (qs, acc) = get_cheap_linears(n);
                c.bench_function(concat!("acc_common_subroutine_", stringify!($exp)), |b| b.iter(|| acc::common_subroutine(&pp, &qs, &acc.pi_V)));
            }
            pub fn [<acc_prover_ $exp>](c: &mut Criterion) {
                let mut rng = test_rng();
                let n = 2usize.pow($exp);
                let pp = &acc::setup(n);
                let (qs, _) = get_cheap_linears(n);
                c.bench_function(concat!("acc_prover_", stringify!($exp)), |b| b.iter(|| acc::prover(&mut rng, pp, &qs)));
            }
            pub fn [<acc_decider_ $exp>](c: &mut Criterion) {
                let n = 2usize.pow($exp);
                let pp = &acc::setup(n);
                let (_, acc) = get_cheap_linears(n);
                c.bench_function(concat!("acc_decider_", stringify!($exp)), |b| b.iter(|| acc::decider(pp, acc.clone())));
            }
            pub fn [<acc_verifier_ $exp>](c: &mut Criterion) {
                let n = 2usize.pow($exp);
                let pp = &acc::setup(n);
                let (qs, acc) = get_cheap_linears(n);
                c.bench_function(concat!("acc_verifier_", stringify!($exp)), |b| {
                    b.iter(|| acc::verifier(pp, &qs, acc.clone()))
                });
            }
        }
    };
}

seq!(K in 5..21 {
    define_acc_benches!(K);
});

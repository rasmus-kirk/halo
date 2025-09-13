#![feature(associated_type_defaults)]
#![allow(non_snake_case)]
#![allow(uncommon_codepoints)]
#![allow(non_ascii_idents)]
#![allow(confusable_idents)]

use std::time::Instant;

use anyhow::Result;
use halo_group::{Fq, PallasConfig, ark_ff::UniformRand, ark_std::rand::thread_rng};
use halo_schnorr::{PublicKey, SchnorrSignature, generate_keypair};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    circuit::PlonkCircuit,
    frontend::{
        Call,
        ivc::{CallIVCState, IVC_FP_CIRCUIT, IVC_FQ_CIRCUIT, IVCState, WireIVCState},
        plonk::{CallPlonk, WirePlonkCircuit, WirePlonkProof, WirePlonkPublicInputs},
        primitives::WireAffine,
    },
    plonk::PlonkProof,
};

mod utils;

pub mod circuit;
pub mod frontend;
pub mod plonk;

fn main() -> Result<()> {
    env_logger::init();
    let rng = &mut thread_rng();

    let (sk, pk) = generate_keypair::<PallasConfig>();

    let N = 40000;
    let now = Instant::now();
    let msg = [Fq::rand(rng); 10];
    let signature = sk.sign(&msg);
    (0..N).into_par_iter().for_each(|_| {
        let _ = pk.verify(&msg, signature.clone());
    });
    println!(
        "Verified {:?} signatures per second",
        (N as f32) / now.elapsed().as_secs_f32()
    );

    // IVCState::print_ivc_circuit()?;
    // Init 0
    let now = Instant::now();
    let ivc_state_0 = IVCState::init();
    println!("IVC Init took {} s", now.elapsed().as_secs_f32());

    let now = Instant::now();
    ivc_state_0.verify()?;
    println!("(0) IVC Verifier took {} s", now.elapsed().as_secs_f32());

    // 0 -> 1
    let now = Instant::now();
    let ivc_state_1 = ivc_state_0.prove()?;
    println!("(0 -> 1) IVC Prover took {} s", now.elapsed().as_secs_f32());

    let now = Instant::now();
    ivc_state_1.verify()?;
    println!("(1) IVC Verifier took {} s", now.elapsed().as_secs_f32());

    // 1 -> 2
    let now = Instant::now();
    let ivc_state_2 = ivc_state_1.prove()?;
    println!("(1 -> 2) IVC Prover took {} s", now.elapsed().as_secs_f32());

    let now = Instant::now();
    ivc_state_2.verify()?;
    println!("(2) IVC Verifier took {} s", now.elapsed().as_secs_f32());

    // 2 -> 3
    let now = Instant::now();
    let ivc_state_3 = ivc_state_2.prove()?;
    println!("(2 -> 3) IVC Prover took {} s", now.elapsed().as_secs_f32());

    let now = Instant::now();
    ivc_state_3.verify()?;
    println!("(3) IVC Verifier took {} s", now.elapsed().as_secs_f32());

    Ok(())
}

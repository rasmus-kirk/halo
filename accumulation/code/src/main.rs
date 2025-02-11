#![allow(non_snake_case)]

use std::{
    env,
    fmt::Display,
    fs::{self, create_dir, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Arc,
    time::{Duration, SystemTime},
};

use acc::Accumulator;
use anyhow::Result;
use archive::std_config;
use pcdl::Instance;
use pp::PublicParams;
use tokio::task::JoinSet;
use wrappers::{WrappedAccumulator, WrappedInstance};

pub mod acc;
mod archive;
pub mod consts;
pub mod group;
pub mod pcdl;
pub mod pedersen;
mod pp;
pub mod wrappers;

// TODO: Maybe remove this
// -------------------- Benchmarking Struct --------------------

struct Benchmark {
    time: Duration,
    n: usize,
}

impl Display for Benchmark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let secs = self.time.as_secs();
        let millis = self.time.as_millis() % 1000;
        write!(f, "[n={}] Ran in {}:{} s", self.n, secs, millis)
    }
}

impl Benchmark {
    fn new(time: Duration, n: usize) -> Self {
        Self { time, n }
    }
}

// -------------------- Benchmarking Functions --------------------

async fn gen(
    pp: Arc<PublicParams>,
    n: usize,
) -> Result<(usize, WrappedInstance, WrappedAccumulator)> {
    let q = gen_q(pp.clone(), n).await?;
    let acc = gen_acc(pp, q.clone()).await?;

    let wq: WrappedInstance = q.clone().into();
    let wacc: WrappedAccumulator = acc.into();
    Ok((n, wq, wacc))
}

async fn gen_q(pp: Arc<PublicParams>, n: usize) -> Result<Instance> {
    let rng = &mut rand::thread_rng();

    let now = SystemTime::now();
    let q = Instance::rand(rng, &(*pp), n);
    let elapsed = now.elapsed()?;

    let time = format!("{}:{} s", elapsed.as_secs(), elapsed.as_millis() % 1000);
    println!("[q {}]: Finished in {}", n, time);

    Ok(q)
}

async fn gen_acc(pp: Arc<PublicParams>, q: Instance) -> Result<Accumulator> {
    let rng = &mut rand::thread_rng();
    let n = q.d + 1;

    let now = SystemTime::now();
    let acc = acc::prover(rng, &(*pp), &[q])?;
    let elapsed = now.elapsed()?;

    let time = format!("{}:{} s", elapsed.as_secs(), elapsed.as_millis() % 1000);
    println!("[acc {}]: Finished acc in {}", n, time);

    Ok(acc)
}

async fn test_succinct_check(pp: Arc<PublicParams>, q: Instance, n: usize) -> Result<Benchmark> {
    let now = SystemTime::now();
    let _ = q.succinct_check(&pp)?;
    let elapsed = now.elapsed()?;

    let benchmark = Benchmark::new(elapsed, n);

    let time = format!("{}:{} s", elapsed.as_secs(), elapsed.as_millis() % 1000);

    let n_f = n as f64;
    let t_f = elapsed.as_millis() as f64;
    let score = t_f.powi(2) / n_f;
    println!("[sc - {}]: Finished in {} (score: {})", n, time, score);

    Ok(benchmark)
}

async fn test_acc_ver(pp: Arc<PublicParams>, q: Instance, acc: Accumulator) -> Result<Benchmark> {
    let n = acc.q.d + 1;

    let now = SystemTime::now();
    acc.verifier(&pp, &[q])?;
    let elapsed = now.elapsed()?;

    let benchmark = Benchmark::new(elapsed, n);

    let time = format!("{}:{} s", elapsed.as_secs(), elapsed.as_millis() % 1000);

    let n_f = n as f64;
    let t_f = elapsed.as_millis() as f64;
    let score = t_f.powi(2) / n_f;
    println!("[acc - {}]: Finished in {} (score: {})", n, time, score);

    Ok(benchmark)
}

// TODO: Benchmark pcdl::open, and acc::prover properly. Idea branch on provers/verifiers otherwise error.
/// A hacky script to do some rough benchmarks. We parallelize what we can to bring down the time as much as possible, this hurts the benchmarks, but they take so long that it's the only feasible option.
/// Run `cargo run -- /path/to/gen/dir gen` to generate the benchmarking accumulators and instances
/// Run `cargo run -- /path/to/gen/dir` to benchmark the cached generated values
#[tokio::main]
async fn main() -> Result<()> {
    // Handle cmd inputs
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("No path specified!");
    let is_gen = match args.get(2) {
        Some(s) => s == "gen",
        None => false,
    };
    // Handle destination dir
    // TODO: Create a file for q and acc
    let dest_path = PathBuf::from(path).join("qs");
    if !dest_path.exists() {
        println!("cargo:warning=creating {:?}", dest_path);
        create_dir(&dest_path)?;
    }
    let q_path = dest_path.join("qs.bin");

    let max = 20;
    let n = 2usize.pow(max);
    let pp = Arc::new(pp::PublicParams::new(n));

    // Branch on `gen` argument
    if is_gen {
        let mut set = JoinSet::new();
        for i in 5..(max + 1) {
            let pp_clone = Arc::clone(&pp);
            set.spawn(gen(pp_clone, 2usize.pow(i)));
        }

        let res: Result<Vec<(usize, WrappedInstance, WrappedAccumulator)>> =
            set.join_all().await.into_iter().collect();

        let bytes = bincode::encode_to_vec(res?, std_config())?;

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(q_path)?;

        Write::write_all(&mut file, &bytes)?;
    } else {
        let bytes = fs::read(q_path)?;
        let (val, _): (Vec<(usize, WrappedInstance, WrappedAccumulator)>, _) =
            bincode::decode_from_slice(&bytes, std_config())?;

        let mut set = JoinSet::new();
        for (n, q, acc) in val.into_iter() {
            let q: Instance = q.into();
            set.spawn(test_succinct_check(pp.clone(), q.clone(), n));
            set.spawn(test_acc_ver(pp.clone(), q, acc.into()));
        }

        let benches: Result<Vec<Benchmark>> = set.join_all().await.into_iter().collect();

        for bench in benches? {
            println!("{}", bench);
        }
    }

    Ok(())
}

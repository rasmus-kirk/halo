use std::time::Duration;

use criterion::{BenchmarkId, Criterion};
use halo_group::{
    Domain, Evals, PallasConfig, PastaConfig, Poly,
    ark_poly::{DenseUVPolynomial, EvaluationDomain, Polynomial},
    ark_std::test_rng,
};

const MIN: usize = 10;
const MAX: usize = 20;
const WARMUP: Duration = Duration::from_millis(1000);

fn smart_mul<P: PastaConfig>(x: &Poly<P>, y: &Poly<P>) -> Poly<P> {
    let new_degree = x.degree() + y.degree();
    let domain = Domain::<P>::new(new_degree + 1).unwrap();
    let x_evals = Evals::<P>::from_poly_ref(&x, domain);
    let y_evals = Evals::<P>::from_poly_ref(&y, domain);
    let out = x_evals * y_evals;
    out.interpolate()
}

pub fn poly_naive_mul(c: &mut Criterion) {
    let rng = &mut test_rng();

    let mut group = c.benchmark_group("poly_naive_mul");
    for size in MIN..MAX + 1 {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                let n = 2usize.pow(size as u32);
                let x = Poly::<PallasConfig>::rand(n - 1, rng);
                let y = Poly::<PallasConfig>::rand(n - 1, rng);

                b.iter(|| &x * &y);
            },
        );
    }
    group.finish();
}

pub fn poly_evals_mul(c: &mut Criterion) {
    let rng = &mut test_rng();

    let mut group = c.benchmark_group("poly_evals_mul");
    for size in MIN..MAX + 1 {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                let n = 2usize.pow(size as u32);
                let x = Poly::<PallasConfig>::rand(n - 1, rng);
                let y = Poly::<PallasConfig>::rand(n - 1, rng);

                b.iter(|| smart_mul::<PallasConfig>(&x, &y));
            },
        );
    }
    group.finish();
}

pub fn poly_evals_mul_raw(c: &mut Criterion) {
    let rng = &mut test_rng();

    let mut group = c.benchmark_group("poly_evals_mul_raw");
    for size in MIN..MAX + 1 {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                let n = 2usize.pow(size as u32);
                let x = Poly::<PallasConfig>::rand(n - 1, rng);
                let y = Poly::<PallasConfig>::rand(n - 1, rng);
                let new_degree = x.degree() + y.degree();
                let domain = Domain::<PallasConfig>::new(new_degree + 1).unwrap();
                let x_evals = Evals::<PallasConfig>::from_poly_ref(&x, domain);
                let y_evals = Evals::<PallasConfig>::from_poly_ref(&y, domain);

                b.iter(|| &x_evals * &y_evals);
            },
        );
    }
    group.finish();
}

pub fn poly_evals_fft(c: &mut Criterion) {
    let rng = &mut test_rng();

    let mut group = c.benchmark_group("poly_evals_fft");
    for size in MIN..MAX + 1 {
        group.warm_up_time(WARMUP).bench_with_input(
            BenchmarkId::from_parameter(size),
            &size,
            |b, &size| {
                let n = 2usize.pow(size as u32);
                let x = Poly::<PallasConfig>::rand(n - 1, rng);
                let y = Poly::<PallasConfig>::rand(n - 1, rng);
                let new_degree = x.degree() + y.degree();
                let domain = Domain::<PallasConfig>::new(new_degree + 1).unwrap();
                let x_evals = Evals::<PallasConfig>::from_poly_ref(&x, domain);
                let y_evals = Evals::<PallasConfig>::from_poly_ref(&y, domain);
                let z = &x_evals * &y_evals;

                b.iter(|| z.interpolate_by_ref());
            },
        );
    }
    group.finish();
}

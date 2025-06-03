use super::{misc::batch_op, Poly, Scalar};

use ark_ec::short_weierstrass::SWCurveConfig;
use ark_poly::{
    univariate::DensePolynomial, EvaluationDomain, Evaluations, GeneralEvaluationDomain,
};

use educe::Educe;
use std::ops::{Add, Index, Mul, Sub};

#[derive(Educe)]
#[educe(Default, Debug, Clone, PartialEq, Eq)]
pub struct Evals<P: SWCurveConfig>(pub Vec<Scalar<P>>);

impl<P: SWCurveConfig> Evals<P> {
    pub fn new(evals: Vec<Scalar<P>>) -> Self {
        Self(evals)
    }

    /// new with `shift_right`
    pub fn new_sr(evals: Vec<Scalar<P>>) -> Self {
        Self::new(evals).shift_right()
    }

    /// ∀X ∈ [n]: f(ωᶦ) = v
    pub fn new_const(v: Scalar<P>, n: usize) -> Self {
        Self(vec![v; n])
    }

    pub fn fft_sp(self) -> DensePolynomial<Scalar<P>> {
        let domain = self.domain();
        Evaluations::from_vec_and_domain(self.0, domain).interpolate()
    }
    /// interpolate consuming self
    pub fn fft_s(self) -> Poly<P> {
        Poly::new_e(Self::fft_sp(self.clone()), self)
    }

    /// interpolate borrowing self
    pub fn fft(&self) -> Poly<P> {
        Self::fft_s(self.clone())
    }

    pub fn domain(&self) -> GeneralEvaluationDomain<Scalar<P>> {
        GeneralEvaluationDomain::<Scalar<P>>::new(self.0.len()).unwrap()
    }

    pub fn last(&self) -> &Scalar<P> {
        self.0.last().unwrap()
    }

    /// ∀X ∈ H₀: g(ωX) = f(X)
    pub fn shift_right(self) -> Self {
        let mut evals = self.0;
        let last = evals.pop().unwrap();
        evals.insert(0, last);
        Evals::<P>::new(evals)
    }

    /// ∀X ∈ H₀: g(X) = f(ωX)
    pub fn shift_left(self) -> Self {
        let mut evals = self.0;
        let evals_first = evals.remove(0);
        evals.push(evals_first);
        Evals::<P>::new(evals)
    }

    /// v => ∀X ∈ [n]: f(ωᶦ) = v
    pub fn deg0(n: u64) -> impl Fn(Scalar<P>) -> Self {
        move |v| Self::new_const(v, n as usize)
    }
}

pub fn batch_fft<'a, P: SWCurveConfig, I>(ps: I) -> Vec<Poly<P>>
where
    I: IntoIterator<Item = &'a Evals<P>>,
{
    batch_op(ps, |f| f.fft())
}

// Mul ------------------------------------------------------------------------

impl<P: SWCurveConfig> Mul for Evals<P> {
    type Output = Evals<P>;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl<P: SWCurveConfig> Mul<Evals<P>> for &Evals<P> {
    type Output = Evals<P>;

    fn mul(self, rhs: Evals<P>) -> Self::Output {
        self * &rhs
    }
}

impl<P: SWCurveConfig> Mul<&Evals<P>> for Evals<P> {
    type Output = Evals<P>;

    fn mul(self, rhs: &Evals<P>) -> Self::Output {
        &self * rhs
    }
}

impl<P: SWCurveConfig> Mul for &Evals<P> {
    type Output = Evals<P>;

    fn mul(self, rhs: &Evals<P>) -> Self::Output {
        Evals(
            self.domain()
                .mul_polynomials_in_evaluation_domain(&self.0, &rhs.0),
        )
    }
}

// Add ------------------------------------------------------------------------

impl<P: SWCurveConfig> Add for Evals<P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl<P: SWCurveConfig> Add<&Evals<P>> for Evals<P> {
    type Output = Self;

    fn add(self, rhs: &Evals<P>) -> Self::Output {
        &self + rhs
    }
}

impl<P: SWCurveConfig> Add<Evals<P>> for &Evals<P> {
    type Output = Evals<P>;

    fn add(self, rhs: Evals<P>) -> Self::Output {
        self + &rhs
    }
}

impl<P: SWCurveConfig> Add for &Evals<P> {
    type Output = Evals<P>;

    fn add(self, rhs: Self) -> Self::Output {
        if rhs.0.is_empty() {
            return Evals::new(self.0.clone());
        }
        if self.0.is_empty() {
            return Evals::new(rhs.0.clone());
        }
        Evals(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(&a, &b)| a + b)
                .collect(),
        )
    }
}

// Sub ------------------------------------------------------------------------

impl<P: SWCurveConfig> Sub for Evals<P> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        &self - &rhs
    }
}

impl<P: SWCurveConfig> Sub<&Evals<P>> for Evals<P> {
    type Output = Self;

    fn sub(self, rhs: &Evals<P>) -> Self::Output {
        &self - rhs
    }
}
impl<P: SWCurveConfig> Sub<Evals<P>> for &Evals<P> {
    type Output = Evals<P>;

    fn sub(self, rhs: Evals<P>) -> Self::Output {
        self - &rhs
    }
}

impl<P: SWCurveConfig> Sub for &Evals<P> {
    type Output = Evals<P>;

    fn sub(self, rhs: Self) -> Self::Output {
        if rhs.0.is_empty() {
            return Evals(self.0.clone());
        }
        if self.0.is_empty() {
            return Evals(rhs.0.iter().map(|&x| -x).collect());
        }
        Evals(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(&a, &b)| a - b)
                .collect(),
        )
    }
}

// Index -------------------------------------------------------------

impl<P: SWCurveConfig> Index<usize> for Evals<P> {
    type Output = Scalar<P>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

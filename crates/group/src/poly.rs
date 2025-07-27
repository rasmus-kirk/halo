use ark_poly::{
    univariate::DensePolynomial, EvaluationDomain, Evaluations, Radix2EvaluationDomain,
};

use crate::{PastaConfig, Scalar};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Evals<P: PastaConfig> {
    pub vec: Vec<Scalar<P>>,
    pub n: usize,
}

impl<P: PastaConfig> Evals<P> {
    pub fn new(evals: Vec<Scalar<P>>, n: usize) -> Self {
        Self { vec: evals, n }
    }

    pub fn omega(&self) -> Scalar<P> {
        assert_eq!(self.vec.len(), self.n);
        self.domain().elements().next().unwrap()
    }

    pub fn fft(mut self) -> DensePolynomial<Scalar<P>> {
        assert_eq!(self.vec.len(), self.n);
        let domain = self.domain();
        self.shift_right();
        Evaluations::from_vec_and_domain(self.vec, domain).interpolate()
    }

    pub fn domain(&self) -> Radix2EvaluationDomain<Scalar<P>> {
        assert_eq!(self.vec.len(), self.n);
        Radix2EvaluationDomain::<Scalar<P>>::new(self.n).unwrap()
    }

    /// ∀X ∈ H₀: g(ωX) = f(X)
    pub fn shift_right(&mut self) {
        let last = self.vec.pop().unwrap();
        self.vec.insert(0, last);
    }

    /// ∀X ∈ H₀: g(X) = f(ωX)
    pub fn shift_left(&mut self) {
        let evals_first = self.vec.remove(0);
        self.vec.push(evals_first);
    }
}

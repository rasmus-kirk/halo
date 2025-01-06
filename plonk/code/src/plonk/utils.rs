use ark_ff::{AdditiveGroup, FftField, Field};
use ark_poly::{
    DenseUVPolynomial, EvaluationDomain, Evaluations, GeneralEvaluationDomain, Polynomial,
};
use halo_accumulation::{
    group::{PallasPoint, PallasPoly, PallasScalar},
    pcdl::{self, EvalProof},
};
use log::debug;
use rand::{
    distributions::{Distribution, Standard},
    rngs::ThreadRng,
};
use std::{
    fmt,
    ops::{Add, Div, Mul, Neg, Sub},
};

pub type Evals<const N: usize> = [Vec<PallasScalar>; N];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommitData {
    pub d: usize,
    pub pt: PallasPoint,
}

impl fmt::Display for CommitData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CommitData {{ d: {}, pt: {} }}", self.d, self.pt)
    }
}

/// Get the root of unity from ark_pallas
pub fn get_omega(order: u64) -> Scalar {
    PallasScalar::get_root_of_unity(order).unwrap().into()
}

/// Get the evaluation domain based on the order of the root of unity
pub fn domain(order: u64) -> GeneralEvaluationDomain<PallasScalar> {
    GeneralEvaluationDomain::<PallasScalar>::new(order as usize).unwrap()
}

/// Interpolate the points to a polynomial
pub fn interpolate(order: u64, points: &[PallasScalar]) -> PallasPoly {
    let domain = domain(order);
    let evaluations = Evaluations::from_vec_and_domain(points.to_owned(), domain);
    evaluations.interpolate()
}

/// p(x) = x^n
pub fn x(n: u64) -> Poly {
    let mut points = vec![PallasScalar::ZERO; n as usize];
    points.push(PallasScalar::ONE);
    PallasPoly::from_coefficients_vec(points).into()
}

/// p(x) = a
pub fn p(a: &PallasScalar) -> Poly {
    PallasPoly::from_coefficients_vec(vec![*a]).into()
}

/// `p` variant for values instead of references
pub fn p_(a: PallasScalar) -> Poly {
    PallasPoly::from_coefficients_vec(vec![a]).into()
}

pub fn pu(a: &u64) -> Poly {
    PallasPoly::from_coefficients_vec(vec![PallasScalar::from(*a)]).into()
}

/// Succint PCDL commit call
pub fn commit(poly: &Poly) -> CommitData {
    let mut d = poly.degree().next_power_of_two() - 1;
    if poly.degree() >= d {
        d += 2;
        d = d.next_power_of_two() - 1;
    }
    debug!("poly.degree(): {:?}", poly.degree());
    debug!("d: {:?}", d);
    CommitData {
        d,
        pt: pcdl::commit(&poly.poly, d, None),
    }
}

pub fn open(rng: &mut ThreadRng, p: &Poly, commit: &CommitData, z: &Scalar) -> EvalProof {
    pcdl::open(rng, p.poly.clone(), commit.pt, commit.d, &z.val, None)
}

pub fn check(commit: &CommitData, z: &Scalar, v: &Scalar, proof: &EvalProof) -> bool {
    pcdl::check(&commit.pt, commit.d, &z.val, &v.val, proof.clone()).is_ok()
}

// Poly ops ------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Poly {
    pub poly: PallasPoly,
}

impl From<PallasPoly> for Poly {
    fn from(poly: PallasPoly) -> Self {
        Poly { poly }
    }
}

impl From<Poly> for PallasPoly {
    fn from(poly: Poly) -> Self {
        poly.poly
    }
}

impl Poly {
    pub fn evaluate(&self, x: &Scalar) -> Scalar {
        self.poly.evaluate(&x.val).into()
    }

    pub fn evaluate_(&self, x: Scalar) -> Scalar {
        self.poly.evaluate(&x.val).into()
    }

    pub fn degree(&self) -> usize {
        self.poly.degree()
    }

    pub fn coeffs(&self) -> Vec<Scalar> {
        self.poly.coeffs().iter().map(|&c| c.into()).collect()
    }

    pub fn from_coefficients_vec(coeffs: Vec<Scalar>) -> Self {
        PallasPoly::from_coefficients_vec(coeffs.iter().map(|c| c.val).collect()).into()
    }
}

impl Mul for Poly {
    type Output = Poly;

    fn mul(self, rhs: Poly) -> Self::Output {
        (self.poly * rhs.poly).into()
    }
}

impl<'a> Mul<&'a Poly> for Poly {
    type Output = Poly;

    fn mul(self, rhs: &'a Poly) -> Self::Output {
        (self.poly * &rhs.poly).into()
    }
}

impl<'a> Mul<Poly> for &'a Poly {
    type Output = Poly;

    fn mul(self, rhs: Poly) -> Self::Output {
        (&self.poly * rhs.poly).into()
    }
}

impl<'a, 'b> Mul<&'b Poly> for &'a Poly {
    type Output = Poly;

    fn mul(self, rhs: &'b Poly) -> Self::Output {
        (&self.poly * &rhs.poly).into()
    }
}

impl Div for Poly {
    type Output = Poly;

    fn div(self, rhs: Poly) -> Self::Output {
        (self.poly / rhs.poly).into()
    }
}

impl<'a> Div<&'a Poly> for Poly {
    type Output = Poly;

    fn div(self, rhs: &'a Poly) -> Self::Output {
        (self.poly / &rhs.poly).into()
    }
}

impl<'a> Div<Poly> for &'a Poly {
    type Output = Poly;

    fn div(self, rhs: Poly) -> Self::Output {
        (&self.poly / rhs.poly).into()
    }
}

impl<'a, 'b> Div<&'b Poly> for &'a Poly {
    type Output = Poly;

    fn div(self, rhs: &'b Poly) -> Self::Output {
        (&self.poly / &rhs.poly).into()
    }
}

impl Add for Poly {
    type Output = Poly;

    fn add(self, rhs: Poly) -> Self::Output {
        (self.poly + rhs.poly).into()
    }
}

impl<'a> Add<&'a Poly> for Poly {
    type Output = Poly;

    fn add(self, rhs: &'a Poly) -> Self::Output {
        (self.poly + &rhs.poly).into()
    }
}

impl<'a> Add<Poly> for &'a Poly {
    type Output = Poly;

    fn add(self, rhs: Poly) -> Self::Output {
        (&self.poly + rhs.poly).into()
    }
}

impl<'a, 'b> Add<&'b Poly> for &'a Poly {
    type Output = Poly;

    fn add(self, rhs: &'b Poly) -> Self::Output {
        (&self.poly + &rhs.poly).into()
    }
}

impl Sub for Poly {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        (self.poly - rhs.poly).into()
    }
}

impl<'a> Sub<&'a Poly> for Poly {
    type Output = Poly;

    fn sub(self, rhs: &'a Poly) -> Self::Output {
        (self.poly - &rhs.poly).into()
    }
}

impl<'a> Sub<Poly> for &'a Poly {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        (&self.poly - rhs.poly).into()
    }
}

impl<'a, 'b> Sub<&'b Poly> for &'a Poly {
    type Output = Poly;

    fn sub(self, rhs: &'b Poly) -> Self::Output {
        (&self.poly - &rhs.poly).into()
    }
}

// Scalar ops ------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Scalar {
    pub val: PallasScalar,
}

impl From<u64> for Scalar {
    fn from(val: u64) -> Self {
        Scalar {
            val: PallasScalar::from(val),
        }
    }
}

impl From<PallasScalar> for Scalar {
    fn from(val: PallasScalar) -> Self {
        Scalar { val }
    }
}

impl From<Scalar> for PallasScalar {
    fn from(scalar: Scalar) -> Self {
        scalar.val
    }
}

impl fmt::Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl Scalar {
    pub fn inverse(&self) -> Option<Scalar> {
        self.val.inverse().map(|v| v.into())
    }

    pub fn pow(&self, exp: u64) -> Scalar {
        self.val.pow([exp]).into()
    }
}

impl Neg for Scalar {
    type Output = Scalar;

    fn neg(self) -> Self::Output {
        (-self.val).into()
    }
}

impl<'a> Neg for &'a Scalar {
    type Output = Scalar;

    fn neg(self) -> Self::Output {
        (-self.val).into()
    }
}

impl From<Scalar> for Poly {
    fn from(scalar: Scalar) -> Self {
        p(&scalar.val)
    }
}

impl Mul for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        (self.val * rhs.val).into()
    }
}

impl<'a> Mul<&'a Scalar> for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &'a Scalar) -> Self::Output {
        (self.val * rhs.val).into()
    }
}

impl<'a> Mul<Scalar> for &'a Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        (self.val * rhs.val).into()
    }
}

impl<'a, 'b> Mul<&'b Scalar> for &'a Scalar {
    type Output = Scalar;

    fn mul(self, rhs: &'b Scalar) -> Self::Output {
        (self.val * rhs.val).into()
    }
}

impl Div for Scalar {
    type Output = Scalar;

    fn div(self, rhs: Scalar) -> Self::Output {
        (self.val / rhs.val).into()
    }
}

impl<'a> Div<&'a Scalar> for Scalar {
    type Output = Scalar;

    fn div(self, rhs: &'a Scalar) -> Self::Output {
        (self.val / rhs.val).into()
    }
}

impl<'a> Div<Scalar> for &'a Scalar {
    type Output = Scalar;

    fn div(self, rhs: Scalar) -> Self::Output {
        (self.val / rhs.val).into()
    }
}

impl<'a, 'b> Div<&'b Scalar> for &'a Scalar {
    type Output = Scalar;

    fn div(self, rhs: &'b Scalar) -> Self::Output {
        (self.val / rhs.val).into()
    }
}

impl Add for Scalar {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Self::Output {
        (self.val + rhs.val).into()
    }
}

impl<'a> Add<&'a Scalar> for Scalar {
    type Output = Scalar;

    fn add(self, rhs: &'a Scalar) -> Self::Output {
        (self.val + rhs.val).into()
    }
}

impl<'a> Add<Scalar> for &'a Scalar {
    type Output = Scalar;

    fn add(self, rhs: Scalar) -> Self::Output {
        (self.val + rhs.val).into()
    }
}

impl<'a, 'b> Add<&'b Scalar> for &'a Scalar {
    type Output = Scalar;

    fn add(self, rhs: &'b Scalar) -> Self::Output {
        (self.val + rhs.val).into()
    }
}

impl Sub for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Self::Output {
        (self.val - rhs.val).into()
    }
}

impl<'a> Sub<&'a Scalar> for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: &'a Scalar) -> Self::Output {
        (self.val - rhs.val).into()
    }
}

impl<'a> Sub<Scalar> for &'a Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Scalar) -> Self::Output {
        (self.val - rhs.val).into()
    }
}

impl<'a, 'b> Sub<&'b Scalar> for &'a Scalar {
    type Output = Scalar;

    fn sub(self, rhs: &'b Scalar) -> Self::Output {
        (self.val - rhs.val).into()
    }
}

// Poly-Scalar ops ------------------------------------------------

impl Mul<Poly> for Scalar {
    type Output = Poly;

    fn mul(self, rhs: Poly) -> Self::Output {
        (rhs.poly * self.val).into()
    }
}

impl<'a> Mul<&'a Poly> for Scalar {
    type Output = Poly;

    fn mul(self, rhs: &'a Poly) -> Self::Output {
        (&rhs.poly * self.val).into()
    }
}

impl<'a> Mul<Poly> for &'a Scalar {
    type Output = Poly;

    fn mul(self, rhs: Poly) -> Self::Output {
        (rhs.poly * self.val).into()
    }
}

impl<'a, 'b> Mul<&'b Poly> for &'a Scalar {
    type Output = Poly;

    fn mul(self, rhs: &'b Poly) -> Self::Output {
        (&rhs.poly * self.val).into()
    }
}

impl Mul<Scalar> for Poly {
    type Output = Poly;

    fn mul(self, rhs: Scalar) -> Self::Output {
        (self.poly * rhs.val).into()
    }
}

impl<'a> Mul<&'a Scalar> for Poly {
    type Output = Poly;

    fn mul(self, rhs: &'a Scalar) -> Self::Output {
        (self.poly * rhs.val).into()
    }
}

impl<'a> Mul<Scalar> for &'a Poly {
    type Output = Poly;

    fn mul(self, rhs: Scalar) -> Self::Output {
        (&self.poly * rhs.val).into()
    }
}

impl<'a, 'b> Mul<&'b Scalar> for &'a Poly {
    type Output = Poly;

    fn mul(self, rhs: &'b Scalar) -> Self::Output {
        (&self.poly * rhs.val).into()
    }
}

impl Div<Scalar> for Poly {
    type Output = Poly;

    fn div(self, rhs: Scalar) -> Self::Output {
        (self.poly * rhs.val.inverse().unwrap()).into()
    }
}

impl<'a> Div<&'a Scalar> for Poly {
    type Output = Poly;

    fn div(self, rhs: &'a Scalar) -> Self::Output {
        (self.poly * rhs.val.inverse().unwrap()).into()
    }
}

impl<'a> Div<Scalar> for &'a Poly {
    type Output = Poly;

    fn div(self, rhs: Scalar) -> Self::Output {
        (&self.poly * rhs.val.inverse().unwrap()).into()
    }
}

impl<'a, 'b> Div<&'b Scalar> for &'a Poly {
    type Output = Poly;

    fn div(self, rhs: &'b Scalar) -> Self::Output {
        (&self.poly * rhs.val.inverse().unwrap()).into()
    }
}

impl Div<Poly> for Scalar {
    type Output = Poly;

    fn div(self, rhs: Poly) -> Self::Output {
        (p(&self.val).poly / rhs.poly).into()
    }
}

impl<'a> Div<&'a Poly> for Scalar {
    type Output = Poly;

    fn div(self, rhs: &'a Poly) -> Self::Output {
        (p(&self.val).poly / &rhs.poly).into()
    }
}

impl<'a> Div<Poly> for &'a Scalar {
    type Output = Poly;

    fn div(self, rhs: Poly) -> Self::Output {
        (p(&self.val).poly / rhs.poly).into()
    }
}

impl<'a, 'b> Div<&'b Poly> for &'a Scalar {
    type Output = Poly;

    fn div(self, rhs: &'b Poly) -> Self::Output {
        (p(&self.val).poly / &rhs.poly).into()
    }
}

impl Add<Scalar> for Poly {
    type Output = Poly;

    fn add(self, rhs: Scalar) -> Self::Output {
        (self.poly + p_(rhs.val).poly).into()
    }
}

impl<'a> Add<&'a Scalar> for Poly {
    type Output = Poly;

    fn add(self, rhs: &'a Scalar) -> Self::Output {
        (self.poly + p_(rhs.val).poly).into()
    }
}

impl<'a> Add<Scalar> for &'a Poly {
    type Output = Poly;

    fn add(self, rhs: Scalar) -> Self::Output {
        (&self.poly + p_(rhs.val).poly).into()
    }
}

impl<'a, 'b> Add<&'b Scalar> for &'a Poly {
    type Output = Poly;

    fn add(self, rhs: &'b Scalar) -> Self::Output {
        (&self.poly + p_(rhs.val).poly).into()
    }
}

impl Add<Poly> for Scalar {
    type Output = Poly;

    fn add(self, rhs: Poly) -> Self::Output {
        (rhs.poly + p_(self.val).poly).into()
    }
}

impl<'a> Add<&'a Poly> for Scalar {
    type Output = Poly;

    fn add(self, rhs: &'a Poly) -> Self::Output {
        (&rhs.poly + p_(self.val).poly).into()
    }
}

impl<'a> Add<Poly> for &'a Scalar {
    type Output = Poly;

    fn add(self, rhs: Poly) -> Self::Output {
        (rhs.poly + p_(self.val).poly).into()
    }
}

impl<'a, 'b> Add<&'b Poly> for &'a Scalar {
    type Output = Poly;

    fn add(self, rhs: &'b Poly) -> Self::Output {
        (&rhs.poly + p_(self.val).poly).into()
    }
}

impl Sub<Poly> for Scalar {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        (p(&self.val).poly - rhs.poly).into()
    }
}

impl<'a> Sub<&'a Poly> for Scalar {
    type Output = Poly;

    fn sub(self, rhs: &'a Poly) -> Self::Output {
        (p(&self.val).poly - &rhs.poly).into()
    }
}

impl<'a> Sub<Poly> for &'a Scalar {
    type Output = Poly;

    fn sub(self, rhs: Poly) -> Self::Output {
        (p(&self.val).poly - rhs.poly).into()
    }
}

impl<'a, 'b> Sub<&'b Poly> for &'a Scalar {
    type Output = Poly;

    fn sub(self, rhs: &'b Poly) -> Self::Output {
        (p(&self.val).poly - &rhs.poly).into()
    }
}

impl Sub<Scalar> for Poly {
    type Output = Poly;

    fn sub(self, rhs: Scalar) -> Self::Output {
        (self.poly - p_(rhs.val).poly).into()
    }
}

impl<'a> Sub<&'a Scalar> for Poly {
    type Output = Poly;

    fn sub(self, rhs: &'a Scalar) -> Self::Output {
        (self.poly - p_(rhs.val).poly).into()
    }
}

impl<'a> Sub<Scalar> for &'a Poly {
    type Output = Poly;

    fn sub(self, rhs: Scalar) -> Self::Output {
        (&self.poly - p_(rhs.val).poly).into()
    }
}

impl<'a, 'b> Sub<&'b Scalar> for &'a Poly {
    type Output = Poly;

    fn sub(self, rhs: &'b Scalar) -> Self::Output {
        (&self.poly - p_(rhs.val).poly).into()
    }
}

// Point ops ------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    point: PallasPoint,
}

impl From<PallasPoint> for Point {
    fn from(point: PallasPoint) -> Self {
        Point { point }
    }
}

impl From<Point> for PallasPoint {
    fn from(point: Point) -> Self {
        point.point
    }
}

impl Distribution<Scalar> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Scalar {
        let scalar: PallasScalar = rng.gen();
        scalar.into()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        (self.point + rhs.point).into()
    }
}

impl<'a> Add<&'a Point> for Point {
    type Output = Point;

    fn add(self, rhs: &'a Point) -> Self::Output {
        (self.point + rhs.point).into()
    }
}

impl<'a> Add<Point> for &'a Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        (self.point + rhs.point).into()
    }
}

impl<'a, 'b> Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, rhs: &'b Point) -> Self::Output {
        (self.point + rhs.point).into()
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        (self.point - rhs.point).into()
    }
}

impl<'a> Sub<&'a Point> for Point {
    type Output = Point;

    fn sub(self, rhs: &'a Point) -> Self::Output {
        (self.point - rhs.point).into()
    }
}

impl<'a> Sub<Point> for &'a Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        (self.point - rhs.point).into()
    }
}

impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Point;

    fn sub(self, rhs: &'b Point) -> Self::Output {
        (self.point - rhs.point).into()
    }
}

// Point-Scalar ops ------------------------------------------------

impl Mul<Scalar> for Point {
    type Output = Point;

    fn mul(self, rhs: Scalar) -> Self::Output {
        (self.point * rhs.val).into()
    }
}

impl<'a> Mul<&'a Scalar> for Point {
    type Output = Point;

    fn mul(self, rhs: &'a Scalar) -> Self::Output {
        (self.point * rhs.val).into()
    }
}

impl<'a> Mul<Scalar> for &'a Point {
    type Output = Point;

    fn mul(self, rhs: Scalar) -> Self::Output {
        (self.point * rhs.val).into()
    }
}

impl<'a, 'b> Mul<&'b Scalar> for &'a Point {
    type Output = Point;

    fn mul(self, rhs: &'b Scalar) -> Self::Output {
        (self.point * rhs.val).into()
    }
}

impl Mul<Point> for Scalar {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        (rhs.point * self.val).into()
    }
}

impl<'a> Mul<&'a Point> for Scalar {
    type Output = Point;

    fn mul(self, rhs: &'a Point) -> Self::Output {
        (rhs.point * self.val).into()
    }
}

impl<'a> Mul<Point> for &'a Scalar {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        (rhs.point * self.val).into()
    }
}

impl<'a, 'b> Mul<&'b Point> for &'a Scalar {
    type Output = Point;

    fn mul(self, rhs: &'b Point) -> Self::Output {
        (rhs.point * self.val).into()
    }
}

impl Div<Scalar> for Point {
    type Output = Point;

    fn div(self, rhs: Scalar) -> Self::Output {
        (self.point * rhs.val.inverse().unwrap()).into()
    }
}

impl<'a> Div<&'a Scalar> for Point {
    type Output = Point;

    fn div(self, rhs: &'a Scalar) -> Self::Output {
        (self.point * rhs.val.inverse().unwrap()).into()
    }
}

impl<'a> Div<Scalar> for &'a Point {
    type Output = Point;

    fn div(self, rhs: Scalar) -> Self::Output {
        (self.point * rhs.val.inverse().unwrap()).into()
    }
}

impl<'a, 'b> Div<&'b Scalar> for &'a Point {
    type Output = Point;

    fn div(self, rhs: &'b Scalar) -> Self::Output {
        (self.point * rhs.val.inverse().unwrap()).into()
    }
}

// CommitData ops ------------------------------------------------

impl Add for CommitData {
    type Output = CommitData;

    fn add(self, rhs: CommitData) -> Self::Output {
        assert_eq!(self.d, rhs.d, "CommitData `d` must be equal");
        CommitData {
            d: self.d,
            pt: self.pt + rhs.pt,
        }
    }
}

impl<'a> Add<&'a CommitData> for CommitData {
    type Output = CommitData;

    fn add(self, rhs: &'a CommitData) -> Self::Output {
        assert_eq!(self.d, rhs.d, "CommitData `d` must be equal");
        CommitData {
            d: self.d,
            pt: self.pt + rhs.pt,
        }
    }
}

impl<'a> Add<CommitData> for &'a CommitData {
    type Output = CommitData;

    fn add(self, rhs: CommitData) -> Self::Output {
        assert_eq!(self.d, rhs.d, "CommitData `d` must be equal");
        CommitData {
            d: self.d,
            pt: self.pt + rhs.pt,
        }
    }
}

impl<'a, 'b> Add<&'b CommitData> for &'a CommitData {
    type Output = CommitData;

    fn add(self, rhs: &'b CommitData) -> Self::Output {
        assert_eq!(self.d, rhs.d, "CommitData `d` must be equal");
        CommitData {
            d: self.d,
            pt: self.pt + rhs.pt,
        }
    }
}

impl Sub for CommitData {
    type Output = CommitData;

    fn sub(self, rhs: CommitData) -> Self::Output {
        // assert_eq!(self.d, rhs.d, "CommitData `d` must be equal");
        CommitData {
            d: self.d,
            pt: self.pt - rhs.pt,
        }
    }
}

impl<'a> Sub<&'a CommitData> for CommitData {
    type Output = CommitData;

    fn sub(self, rhs: &'a CommitData) -> Self::Output {
        assert_eq!(self.d, rhs.d, "CommitData `d` must be equal");
        CommitData {
            d: self.d,
            pt: self.pt - rhs.pt,
        }
    }
}

impl<'a> Sub<CommitData> for &'a CommitData {
    type Output = CommitData;

    fn sub(self, rhs: CommitData) -> Self::Output {
        assert_eq!(self.d, rhs.d, "CommitData `d` must be equal");
        CommitData {
            d: self.d,
            pt: self.pt - rhs.pt,
        }
    }
}

impl<'a, 'b> Sub<&'b CommitData> for &'a CommitData {
    type Output = CommitData;

    fn sub(self, rhs: &'b CommitData) -> Self::Output {
        assert_eq!(self.d, rhs.d, "CommitData `d` must be equal");
        CommitData {
            d: self.d,
            pt: self.pt - rhs.pt,
        }
    }
}

// CommitData-Scalar ops ------------------------------------------------

impl Mul<Scalar> for CommitData {
    type Output = CommitData;

    fn mul(self, rhs: Scalar) -> Self::Output {
        CommitData {
            d: self.d,
            pt: self.pt * rhs.val,
        }
    }
}

impl<'a> Mul<&'a Scalar> for CommitData {
    type Output = CommitData;

    fn mul(self, rhs: &'a Scalar) -> Self::Output {
        CommitData {
            d: self.d,
            pt: self.pt * rhs.val,
        }
    }
}

impl<'a> Mul<Scalar> for &'a CommitData {
    type Output = CommitData;

    fn mul(self, rhs: Scalar) -> Self::Output {
        CommitData {
            d: self.d,
            pt: self.pt * rhs.val,
        }
    }
}

impl<'a, 'b> Mul<&'b Scalar> for &'a CommitData {
    type Output = CommitData;

    fn mul(self, rhs: &'b Scalar) -> Self::Output {
        CommitData {
            d: self.d,
            pt: self.pt * rhs.val,
        }
    }
}

impl Mul<CommitData> for Scalar {
    type Output = CommitData;

    fn mul(self, rhs: CommitData) -> Self::Output {
        CommitData {
            d: rhs.d,
            pt: rhs.pt * self.val,
        }
    }
}

impl<'a> Mul<&'a CommitData> for Scalar {
    type Output = CommitData;

    fn mul(self, rhs: &'a CommitData) -> Self::Output {
        CommitData {
            d: rhs.d,
            pt: rhs.pt * self.val,
        }
    }
}

impl<'a> Mul<CommitData> for &'a Scalar {
    type Output = CommitData;

    fn mul(self, rhs: CommitData) -> Self::Output {
        CommitData {
            d: rhs.d,
            pt: rhs.pt * self.val,
        }
    }
}

impl<'a, 'b> Mul<&'b CommitData> for &'a Scalar {
    type Output = CommitData;

    fn mul(self, rhs: &'b CommitData) -> Self::Output {
        CommitData {
            d: rhs.d,
            pt: rhs.pt * self.val,
        }
    }
}

impl Div<Scalar> for CommitData {
    type Output = CommitData;

    fn div(self, rhs: Scalar) -> Self::Output {
        CommitData {
            d: self.d,
            pt: self.pt * rhs.val.inverse().unwrap(),
        }
    }
}

impl<'a> Div<&'a Scalar> for CommitData {
    type Output = CommitData;

    fn div(self, rhs: &'a Scalar) -> Self::Output {
        CommitData {
            d: self.d,
            pt: self.pt * rhs.val.inverse().unwrap(),
        }
    }
}

impl<'a> Div<Scalar> for &'a CommitData {
    type Output = CommitData;

    fn div(self, rhs: Scalar) -> Self::Output {
        CommitData {
            d: self.d,
            pt: self.pt * rhs.val.inverse().unwrap(),
        }
    }
}

impl<'a, 'b> Div<&'b Scalar> for &'a CommitData {
    type Output = CommitData;

    fn div(self, rhs: &'b Scalar) -> Self::Output {
        CommitData {
            d: self.d,
            pt: self.pt * rhs.val.inverse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ark_ff::{FftField, Field};
    use ark_poly::{
        evaluations::univariate::Evaluations, EvaluationDomain, GeneralEvaluationDomain, Polynomial,
    };

    #[test]
    fn test_largest_root_of_unity() {
        let n = 2u64.pow(32);
        let omega = PallasScalar::get_root_of_unity(n);
        assert!(omega != None);
    }

    #[test]
    fn test_interpolate() {
        let omega = &PallasScalar::get_root_of_unity(4).unwrap();
        let evals = vec![
            PallasScalar::from(10u64),
            PallasScalar::from(2u64),
            PallasScalar::from(13u64),
        ];
        let domain = GeneralEvaluationDomain::<PallasScalar>::new(3).unwrap();
        let poly = Evaluations::from_vec_and_domain(evals, domain).interpolate();
        assert_eq!(poly.evaluate(&PallasScalar::ONE), PallasScalar::from(10u64));
        assert_eq!(poly.evaluate(omega), PallasScalar::from(2u64));
        assert_eq!(poly.evaluate(&(omega * omega)), PallasScalar::from(13u64));
    }

    #[test]
    fn test_interpolate_points_to_polys() {
        let order = 16;
        let omega = &get_omega(order);
        let points = vec![
            vec![
                PallasScalar::from(1u64),
                PallasScalar::from(2u64),
                PallasScalar::from(3u64),
                PallasScalar::from(4u64),
            ],
            vec![
                PallasScalar::from(5u64),
                PallasScalar::from(6u64),
                PallasScalar::from(7u64),
                PallasScalar::from(8u64),
            ],
        ];
        let polys = points
            .iter()
            .map(|p| interpolate(order, p).into())
            .collect::<Vec<Poly>>();
        assert_eq!(polys.len(), 2);

        assert_eq!(
            polys[0].evaluate(&PallasScalar::ONE.into()),
            PallasScalar::from(1u64).into()
        );
        assert_eq!(polys[0].evaluate(omega), PallasScalar::from(2u64).into());
        assert_eq!(
            polys[0].evaluate(&(omega * omega)),
            PallasScalar::from(3u64).into()
        );
        assert_eq!(
            polys[0].evaluate(&(omega * omega * omega)),
            PallasScalar::from(4u64).into()
        );

        assert_eq!(
            polys[1].evaluate(&PallasScalar::ONE.into()),
            PallasScalar::from(5u64).into()
        );
        assert_eq!(polys[1].evaluate(omega), PallasScalar::from(6u64).into());
        assert_eq!(
            polys[1].evaluate(&(omega * omega)),
            PallasScalar::from(7u64).into()
        );
        assert_eq!(
            polys[1].evaluate(&(omega * omega * omega)),
            PallasScalar::from(8u64).into()
        );
    }
}

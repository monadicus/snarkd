use super::{LabeledPolynomial, PolynomialInfo};
use crate::{
    bls12_377::{Field, G1Affine, G1Projective, G2Affine, G2Prepared, Projective, Scalar},
    fft::EvaluationDomain,
    polycommit::kzg10,
    utils::*,
    Prepare,
};
use hashbrown::HashMap;
use std::{
    borrow::{Borrow, Cow},
    collections::{BTreeMap, BTreeSet},
    fmt,
    ops::{AddAssign, MulAssign, SubAssign},
};

/// `UniversalParams` are the universal parameters for the KZG10 scheme.
pub type UniversalParams = kzg10::UniversalParams;

/// `Randomness` is the randomness for the KZG10 scheme.
pub type Randomness = kzg10::KZGRandomness;

/// `Commitment` is the commitment for the KZG10 scheme.
pub type Commitment = kzg10::KZGCommitment;

/// `PreparedCommitment` is the prepared commitment for the KZG10 scheme.
pub type PreparedCommitment = kzg10::PreparedKZGCommitment;

impl Prepare for Commitment {
    type Prepared = PreparedCommitment;

    /// prepare `PreparedCommitment` from `Commitment`
    fn prepare(&self) -> PreparedCommitment {
        let mut prepared_comm = Vec::<G1Affine>::new();
        let mut cur = G1Projective::from(self.0);
        for _ in 0..128 {
            prepared_comm.push(cur.into());
            cur.double_in_place();
        }

        kzg10::PreparedKZGCommitment(prepared_comm)
    }
}

/// `CommitterKey` is used to commit to, and create evaluation proofs for, a given polynomial.
#[derive(Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct CommitterKey {
    /// The key used to commit to polynomials.
    pub powers_of_beta_g: Vec<G1Affine>,

    /// The key used to commit to polynomials in Lagrange basis.
    pub lagrange_bases_at_beta_g: BTreeMap<usize, Vec<G1Affine>>,

    /// The key used to commit to hiding polynomials.
    pub powers_of_beta_times_gamma_g: Vec<G1Affine>,

    /// The powers used to commit to shifted polynomials.
    /// This is `None` if `self` does not support enforcing any degree bounds.
    pub shifted_powers_of_beta_g: Option<Vec<G1Affine>>,

    /// The powers used to commit to shifted hiding polynomials.
    /// This is `None` if `self` does not support enforcing any degree bounds.
    pub shifted_powers_of_beta_times_gamma_g: Option<BTreeMap<usize, Vec<G1Affine>>>,

    /// The degree bounds that are supported by `self`.
    /// Sorted in ascending order from smallest bound to largest bound.
    /// This is `None` if `self` does not support enforcing any degree bounds.
    pub enforced_degree_bounds: Option<Vec<usize>>,

    /// The maximum degree supported by the `UniversalParams` from which `self` was derived
    pub max_degree: usize,
}

impl CommitterKey {
    /// Obtain powers for the underlying KZG10 construction
    pub fn powers(&self) -> kzg10::Powers {
        kzg10::Powers {
            powers_of_beta_g: self.powers_of_beta_g.as_slice().into(),
            powers_of_beta_times_gamma_g: self.powers_of_beta_times_gamma_g.as_slice().into(),
        }
    }

    /// Obtain powers for committing to shifted polynomials.
    pub fn shifted_powers_of_beta_g(
        &self,
        degree_bound: impl Into<Option<usize>>,
    ) -> Option<kzg10::Powers> {
        match (
            &self.shifted_powers_of_beta_g,
            &self.shifted_powers_of_beta_times_gamma_g,
        ) {
            (Some(shifted_powers_of_beta_g), Some(shifted_powers_of_beta_times_gamma_g)) => {
                let max_bound = self
                    .enforced_degree_bounds
                    .as_ref()
                    .unwrap()
                    .last()
                    .unwrap();
                let (bound, powers_range) = if let Some(degree_bound) = degree_bound.into() {
                    assert!(self
                        .enforced_degree_bounds
                        .as_ref()
                        .unwrap()
                        .contains(&degree_bound));
                    (degree_bound, (max_bound - degree_bound)..)
                } else {
                    (*max_bound, 0..)
                };

                let ck = kzg10::Powers {
                    powers_of_beta_g: shifted_powers_of_beta_g[powers_range].into(),
                    powers_of_beta_times_gamma_g: shifted_powers_of_beta_times_gamma_g[&bound]
                        .clone()
                        .into(),
                };

                Some(ck)
            }

            (_, _) => None,
        }
    }

    /// Obtain elements of the SRS in the lagrange basis powers, for use with the underlying
    /// KZG10 construction.
    pub fn lagrange_basis(&self, domain: EvaluationDomain) -> Option<kzg10::LagrangeBasis> {
        self.lagrange_bases_at_beta_g
            .get(&domain.size())
            .map(|basis| kzg10::LagrangeBasis {
                lagrange_basis_at_beta_g: Cow::Borrowed(basis),
                powers_of_beta_times_gamma_g: Cow::Borrowed(&self.powers_of_beta_times_gamma_g),
                domain,
            })
    }
}

impl CommitterKey {
    pub fn max_degree(&self) -> usize {
        self.max_degree
    }

    pub fn supported_degree(&self) -> usize {
        self.powers_of_beta_g.len() - 1
    }
}

/// `VerifierKey` is used to check evaluation proofs for a given commitment.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VerifierKey {
    /// The verification key for the underlying KZG10 scheme.
    pub vk: kzg10::VerifierKey,

    /// Pairs a degree_bound with its corresponding G2 element.
    /// Each pair is in the form `(degree_bound, \beta^{degree_bound - max_degree} h),` where `h` is the generator of G2 above
    pub degree_bounds_and_neg_powers_of_h: Option<Vec<(usize, G2Affine)>>,

    /// The prepared version of `degree_bounds_and_neg_powers_of_h`.
    pub degree_bounds_and_prepared_neg_powers_of_h: Option<Vec<(usize, G2Prepared)>>,

    /// The maximum degree supported by the trimmed parameters that `self` is
    /// a part of.
    pub supported_degree: usize,

    /// The maximum degree supported by the `UniversalParams` `self` was derived
    /// from.
    pub max_degree: usize,
}

impl VerifierKey {
    /// Find the appropriate shift for the degree bound.
    pub fn get_shift_power(&self, degree_bound: usize) -> Option<G2Affine> {
        self.degree_bounds_and_neg_powers_of_h
            .as_ref()
            .and_then(|v| {
                v.binary_search_by(|(d, _)| d.cmp(&degree_bound))
                    .ok()
                    .map(|i| v[i].1)
            })
    }

    pub fn get_prepared_shift_power(&self, degree_bound: usize) -> Option<G2Prepared> {
        self.degree_bounds_and_prepared_neg_powers_of_h
            .as_ref()
            .and_then(|v| {
                v.binary_search_by(|(d, _)| d.cmp(&degree_bound))
                    .ok()
                    .map(|i| v[i].1.clone())
            })
    }
}

impl VerifierKey {
    pub fn max_degree(&self) -> usize {
        self.max_degree
    }

    pub fn supported_degree(&self) -> usize {
        self.supported_degree
    }
}

/// `PreparedVerifierKey` is used to check evaluation proofs for a given commitment.
#[derive(Clone, Debug)]
pub struct PreparedVerifierKey {
    /// The verification key for the underlying KZG10 scheme.
    pub prepared_vk: kzg10::PreparedVerifierKey,
    /// Information required to enforce degree bounds. Each pair
    /// is of the form `(degree_bound, shifting_advice)`.
    /// This is `None` if `self` does not support enforcing any degree bounds.
    pub degree_bounds_and_prepared_neg_powers_of_h: Option<Vec<(usize, G2Prepared)>>,
    /// The maximum degree supported by the `UniversalParams` `self` was derived
    /// from.
    pub max_degree: usize,
    /// The maximum degree supported by the trimmed parameters that `self` is
    /// a part of.
    pub supported_degree: usize,
}

impl PreparedVerifierKey {
    /// Find the appropriate shift for the degree bound.
    pub fn get_prepared_shift_power(&self, bound: usize) -> Option<G2Prepared> {
        self.degree_bounds_and_prepared_neg_powers_of_h
            .as_ref()
            .and_then(|v| {
                v.binary_search_by(|(d, _)| d.cmp(&bound))
                    .ok()
                    .map(|i| v[i].1.clone())
            })
    }
}

impl Prepare for VerifierKey {
    type Prepared = PreparedVerifierKey;

    /// prepare `PreparedVerifierKey` from `VerifierKey`
    fn prepare(&self) -> PreparedVerifierKey {
        let prepared_vk = kzg10::PreparedVerifierKey::prepare(&self.vk);

        PreparedVerifierKey {
            prepared_vk,
            degree_bounds_and_prepared_neg_powers_of_h: self
                .degree_bounds_and_prepared_neg_powers_of_h
                .clone(),
            max_degree: self.max_degree,
            supported_degree: self.supported_degree,
        }
    }
}

/// Evaluation proof at a query set.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct BatchProof(pub(crate) Vec<kzg10::KZGProof>);

impl BatchProof {
    pub fn is_hiding(&self) -> bool {
        self.0.iter().any(|c| c.is_hiding())
    }
}

/// Labels a `LabeledPolynomial` or a `LabeledCommitment`.
pub type PolynomialLabel = String;

/// A commitment along with information about its degree bound (if any).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LabeledCommitment {
    label: PolynomialLabel,
    commitment: Commitment,
    degree_bound: Option<usize>,
}

impl LabeledCommitment {
    /// Instantiate a new polynomial_context.
    pub fn new(
        label: PolynomialLabel,
        commitment: Commitment,
        degree_bound: Option<usize>,
    ) -> Self {
        Self {
            label,
            commitment,
            degree_bound,
        }
    }

    pub fn new_with_info(info: &PolynomialInfo, commitment: Commitment) -> Self {
        Self {
            label: info.label().to_string(),
            commitment,
            degree_bound: info.degree_bound(),
        }
    }

    /// Return the label for `self`.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Retrieve the commitment from `self`.
    pub fn commitment(&self) -> &Commitment {
        &self.commitment
    }

    /// Retrieve the degree bound in `self`.
    pub fn degree_bound(&self) -> Option<usize> {
        self.degree_bound
    }
}

/// A term in a linear combination.
#[derive(Hash, Ord, PartialOrd, Clone, Eq, PartialEq)]
pub enum LCTerm {
    /// The constant term representing `one`.
    One,
    /// Label for a polynomial.
    PolyLabel(String),
}

impl fmt::Debug for LCTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LCTerm::One => write!(f, "1"),
            LCTerm::PolyLabel(label) => write!(f, "{label}"),
        }
    }
}

impl LCTerm {
    /// Returns `true` if `self == LCTerm::One`
    #[inline]
    pub fn is_one(&self) -> bool {
        matches!(self, LCTerm::One)
    }
}

impl From<PolynomialLabel> for LCTerm {
    fn from(other: PolynomialLabel) -> Self {
        Self::PolyLabel(other)
    }
}

impl<'a> From<&'a str> for LCTerm {
    fn from(other: &str) -> Self {
        Self::PolyLabel(other.into())
    }
}

impl core::convert::TryInto<PolynomialLabel> for LCTerm {
    type Error = ();

    fn try_into(self) -> Result<PolynomialLabel, ()> {
        match self {
            Self::One => Err(()),
            Self::PolyLabel(l) => Ok(l),
        }
    }
}

impl<'a> core::convert::TryInto<&'a PolynomialLabel> for &'a LCTerm {
    type Error = ();

    fn try_into(self) -> Result<&'a PolynomialLabel, ()> {
        match self {
            LCTerm::One => Err(()),
            LCTerm::PolyLabel(l) => Ok(l),
        }
    }
}

impl<B: Borrow<String>> PartialEq<B> for LCTerm {
    fn eq(&self, other: &B) -> bool {
        match self {
            Self::One => false,
            Self::PolyLabel(l) => l == other.borrow(),
        }
    }
}

/// A labeled linear combinations of polynomials.
#[derive(Clone, Debug)]
pub struct LinearCombination {
    /// The label.
    pub label: String,
    /// The linear combination of `(coeff, poly_label)` pairs.
    pub terms: BTreeMap<LCTerm, Scalar>,
}

#[allow(clippy::or_fun_call)]
impl LinearCombination {
    /// Construct an empty labeled linear combination.
    pub fn empty(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            terms: BTreeMap::new(),
        }
    }

    /// Construct a new labeled linear combination.
    /// with the terms specified in `term`.
    pub fn new(
        label: impl Into<String>,
        _terms: impl IntoIterator<Item = (Scalar, impl Into<LCTerm>)>,
    ) -> Self {
        let mut terms = BTreeMap::new();
        for (c, l) in _terms.into_iter().map(|(c, t)| (c, t.into())) {
            *terms.entry(l).or_insert(Scalar::ZERO) += c;
        }

        Self {
            label: label.into(),
            terms,
        }
    }

    /// Returns the label of the linear combination.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns `true` if the linear combination has no terms.
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }

    /// Add a term to the linear combination.
    pub fn add(&mut self, c: Scalar, t: impl Into<LCTerm>) -> &mut Self {
        let t = t.into();
        *self.terms.entry(t.clone()).or_insert(Scalar::ZERO) += c;
        if self.terms[&t].is_zero() {
            self.terms.remove(&t);
        }
        self
    }

    pub fn len(&self) -> usize {
        self.terms.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Scalar, &LCTerm)> {
        self.terms.iter().map(|(t, c)| (c, t))
    }
}

impl<'a> AddAssign<(Scalar, &'a LinearCombination)> for LinearCombination {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, (coeff, other): (Scalar, &'a LinearCombination)) {
        for (t, c) in other.terms.iter() {
            self.add(coeff * c, t.clone());
        }
    }
}

impl<'a> SubAssign<(Scalar, &'a LinearCombination)> for LinearCombination {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn sub_assign(&mut self, (coeff, other): (Scalar, &'a LinearCombination)) {
        for (t, c) in other.terms.iter() {
            self.add(-coeff * c, t.clone());
        }
    }
}

impl<'a> AddAssign<&'a LinearCombination> for LinearCombination {
    fn add_assign(&mut self, other: &'a LinearCombination) {
        for (t, c) in other.terms.iter() {
            self.add(*c, t.clone());
        }
    }
}

impl<'a> SubAssign<&'a LinearCombination> for LinearCombination {
    fn sub_assign(&mut self, other: &'a LinearCombination) {
        for (t, &c) in other.terms.iter() {
            self.add(-c, t.clone());
        }
    }
}

impl AddAssign<Scalar> for LinearCombination {
    fn add_assign(&mut self, coeff: Scalar) {
        self.add(coeff, LCTerm::One);
    }
}

impl SubAssign<Scalar> for LinearCombination {
    fn sub_assign(&mut self, coeff: Scalar) {
        self.add(-coeff, LCTerm::One);
    }
}

impl MulAssign<Scalar> for LinearCombination {
    fn mul_assign(&mut self, coeff: Scalar) {
        self.terms.values_mut().for_each(|c| *c *= &coeff);
    }
}

/// `QuerySet` is the set of queries that are to be made to a set of labeled polynomials/equations
/// `p` that have previously been committed to. Each element of a `QuerySet` is a `(label, query)`
/// pair, where `label` is the label of a polynomial in `p`, and `query` is the field element
/// that `p[label]` is to be queried at.
///
/// Added the third field: the point name.
pub type QuerySet<'a> = BTreeSet<(String, (String, Scalar))>;

/// `Evaluations` is the result of querying a set of labeled polynomials or equations
/// `p` at a `QuerySet` `Q`. It maps each element of `Q` to the resulting evaluation.
/// That is, if `(label, query)` is an element of `Q`, then `evaluation.get((label, query))`
/// should equal `p[label].evaluate(query)`.
pub type Evaluations<'a> = BTreeMap<(String, Scalar), Scalar>;

/// Evaluate the given polynomials at `query_set`.
pub fn evaluate_query_set<'a>(
    polys: impl IntoIterator<Item = &'a LabeledPolynomial>,
    query_set: &QuerySet<'a>,
) -> Evaluations<'a> {
    let polys: HashMap<_, _> = polys.into_iter().map(|p| (p.label(), p)).collect();
    let mut evaluations = Evaluations::new();
    for (label, (_point_name, point)) in query_set {
        let poly = polys
            .get(label as &str)
            .expect("polynomial in evaluated lc is not found");
        let eval = poly.evaluate(*point);
        evaluations.insert((label.clone(), *point), eval);
    }
    evaluations
}

/// A proof of satisfaction of linear combinations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BatchLCProof {
    /// Evaluation proof.
    pub proof: BatchProof,
    /// Evaluations required to verify the proof.
    pub evaluations: Option<Vec<Scalar>>,
}

impl BatchLCProof {
    pub fn is_hiding(&self) -> bool {
        self.proof.is_hiding()
    }
}

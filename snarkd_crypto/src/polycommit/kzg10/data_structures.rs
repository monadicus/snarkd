use crate::{
    bls12_377::{Fp, G1Affine, G1Projective, G2Affine, G2Prepared, Scalar},
    fft::{DensePolynomial, EvaluationDomain},
    AlgebraicSponge,
};
use anyhow::Result;
use core::ops::{Add, AddAssign};
use parking_lot::RwLock;
use rand_core::RngCore;
use std::{borrow::Cow, collections::BTreeMap, io, sync::Arc};

/// `UniversalParams` are the universal parameters for the KZG10 scheme.
#[derive(Clone, Debug)]
pub struct UniversalParams {
    /// Group elements of the form `{ \beta^i G }`, where `i` ranges from 0 to `degree`,
    /// and group elements of the form `{ \beta^i \gamma G }`, where `i` ranges from 0 to `degree`.
    /// This struct provides an abstraction over the powers which are located on-disk
    /// to reduce memory usage.
    pub powers: Arc<RwLock<PowersOfG>>,
    /// The generator of G2.
    pub h: G2Affine,
    /// \beta times the above generator of G2.
    pub beta_h: G2Affine,
    /// Supported degree bounds.
    pub supported_degree_bounds: Vec<usize>,
    /// Group elements of the form `{ \beta^{max_degree -i} G2 }`, where `i` is the supported degree bound.
    /// This one is used for deriving the verifying key.
    pub inverse_neg_powers_of_beta_h: BTreeMap<usize, G2Affine>,
    /// The generator of G2, prepared for use in pairings.
    pub prepared_h: G2Prepared,
    /// \beta times the above generator of G2, prepared for use in pairings.
    pub prepared_beta_h: G2Prepared,
}

impl UniversalParams {
    pub fn lagrange_basis(&self, domain: EvaluationDomain) -> Result<Vec<G1Affine>> {
        let basis = domain.ifft(
            &self
                .powers_of_beta_g(0, domain.size())?
                .iter()
                .map(|e| (*e).to_projective())
                .collect::<Vec<_>>(),
        );
        Ok(G1Projective::batch_normalization_into_affine(basis))
    }

    pub fn power_of_beta_g(&self, which_power: usize) -> Result<G1Affine> {
        self.powers.write().power_of_beta_g(which_power)
    }

    pub fn powers_of_beta_g(&self, lower: usize, upper: usize) -> Result<Vec<G1Affine>> {
        self.powers.write().powers_of_beta_g(lower, upper)
    }

    pub fn get_powers_times_gamma_g(&self) -> BTreeMap<usize, G1Affine> {
        self.powers.read().powers_times_gamma_g().clone()
    }

    pub fn download_up_to(&self, degree: usize) -> Result<()> {
        self.powers.write().download_up_to(degree)
    }
}

impl UniversalParams {
    pub fn max_degree(&self) -> usize {
        self.powers.read().degree() - 1
    }

    pub fn supported_degree_bounds(&self) -> &[usize] {
        &self.supported_degree_bounds
    }

    pub fn increase_degree(&self, degree: usize) -> Result<()> {
        self.download_up_to(degree)
    }
}

/// `Powers` is used to commit to and create evaluation proofs for a given polynomial.
#[derive(Clone, Debug, Default, Hash)]
pub struct Powers<'a> {
    /// Group elements of the form `β^i G`, for different values of `i`.
    pub powers_of_beta_g: Cow<'a, [G1Affine]>,
    /// Group elements of the form `β^i γG`, for different values of `i`.
    pub powers_of_beta_times_gamma_g: Cow<'a, [G1Affine]>,
}

impl Powers<'_> {
    /// The number of powers in `self`.
    pub fn size(&self) -> usize {
        self.powers_of_beta_g.len()
    }
}
/// `LagrangeBasis` is used to commit to and create evaluation proofs for a given polynomial.
#[derive(Clone, Debug, Hash)]
pub struct LagrangeBasis<'a> {
    /// Group elements of the form `β^i G`, for different values of `i`.
    pub lagrange_basis_at_beta_g: Cow<'a, [G1Affine]>,
    /// Group elements of the form `β^i γG`, for different values of `i`.
    pub powers_of_beta_times_gamma_g: Cow<'a, [G1Affine]>,
    /// Domain representing the multiplicative subgroup the powers
    /// in `self.lagrange_basis_at_beta_g` are defined over.
    pub domain: EvaluationDomain,
}

impl LagrangeBasis<'_> {
    /// The number of powers in `self`.
    pub fn size(&self) -> usize {
        self.lagrange_basis_at_beta_g.len()
    }
}

/// `VerifierKey` is used to check evaluation proofs for a given commitment.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VerifierKey {
    /// The generator of G1.
    pub g: G1Affine,
    /// The generator of G1 that is used for making a commitment hiding.
    pub gamma_g: G1Affine,
    /// The generator of G2.
    pub h: G2Affine,
    /// \beta times the above generator of G2.
    pub beta_h: G2Affine,
    /// The generator of G2, prepared for use in pairings.
    pub prepared_h: G2Prepared,
    /// \beta times the above generator of G2, prepared for use in pairings.
    pub prepared_beta_h: G2Prepared,
}

/// `PreparedVerifierKey` is the fully prepared version for checking evaluation proofs for a given commitment.
/// We omit gamma here for simplicity.
#[derive(Clone, Debug, Default)]
pub struct PreparedVerifierKey {
    /// The generator of G1, prepared for power series.
    pub prepared_g: Vec<G1Affine>,
    /// The generator of G1 that is used for making a commitment hiding, prepared for power series
    pub prepared_gamma_g: Vec<G1Affine>,
    /// The generator of G2, prepared for use in pairings.
    pub prepared_h: G2Prepared,
    /// \beta times the above generator of G2, prepared for use in pairings.
    pub prepared_beta_h: G2Prepared,
}

impl PreparedVerifierKey {
    /// prepare `PreparedVerifierKey` from `VerifierKey`
    pub fn prepare(vk: &VerifierKey) -> Self {
        let supported_bits = Scalar::size_in_bits();

        let mut prepared_g = Vec::<G1Affine>::new();
        let mut g = G1Projective::from(vk.g);
        for _ in 0..supported_bits {
            prepared_g.push(g.into());
            g.double_in_place();
        }

        let mut prepared_gamma_g = Vec::<G1Affine>::new();
        let mut gamma_g = G1Projective::from(vk.gamma_g);
        for _ in 0..supported_bits {
            prepared_gamma_g.push(gamma_g.into());
            gamma_g.double_in_place();
        }

        Self {
            prepared_g,
            prepared_gamma_g,
            prepared_h: vk.prepared_h.clone(),
            prepared_beta_h: vk.prepared_beta_h.clone(),
        }
    }
}

/// `Commitment` commits to a polynomial. It is output by `KZG10::commit`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Commitment(
    /// The commitment is a group element.
    pub G1Affine,
);

impl Commitment {
    #[inline]
    pub fn empty() -> Self {
        Commitment(G1Affine::zero())
    }

    pub fn has_degree_bound(&self) -> bool {
        false
    }

    pub fn is_in_correct_subgroup_assuming_on_curve(&self) -> bool {
        self.0.is_in_correct_subgroup_assuming_on_curve()
    }
}

/// `PreparedCommitment` commits to a polynomial and prepares for mul_bits.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PreparedCommitment(
    /// The commitment is a group element.
    pub Vec<G1Affine>,
);

impl PreparedCommitment {
    /// prepare `PreparedCommitment` from `Commitment`
    pub fn prepare(comm: &Commitment) -> Self {
        let mut prepared_comm = Vec::<G1Affine>::new();
        let mut cur = G1Projective::from(comm.0);

        let supported_bits = Scalar::size_in_bits();

        for _ in 0..supported_bits {
            prepared_comm.push(cur.into());
            cur.double_in_place();
        }

        Self(prepared_comm)
    }
}

/// `Randomness` hides the polynomial inside a commitment. It is output by `KZG10::commit`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Randomness {
    /// For KZG10, the commitment randomness is a random polynomial.
    pub blinding_polynomial: DensePolynomial,
}

impl Randomness {
    /// Does `self` provide any hiding properties to the corresponding commitment?
    /// `self.is_hiding() == true` only if the underlying polynomial is non-zero.
    #[inline]
    pub fn is_hiding(&self) -> bool {
        !self.blinding_polynomial.is_zero()
    }

    /// What is the degree of the hiding polynomial for a given hiding bound?
    #[inline]
    pub fn calculate_hiding_polynomial_degree(hiding_bound: usize) -> usize {
        hiding_bound + 1
    }
}

impl Randomness {
    pub fn empty() -> Self {
        Self {
            blinding_polynomial: DensePolynomial::zero(),
        }
    }

    pub fn rand<R: RngCore>(hiding_bound: usize, _: bool, rng: &mut R) -> Self {
        let mut randomness = Randomness::empty();
        let hiding_poly_degree = Self::calculate_hiding_polynomial_degree(hiding_bound);
        randomness.blinding_polynomial = DensePolynomial::rand(hiding_poly_degree, rng);
        randomness
    }
}

impl<'a> Add<&'a Randomness> for Randomness {
    type Output = Self;

    #[inline]
    fn add(mut self, other: &'a Self) -> Self {
        self.blinding_polynomial += &other.blinding_polynomial;
        self
    }
}

impl<'a> Add<(Scalar, &'a Randomness)> for Randomness {
    type Output = Self;

    #[inline]
    fn add(mut self, other: (Scalar, &'a Randomness)) -> Self {
        self += other;
        self
    }
}

impl<'a> AddAssign<&'a Randomness> for Randomness {
    #[inline]
    fn add_assign(&mut self, other: &'a Self) {
        self.blinding_polynomial += &other.blinding_polynomial;
    }
}

impl<'a> AddAssign<(Scalar, &'a Randomness)> for Randomness {
    #[inline]
    fn add_assign(&mut self, (f, other): (Scalar, &'a Randomness)) {
        self.blinding_polynomial += (f, &other.blinding_polynomial);
    }
}

/// `Proof` is an evaluation proof that is output by `KZG10::open`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Proof {
    /// This is a commitment to the witness polynomial; see [\[KZG10\]][kzg] for more details.
    ///
    /// [kzg]: http://cacr.uwaterloo.ca/techreports/2010/cacr2010-10.pdf
    pub w: G1Affine,
    /// This is the evaluation of the random polynomial at the point for which
    /// the evaluation proof was produced.
    pub random_v: Option<Scalar>,
}

impl Proof {
    pub fn absorb_into_sponge(&self, sponge: &mut impl AlgebraicSponge<Fp, 2>) {
        sponge.absorb_native_field_elements(&self.w.to_field_elements().unwrap());
        if let Some(random_v) = self.random_v {
            sponge.absorb_nonnative_field_elements([random_v]);
        }
    }
}

impl Proof {
    pub fn is_hiding(&self) -> bool {
        self.random_v.is_some()
    }
}

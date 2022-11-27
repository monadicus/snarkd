use crate::{
    bls12_377::{scalar, Affine, G1Affine, G1Projective, G2Affine, G2Prepared, Projective, Scalar},
    fft::{DensePolynomial, EvaluationDomain},
    polycommit::powers::PowersOfG,
    utils::*,
};
use anyhow::Result;
use core::ops::{Add, AddAssign};
use parking_lot::RwLock;
use rand_core::RngCore;
use std::{borrow::Cow, collections::BTreeMap, io, ops::Range, sync::Arc};

/// `UniversalParams` are the universal parameters for the KZG10 scheme.
#[derive(Clone, Debug)]
pub struct UniversalParams {
    /// Group elements of the form `{ \beta^i G }`, where `i` ranges from 0 to `degree`,
    /// and group elements of the form `{ \beta^i \gamma G }`, where `i` ranges from 0 to `degree`.
    /// This struct provides an abstraction over the powers which are located on-disk
    /// to reduce memory usage.
    powers: Arc<RwLock<PowersOfG>>,
    /// The generator of G2.
    pub h: G2Affine,
    /// Supported degree bounds.
    supported_degree_bounds: Vec<usize>,
    /// The generator of G2, prepared for use in pairings.
    pub prepared_h: G2Prepared,
    /// \beta times the above generator of G2, prepared for use in pairings.
    pub prepared_beta_h: G2Prepared,
}

impl UniversalParams {
    pub fn load() -> Result<Self> {
        let powers = Arc::new(RwLock::new(PowersOfG::load()?));
        let h = G2Affine::prime_subgroup_generator();
        let prepared_h = G2Prepared::from_affine(h);
        let beta_h = (*powers.read()).beta_h();
        let prepared_beta_h = G2Prepared::from_affine(beta_h);
        let supported_degree_bounds = vec![1 << 10, 1 << 15, 1 << 20, 1 << 25, 1 << 30];

        Ok(Self {
            powers,
            h,
            supported_degree_bounds,
            prepared_h,
            prepared_beta_h,
        })
    }

    pub fn download_powers_for(&self, range: Range<usize>) -> Result<()> {
        self.powers.write().download_powers_for(range)
    }

    pub fn lagrange_basis(&self, domain: EvaluationDomain) -> Result<Vec<G1Affine>> {
        let mut basis = domain.ifft_projective(
            &self
                .powers_of_beta_g(0, domain.size())?
                .iter()
                .map(|e| (*e).to_projective())
                .collect::<Vec<_>>(),
        );
        G1Projective::batch_normalization(&mut basis);
        Ok(basis.iter().map(|e| (*e).into()).collect())
    }

    pub fn power_of_beta_g(&self, which_power: usize) -> Result<G1Affine> {
        self.powers.write().power_of_beta_g(which_power)
    }

    pub fn powers_of_beta_g(&self, lower: usize, upper: usize) -> Result<Vec<G1Affine>> {
        Ok(self.powers.write().powers_of_beta_g(lower..upper)?.to_vec())
    }

    pub fn powers_of_beta_times_gamma_g(&self) -> BTreeMap<usize, G1Affine> {
        self.powers.read().powers_of_beta_gamma_g()
    }

    pub fn beta_h(&self) -> G2Affine {
        self.powers.read().beta_h()
    }

    pub fn neg_powers_of_beta_h(&self) -> BTreeMap<usize, G2Affine> {
        self.powers.read().negative_powers_of_beta_h()
    }

    pub fn max_degree(&self) -> usize {
        self.powers.read().max_num_powers() - 1
    }

    pub fn supported_degree_bounds(&self) -> &[usize] {
        &self.supported_degree_bounds
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
        let supported_bits = scalar::MODULUS_BITS;

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

/// `KZGCommitment` commits to a polynomial. It is output by `KZG10::commit`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct KZGCommitment(
    /// The commitment is a group element.
    pub G1Affine,
);

impl KZGCommitment {
    #[inline]
    pub fn empty() -> Self {
        KZGCommitment(G1Affine::ZERO)
    }

    pub fn has_degree_bound(&self) -> bool {
        false
    }

    pub fn is_in_correct_subgroup_assuming_on_curve(&self) -> bool {
        self.0.is_in_correct_subgroup_assuming_on_curve()
    }
}

/// `PreparedKZGCommitment` commits to a polynomial and prepares for mul_bits.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct PreparedKZGCommitment(
    /// The commitment is a group element.
    pub Vec<G1Affine>,
);

impl PreparedKZGCommitment {
    /// prepare `PreparedKZGCommitment` from `KZGCommitment`
    pub fn prepare(comm: &KZGCommitment) -> Self {
        let mut prepared_comm = Vec::<G1Affine>::new();
        let mut cur = G1Projective::from(comm.0);

        let supported_bits = scalar::MODULUS_BITS;

        for _ in 0..supported_bits {
            prepared_comm.push(cur.into());
            cur.double_in_place();
        }

        Self(prepared_comm)
    }
}

/// `KZGRandomness` hides the polynomial inside a commitment. It is output by `KZG10::commit`.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct KZGRandomness {
    /// For KZG10, the commitment randomness is a random polynomial.
    pub blinding_polynomial: DensePolynomial,
}

impl KZGRandomness {
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

impl KZGRandomness {
    pub fn empty() -> Self {
        Self {
            blinding_polynomial: DensePolynomial::zero(),
        }
    }

    pub fn rand<R: RngCore>(hiding_bound: usize, _: bool, rng: &mut R) -> Self {
        let mut randomness = KZGRandomness::empty();
        let hiding_poly_degree = Self::calculate_hiding_polynomial_degree(hiding_bound);
        randomness.blinding_polynomial = DensePolynomial::rand(hiding_poly_degree, rng);
        randomness
    }
}

impl<'a> Add<&'a KZGRandomness> for KZGRandomness {
    type Output = Self;

    #[inline]
    fn add(mut self, other: &'a Self) -> Self {
        self.blinding_polynomial += &other.blinding_polynomial;
        self
    }
}

impl<'a> Add<(Scalar, &'a KZGRandomness)> for KZGRandomness {
    type Output = Self;

    #[inline]
    fn add(mut self, other: (Scalar, &'a KZGRandomness)) -> Self {
        self += other;
        self
    }
}

impl<'a> AddAssign<&'a KZGRandomness> for KZGRandomness {
    #[inline]
    fn add_assign(&mut self, other: &'a Self) {
        self.blinding_polynomial += &other.blinding_polynomial;
    }
}

impl<'a> AddAssign<(Scalar, &'a KZGRandomness)> for KZGRandomness {
    #[inline]
    fn add_assign(&mut self, (f, other): (Scalar, &'a KZGRandomness)) {
        self.blinding_polynomial += (f, &other.blinding_polynomial);
    }
}

/// `KZGProof` is an evaluation proof that is output by `KZG10::open`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct KZGProof {
    /// This is a commitment to the witness polynomial; see [\[KZG10\]][kzg] for more details.
    ///
    /// [kzg]: http://cacr.uwaterloo.ca/techreports/2010/cacr2010-10.pdf
    pub w: G1Affine,
    /// This is the evaluation of the random polynomial at the point for which
    /// the evaluation proof was produced.
    pub random_v: Option<Scalar>,
}

impl KZGProof {
    pub fn absorb_into_sponge(&self, sponge: &mut PoseidonSponge) {
        sponge.absorb_native_field_elements(&[self.w.x, self.w.y]);
        if let Some(random_v) = self.random_v {
            sponge.absorb_nonnative_field_elements([random_v]);
        }
    }
}

impl KZGProof {
    pub fn is_hiding(&self) -> bool {
        self.random_v.is_some()
    }
}

use std::sync::Arc;

use crate::{
    bls12_377::Scalar,
    fft::{
        domain::{FFTPrecomputation, IFFTPrecomputation},
        DensePolynomial, EvaluationDomain, Evaluations as EvaluationsOnDomain,
    },
    marlin::{
        ahp::{indexer::Circuit, verifier},
        AHPError,
    },
};
use anyhow::anyhow;

/// State for the AHP prover.
pub struct State<'a> {
    pub(super) index: &'a Circuit,

    /// A domain that is sized for the public input.
    pub(super) input_domain: EvaluationDomain,

    /// A domain that is sized for the number of constraints.
    pub(super) constraint_domain: EvaluationDomain,

    /// A domain that is sized for the number of non-zero elements in A.
    pub(super) non_zero_a_domain: EvaluationDomain,
    /// A domain that is sized for the number of non-zero elements in B.
    pub(super) non_zero_b_domain: EvaluationDomain,
    /// A domain that is sized for the number of non-zero elements in C.
    pub(super) non_zero_c_domain: EvaluationDomain,

    /// The number of instances being proved in this batch.
    pub(in crate::marlin) batch_size: usize,

    /// The list of public inputs for each instance in the batch.
    /// The length of this list must be equal to the batch size.
    pub(super) padded_public_variables: Vec<Vec<Scalar>>,

    /// The list of private variables for each instance in the batch.
    /// The length of this list must be equal to the batch size.
    pub(super) private_variables: Vec<Vec<Scalar>>,

    /// The list of Az vectors for each instance in the batch.
    /// The length of this list must be equal to the batch size.
    pub(super) z_a: Option<Vec<Vec<Scalar>>>,

    /// The list of Bz vectors for each instance in the batch.
    /// The length of this list must be equal to the batch size.
    pub(super) z_b: Option<Vec<Vec<Scalar>>>,

    /// A list of polynomials corresponding to the interpolation of the public input.
    /// The length of this list must be equal to the batch size.
    pub(super) x_poly: Vec<DensePolynomial>,

    /// The first round oracles sent by the prover.
    /// The length of this list must be equal to the batch size.
    pub(in crate::marlin) first_round_oracles: Option<Arc<super::FirstOracles<'a>>>,

    /// Randomizers for z_b.
    /// The length of this list must be equal to the batch size.
    pub(super) mz_poly_randomizer: Option<Vec<Scalar>>,

    /// The challenges sent by the verifier in the first round
    pub(super) verifier_first_message: Option<verifier::FirstMessage>,

    /// Polynomials involved in the holographic sumcheck.
    pub(super) lhs_polynomials: Option<[DensePolynomial; 3]>,
    /// Polynomials involved in the holographic sumcheck.
    pub(super) sums: Option<[Scalar; 3]>,
}

impl<'a> State<'a> {
    pub fn initialize(
        padded_public_input: Vec<Vec<Scalar>>,
        private_variables: Vec<Vec<Scalar>>,
        index: &'a Circuit,
    ) -> Result<Self, AHPError> {
        let index_info = &index.index_info;
        let constraint_domain = EvaluationDomain::new(index_info.num_constraints)
            .ok_or(anyhow!("polynomial degree too large"))?;

        let non_zero_a_domain = EvaluationDomain::new(index_info.num_non_zero_a)
            .ok_or(anyhow!("polynomial degree too large"))?;
        let non_zero_b_domain = EvaluationDomain::new(index_info.num_non_zero_b)
            .ok_or(anyhow!("polynomial degree too large"))?;
        let non_zero_c_domain = EvaluationDomain::new(index_info.num_non_zero_c)
            .ok_or(anyhow!("polynomial degree too large"))?;

        let input_domain = EvaluationDomain::new(padded_public_input[0].len())
            .ok_or(anyhow!("polynomial degree too large"))?;

        let x_poly = padded_public_input
            .iter()
            .map(|padded_public_input| {
                EvaluationsOnDomain::from_vec_and_domain(padded_public_input.clone(), input_domain)
                    .interpolate()
            })
            .collect();
        let batch_size = private_variables.len();
        assert_eq!(padded_public_input.len(), batch_size);

        Ok(Self {
            index,
            input_domain,
            constraint_domain,
            non_zero_a_domain,
            non_zero_b_domain,
            non_zero_c_domain,
            batch_size,
            padded_public_variables: padded_public_input,
            x_poly,
            private_variables,
            z_a: None,
            z_b: None,
            first_round_oracles: None,
            mz_poly_randomizer: None,
            verifier_first_message: None,
            lhs_polynomials: None,
            sums: None,
        })
    }

    /// Get the batch size.
    pub fn batch_size(&self) -> usize {
        self.batch_size
    }

    /// Get the public inputs for the entire batch.
    pub fn public_inputs(&self) -> Vec<Vec<Scalar>> {
        self.padded_public_variables
            .iter()
            .map(|v| super::ConstraintSystem::unformat_public_input(v))
            .collect()
    }

    /// Get the padded public inputs for the entire batch.
    pub fn padded_public_inputs(&self) -> Vec<Vec<Scalar>> {
        self.padded_public_variables.clone()
    }

    pub fn fft_precomputation(&self) -> &FFTPrecomputation {
        &self.index.fft_precomputation
    }

    pub fn ifft_precomputation(&self) -> &IFFTPrecomputation {
        &self.index.ifft_precomputation
    }
}

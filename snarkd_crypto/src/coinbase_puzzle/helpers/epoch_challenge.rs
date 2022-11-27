use super::*;
use crate::{coinbase_puzzle::hash_to_polynomial, fft::Evaluations as EvaluationsOnDomain};
use anyhow::{anyhow, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EpochChallenge {
    /// The epoch number.
    epoch_number: u32,
    /// The epoch block hash, defined as the block hash right before the epoch updated.
    epoch_block_hash: Scalar,
    /// The epoch polynomial.
    epoch_polynomial: DensePolynomial,
    /// The evaluations of the epoch polynomial over the product domain.
    epoch_polynomial_evaluations: EvaluationsOnDomain,
}

impl EpochChallenge {
    /// Initializes a new epoch challenge.
    pub fn new(epoch_number: u32, epoch_block_hash: Scalar, degree: u32) -> Result<Self> {
        // Construct the 'input' as '( epoch_number || epoch_block_hash )'
        let input: Vec<u8> = epoch_number
            .to_le_bytes()
            .into_iter()
            .chain(epoch_block_hash.0.to_le_bytes::<32>().into_iter())
            .collect();

        let product_domain = CoinbasePuzzle::product_domain(degree)?;

        let epoch_polynomial = hash_to_polynomial(&input, degree);
        if u32::try_from(epoch_polynomial.degree()).is_err() {
            return Err(anyhow!("Degree is too large"));
        }

        let epoch_polynomial_evaluations =
            epoch_polynomial.evaluate_over_domain_by_ref(product_domain);
        // Returns the epoch challenge.
        Ok(EpochChallenge {
            epoch_number,
            epoch_block_hash,
            epoch_polynomial,
            epoch_polynomial_evaluations,
        })
    }

    /// Returns the epoch number for the solution.
    pub const fn epoch_number(&self) -> u32 {
        self.epoch_number
    }

    /// Returns the epoch block hash for the solution.
    pub const fn epoch_block_hash(&self) -> Scalar {
        self.epoch_block_hash
    }

    /// Returns the epoch polynomial for the solution.
    pub const fn epoch_polynomial(&self) -> &DensePolynomial {
        &self.epoch_polynomial
    }

    /// Returns the evaluations of the epoch polynomial over the product domain.
    pub const fn epoch_polynomial_evaluations(&self) -> &EvaluationsOnDomain {
        &self.epoch_polynomial_evaluations
    }

    /// Returns the number of coefficients of the epoch polynomial.
    pub fn degree(&self) -> u32 {
        // Convert the degree into a u32.
        // The `unwrap` is guaranteed to succeed as we check the degree is less
        // than `u32::MAX` in `new`.
        u32::try_from(self.epoch_polynomial.degree()).unwrap()
    }

    /// Returns the number of coefficients of the epoch polynomial.
    pub fn num_coefficients(&self) -> Result<u32> {
        let degree = self.degree();
        degree
            .checked_add(1)
            .ok_or_else(|| anyhow!("Epoch polynomial degree ({degree} + 1) overflows"))
    }
}

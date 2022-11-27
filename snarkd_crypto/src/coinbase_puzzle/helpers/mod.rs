mod coinbase_solution;
pub use coinbase_solution::*;

mod epoch_challenge;
pub use epoch_challenge::*;

mod partial_solution;
pub use partial_solution::*;

mod prover_solution;
pub use prover_solution::*;

mod puzzle_commitment;
pub use puzzle_commitment::*;

use crate::coinbase_puzzle::{hash_commitment, hash_commitments, CoinbasePuzzle};
use crate::{
    bls12_377::{G1Affine, Scalar},
    fft::{domain::FFTPrecomputation, DensePolynomial, EvaluationDomain},
    keys::Address,
    polycommit::kzg10::{KZGCommitment, KZGProof, LagrangeBasis, VerifierKey, KZG10},
};
use anyhow::Result;
use std::{
    borrow::Cow,
    io::{Read, Result as IoResult, Write},
};

/// The proof of opening the polynomial, for the solution.
pub type PuzzleProof = KZGProof;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PuzzleConfig {
    /// The maximum degree of the polynomial.
    pub degree: u32,
}

pub type CoinbaseVerifyingKey = VerifierKey;

#[derive(Clone, Debug)]
pub struct CoinbaseProvingKey {
    /// The key used to commit to polynomials in Lagrange basis.
    pub lagrange_basis_at_beta_g: Vec<G1Affine>,
    /// Domain used to compute the product of the epoch polynomial and the prover polynomial.
    pub product_domain: EvaluationDomain,
    /// Precomputation to speed up FFTs.
    pub fft_precomputation: FFTPrecomputation,
    /// Elements of the product domain.
    pub product_domain_elements: Vec<Scalar>,
    /// The verifying key of the coinbase puzzle.
    pub verifying_key: CoinbaseVerifyingKey,
}

impl CoinbaseProvingKey {
    /// Obtain elements of the SRS in the lagrange basis powers.
    pub fn lagrange_basis(&self) -> LagrangeBasis {
        LagrangeBasis {
            lagrange_basis_at_beta_g: Cow::Borrowed(self.lagrange_basis_at_beta_g.as_slice()),
            powers_of_beta_times_gamma_g: Cow::Owned(vec![]),
            domain: self.product_domain,
        }
    }

    /// Returns the elements of the product domain.
    pub fn product_domain_elements(&self) -> &[Scalar] {
        &self.product_domain_elements
    }
}

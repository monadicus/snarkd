use crate::{snark::marlin::ahp::AHPError, SNARKError};

use core::fmt::Debug;

/// A `enum` specifying the possible failure modes of `Marlin`.
#[derive(Debug)]
pub enum MarlinError {
    /// The index is too large for the universal public parameters.
    IndexTooLarge(usize, usize),
    /// There was an error in the underlying holographic IOP.
    AHPError(AHPError),
    /// There was a synthesis error.
    R1CSError(snarkvm_r1cs::SynthesisError),
    /// There was an error in the underlying polynomial commitment.
    PolynomialCommitmentError(crate::polycommit::PCError),
    Terminated,
}

impl From<AHPError> for MarlinError {
    fn from(err: AHPError) -> Self {
        MarlinError::AHPError(err)
    }
}

impl From<snarkvm_r1cs::SynthesisError> for MarlinError {
    fn from(err: snarkvm_r1cs::SynthesisError) -> Self {
        MarlinError::R1CSError(err)
    }
}

impl From<crate::polycommit::PCError> for MarlinError {
    fn from(err: crate::polycommit::PCError) -> Self {
        match err {
            crate::polycommit::PCError::Terminated => MarlinError::Terminated,
            err => MarlinError::PolynomialCommitmentError(err),
        }
    }
}

impl From<MarlinError> for SNARKError {
    fn from(error: MarlinError) -> Self {
        match error {
            MarlinError::Terminated => SNARKError::Terminated,
            err => SNARKError::Crate("marlin", format!("{:?}", err)),
        }
    }
}

impl From<AHPError> for SNARKError {
    fn from(err: AHPError) -> Self {
        MarlinError::AHPError(err).into()
    }
}

impl From<crate::polycommit::PCError> for SNARKError {
    fn from(err: crate::polycommit::PCError) -> Self {
        match err {
            crate::polycommit::PCError::Terminated => MarlinError::Terminated,
            err => MarlinError::PolynomialCommitmentError(err),
        }
        .into()
    }
}

//! A crate for the Marlin preprocessing zkSNARK for R1CS.
//!
//! # Note
//!
//! Currently, Marlin only supports R1CS instances where the number of inputs
//! is the same as the number of constraints (i.e., where the constraint
//! matrices are square). Furthermore, Marlin only supports instances where the
//! public inputs are of size one less than a power of 2 (i.e., 2^n - 1).
#![forbid(unsafe_code)]
#![allow(clippy::module_inception)]
#![allow(clippy::type_complexity)]

use crate::{
    bls12_377::ToScalar, sonic_pc::UniversalParams, ConstraintSynthesizer, PoseidonParameters,
};
use core::{borrow::Borrow, sync::atomic::AtomicBool};
use rand::Rng;
use rand_core::CryptoRng;
use thiserror::Error;

/// Implements an Algebraic Holographic Proof (AHP) for the R1CS indexed relation.
pub mod ahp;
pub use ahp::*;

pub(crate) mod data_structures;
pub use data_structures::*;

/// Errors.
mod errors;
pub use errors::*;

/// Implements the base Marlin zkSNARK proof system.
mod marlin;
pub use marlin::*;

/// Specifies the Marlin mode.
mod mode;
pub use mode::*;

#[cfg(test)]
pub mod tests;

#[derive(Debug, Error)]
pub enum SNARKError {
    #[error("{}", _0)]
    AnyhowError(#[from] anyhow::Error),

    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("Expected a circuit-specific SRS in SNARK")]
    ExpectedCircuitSpecificSRS,

    #[error("{}", _0)]
    Message(String),

    #[error("Batch size was zero; must be at least 1")]
    EmptyBatch,

    #[error("terminated")]
    Terminated,
}

/// Defines a trait that describes preparing from an unprepared version to a prepare version.
pub trait Prepare {
    type Prepared;
    fn prepare(&self) -> Self::Prepared;
}

/// An abstraction layer to enable a circuit-specific SRS or universal SRS.
/// Forward compatible with future assumptions that proof systems will require.
pub enum SRS<'a, T> {
    CircuitSpecific,
    Universal(&'a T),
}

pub trait SNARK {
    fn universal_setup(config: usize) -> Result<UniversalParams, SNARKError>;

    fn setup<C: ConstraintSynthesizer>(
        circuit: &C,
        srs: &mut SRS<UniversalParams>,
        mode: bool,
    ) -> Result<(CircuitProvingKey, CircuitVerifyingKey), SNARKError>;

    fn prove_vk(
        fs_parameters: &PoseidonParameters,
        verifying_key: &CircuitVerifyingKey,
        proving_key: &CircuitProvingKey,
    ) -> Result<Certificate, SNARKError>;

    fn prove_batch<C: ConstraintSynthesizer, R: Rng + CryptoRng>(
        &self,
        fs_parameters: &PoseidonParameters,
        proving_key: &CircuitProvingKey,
        input_and_witness: &[C],
        rng: &mut R,
    ) -> Result<Proof, SNARKError> {
        self.prove_batch_with_terminator(
            fs_parameters,
            proving_key,
            input_and_witness,
            &AtomicBool::new(false),
            rng,
        )
    }

    fn prove<C: ConstraintSynthesizer, R: Rng + CryptoRng>(
        &self,
        fs_parameters: &PoseidonParameters,
        proving_key: &CircuitProvingKey,
        input_and_witness: &C,
        rng: &mut R,
    ) -> Result<Proof, SNARKError> {
        self.prove_batch(
            fs_parameters,
            proving_key,
            std::slice::from_ref(input_and_witness),
            rng,
        )
    }

    fn prove_batch_with_terminator<C: ConstraintSynthesizer, R: Rng + CryptoRng>(
        &self,
        fs_parameters: &PoseidonParameters,
        proving_key: &CircuitProvingKey,
        input_and_witness: &[C],
        terminator: &AtomicBool,
        rng: &mut R,
    ) -> Result<Proof, SNARKError>;

    fn prove_with_terminator<C: ConstraintSynthesizer, R: Rng + CryptoRng>(
        &self,
        fs_parameters: &PoseidonParameters,
        proving_key: &CircuitProvingKey,
        input_and_witness: &C,
        terminator: &AtomicBool,
        rng: &mut R,
    ) -> Result<Proof, SNARKError> {
        self.prove_batch_with_terminator(
            fs_parameters,
            proving_key,
            std::slice::from_ref(input_and_witness),
            terminator,
            rng,
        )
    }

    fn verify_vk<C: ConstraintSynthesizer>(
        fs_parameters: &PoseidonParameters,
        circuit: &C,
        verifying_key: &CircuitVerifyingKey,
        certificate: &Certificate,
    ) -> Result<bool, SNARKError>;

    fn verify_batch_prepared<TS: ToScalar + ?Sized, B: Borrow<TS>>(
        &self,
        fs_parameters: &PoseidonParameters,
        prepared_verifying_key: &<CircuitVerifyingKey as Prepare>::Prepared,
        input: &[B],
        proof: &Proof,
    ) -> Result<bool, SNARKError>;

    fn verify_batch<TS: ToScalar + ?Sized, B: Borrow<TS>>(
        &self,
        fs_parameters: &PoseidonParameters,
        verifying_key: &CircuitVerifyingKey,
        input: &[B],
        proof: &Proof,
    ) -> Result<bool, SNARKError> {
        let processed_verifying_key = verifying_key.prepare();
        self.verify_batch_prepared(fs_parameters, &processed_verifying_key, input, proof)
    }

    fn verify<TS: ToScalar + ?Sized, B: Borrow<TS>>(
        &self,
        fs_parameters: &PoseidonParameters,
        verifying_key: &CircuitVerifyingKey,
        input: B,
        proof: &Proof,
    ) -> Result<bool, SNARKError> {
        self.verify_batch(fs_parameters, verifying_key, &[input], proof)
    }
}

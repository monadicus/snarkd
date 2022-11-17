use crate::{
    fft::EvaluationDomain,
    polycommit::sonic_pc,
    snark::marlin::{ahp::indexer::*, CircuitProvingKey, MarlinMode, PreparedCircuitVerifyingKey},
    Prepare,
};
use snarkvm_curves::PairingEngine;
use snarkvm_fields::{ConstraintFieldError, ToConstraintField};
use snarkvm_r1cs::SynthesisError;
use snarkvm_utilities::{
    error,
    io::{self, Read, Write},
    serialize::*,
    string::String,
};

use anyhow::Result;
use core::{fmt, marker::PhantomData, str::FromStr};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// Verification key for a specific index (i.e., R1CS matrices).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitVerifyingKey {
    /// Stores information about the size of the circuit, as well as its defined field.
    pub circuit_info: CircuitInfo,
    /// Commitments to the indexed polynomials.
    pub circuit_commitments: Vec<sonic_pc::Commitment>,
    /// The verifier key for this index, trimmed from the universal SRS.
    pub verifier_key: sonic_pc::VerifierKey,
}

impl Prepare for CircuitVerifyingKey {
    type Prepared = PreparedCircuitVerifyingKey;

    /// Prepare the circuit verifying key.
    fn prepare(&self) -> Self::Prepared {
        let constraint_domain_size =
            EvaluationDomain::compute_size_of_domain(self.circuit_info.num_constraints).unwrap()
                as u64;
        let non_zero_a_domain_size =
            EvaluationDomain::compute_size_of_domain(self.circuit_info.num_non_zero_a).unwrap()
                as u64;
        let non_zero_b_domain_size =
            EvaluationDomain::compute_size_of_domain(self.circuit_info.num_non_zero_b).unwrap()
                as u64;
        let non_zero_c_domain_size =
            EvaluationDomain::compute_size_of_domain(self.circuit_info.num_non_zero_b).unwrap()
                as u64;

        PreparedCircuitVerifyingKey {
            constraint_domain_size,
            non_zero_a_domain_size,
            non_zero_b_domain_size,
            non_zero_c_domain_size,
            orig_vk: (*self).clone(),
        }
    }
}

impl From<CircuitProvingKey> for CircuitVerifyingKey {
    fn from(other: CircuitProvingKey) -> Self {
        other.circuit_verifying_key
    }
}

impl From<&'a CircuitProvingKey> for CircuitVerifyingKey {
    fn from(other: &'a CircuitProvingKey) -> Self {
        other.circuit_verifying_key.clone()
    }
}

impl From<PreparedCircuitVerifyingKey> for CircuitVerifyingKey {
    fn from(other: PreparedCircuitVerifyingKey) -> Self {
        other.orig_vk
    }
}

impl CircuitVerifyingKey {
    /// Iterate over the commitments to indexed polynomials in `self`.
    pub fn iter(&self) -> impl Iterator<Item = &sonic_pc::Commitment> {
        self.circuit_commitments.iter()
    }
}

impl CircuitVerifyingKey {
    fn to_field_elements(&self) -> Result<Vec<E::Fq>, ConstraintFieldError> {
        let constraint_domain_size =
            EvaluationDomain::<E::Fr>::compute_size_of_domain(self.circuit_info.num_constraints)
                .unwrap() as u128;
        let non_zero_a_domain_size =
            EvaluationDomain::<E::Fr>::compute_size_of_domain(self.circuit_info.num_non_zero_a)
                .unwrap() as u128;
        let non_zero_b_domain_size =
            EvaluationDomain::<E::Fr>::compute_size_of_domain(self.circuit_info.num_non_zero_b)
                .unwrap() as u128;
        let non_zero_c_domain_size =
            EvaluationDomain::<E::Fr>::compute_size_of_domain(self.circuit_info.num_non_zero_c)
                .unwrap() as u128;

        let mut res = Vec::new();
        res.append(&mut E::Fq::from(constraint_domain_size).to_field_elements()?);
        res.append(&mut E::Fq::from(non_zero_a_domain_size).to_field_elements()?);
        res.append(&mut E::Fq::from(non_zero_b_domain_size).to_field_elements()?);
        res.append(&mut E::Fq::from(non_zero_c_domain_size).to_field_elements()?);
        for comm in self.circuit_commitments.iter() {
            res.append(&mut comm.to_field_elements()?);
        }

        // Intentionally ignore the appending of the PC verifier key.

        Ok(res)
    }
}

impl FromStr for CircuitVerifyingKe> {
    type Err = anyhow::Error;

    #[inline]
    fn from_str(vk_hex: &str) -> Result<Self, Self::Err> {
        Self::from_bytes_le(&hex::decode(vk_hex)?)
    }
}

impl fmt::Display for CircuitVerifyingKey {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vk_hex = hex::encode(
            self.to_bytes_le()
                .expect("Failed to convert verifying key to bytes"),
        );
        write!(f, "{}", vk_hex)
    }
}

impl Serialize for CircuitVerifyingKey {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match serializer.is_human_readable() {
            true => serializer.collect_str(self),
            false => ToBytesSerializer::serialize_with_size_encoding(self, serializer),
        }
    }
}

impl<'de> Deserialize<'de> for CircuitVerifyingKey {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match deserializer.is_human_readable() {
            true => {
                let s: String = Deserialize::deserialize(deserializer)?;
                FromStr::from_str(&s).map_err(de::Error::custom)
            }
            false => FromBytesDeserializer::<Self>::deserialize_with_size_encoding(
                deserializer,
                "verifying key",
            ),
        }
    }
}

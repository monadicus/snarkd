use crate::{bls12_377::Scalar, fft::DensePolynomial, polycommit::kzg10::KZGCommitment, utils::*};
use anyhow::{anyhow, Result};
use blake2::Digest;
use rayon::prelude::*;
use ruint::Uint;

pub fn hash_to_coefficients(input: &[u8], num_coefficients: u32) -> Vec<Scalar> {
    // Hash the input.
    let hash = blake2::Blake2s256::digest(input);
    // Hash with a counter and return the coefficients.
    cfg_into_iter!(0..num_coefficients)
        .map(|counter| {
            let mut input_with_counter = [0u8; 36];
            input_with_counter[..32].copy_from_slice(&hash);
            input_with_counter[32..].copy_from_slice(&counter.to_le_bytes());
            let digest = blake2::Blake2b512::digest(input_with_counter);
            let mut arr = [0u8; 32];
            arr.copy_from_slice(&digest[..32]);
            Scalar(Uint::from_le_bytes(arr))
        })
        .collect()
}

pub fn hash_to_polynomial(input: &[u8], degree: u32) -> DensePolynomial {
    // Hash the input into coefficients.
    let coefficients = hash_to_coefficients(input, degree + 1);
    // Construct the polynomial from the coefficients.
    DensePolynomial::from_coefficients_vec(coefficients)
}

pub fn hash_commitment(commitment: &KZGCommitment) -> Result<Scalar> {
    // Convert the commitment into bytes.
    let mut bytes = Vec::with_capacity(96);
    bytes.extend(commitment.0.x.0.to_le_bytes::<48>().into_iter());
    bytes.extend(commitment.0.y.0.to_le_bytes::<48>().into_iter());
    if bytes.len() != 96 {
        return Err(anyhow!("Invalid commitment byte length for hashing"));
    }

    let digest = blake2::Blake2b512::digest(&bytes);
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&digest[..32]);
    // Return the hash of the commitment.
    Ok(Scalar(Uint::from_le_bytes(arr)))
}

pub fn hash_commitments(
    commitments: impl ExactSizeIterator<Item = KZGCommitment>,
) -> Result<Vec<Scalar>> {
    // Retrieve the number of commitments.
    let num_commitments = match u32::try_from(commitments.len()) {
        Ok(num_commitments) => num_commitments,
        Err(_) => {
            return Err(anyhow!(
                "Cannot hash more than 2^32 commitments: found {}",
                commitments.len()
            ))
        }
    };
    if num_commitments == 0 {
        return Err(anyhow!("No commitments provided for hashing"));
    }

    // Convert the commitments into bytes.
    let bytes = commitments
        .flat_map(|commitment| {
            let mut bytes = Vec::with_capacity(96);
            bytes.extend(commitment.0.x.0.to_le_bytes::<48>().into_iter());
            bytes.extend(commitment.0.y.0.to_le_bytes::<48>().into_iter());
            bytes
        })
        .collect::<Vec<_>>();
    if bytes.len() != 96 * usize::try_from(num_commitments)? {
        return Err(anyhow!("Invalid commitment byte length for hashing"));
    }

    // Hash the commitment bytes into coefficients.
    Ok(hash_to_coefficients(&bytes, num_commitments + 1))
}

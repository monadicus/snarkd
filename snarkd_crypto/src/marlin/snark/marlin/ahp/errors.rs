/// Describes the failure modes of the AHP scheme.
#[derive(Debug)]
pub enum AHPError {
    /// An error occurred during constraint generation.
    ConstraintSystemError(snarkvm_r1cs::errors::SynthesisError),
    /// The instance generated during proving does not match that in the index.
    InstanceDoesNotMatchIndex,
    /// The number of public inputs is incorrect.
    InvalidPublicInputLength,
    /// During verification, a required evaluation is missing
    MissingEval(String),
    /// Currently we only support square constraint matrices.
    NonSquareMatrix,
    /// During synthesis, our polynomials ended up being too high of degree
    PolynomialDegreeTooLarge,
}

impl From<snarkvm_r1cs::errors::SynthesisError> for AHPError {
    fn from(other: snarkvm_r1cs::errors::SynthesisError) -> Self {
        AHPError::ConstraintSystemError(other)
    }
}

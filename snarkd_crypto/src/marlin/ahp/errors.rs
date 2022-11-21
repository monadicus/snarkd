/// Describes the failure modes of the AHP scheme.
#[derive(Debug)]
pub enum AHPError {
    /// An error occurred during constraint generation.
    ConstraintSystemError(anyhow::Error),
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

impl From<anyhow::Error> for AHPError {
    fn from(other: anyhow::Error) -> Self {
        AHPError::ConstraintSystemError(other)
    }
}

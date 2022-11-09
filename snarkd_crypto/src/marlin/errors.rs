pub type SynthesisResult<T> = Result<T, SynthesisError>;

/// This is an error that could occur during circuit synthesis contexts,
/// such as CRS generation, proving or verification.
#[derive(Debug, Error)]
pub enum SynthesisError {
    #[error("{}", _0)]
    AnyhowError(#[from] anyhow::Error),
    /// During synthesis, we lacked knowledge of a variable assignment.
    #[error("An assignment for a variable could not be computed")]
    AssignmentMissing,
    /// Handles a failed conversion of objects into constraint field elements.
    #[error("Failed to convert object into constraint field elements")]
    ConstraintFieldError(#[from] snarkvm_fields::ConstraintFieldError),
    /// During synthesis, we divided by zero.
    #[error("Division by zero during synthesis")]
    DivisionByZero,
    /// During synthesis, we constructed an unsatisfiable constraint system.
    #[error("Unsatisfiable constraint system")]
    Unsatisfiable,
    /// During synthesis, our polynomials ended up being too high of degree
    #[error("Polynomial degree is too large")]
    PolynomialDegreeTooLarge,
    /// During proof generation, we encountered an identity in the CRS
    #[error("Encountered an identity element in the CRS")]
    UnexpectedIdentity,
    /// During proof generation, we encountered an I/O error with the CRS
    #[error("Encountered an I/O error")]
    IoError(std::io::Error),
    /// During verification, our verifying key was malformed.
    #[error(
        "Malformed verifying key, public input count was {} but expected {}",
        _0,
        _1
    )]
    MalformedVerifyingKey(usize, usize),
    /// During CRS generation, we observed an unconstrained auxiliary variable
    #[error("Auxiliary variable was unconstrained")]
    UnconstrainedVariable,
}

impl From<std::io::Error> for SynthesisError {
    fn from(e: std::io::Error) -> SynthesisError {
        SynthesisError::IoError(e)
    }
}

#[derive(Debug, Error)]
pub enum SNARKError {
    #[error("{}", _0)]
    AnyhowError(#[from] anyhow::Error),

    #[error("{}", _0)]
    ConstraintFieldError(#[from] ConstraintFieldError),

    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("Expected a circuit-specific SRS in SNARK")]
    ExpectedCircuitSpecificSRS,

    #[error("{}", _0)]
    Message(String),

    #[error("{}", _0)]
    SynthesisError(SynthesisError),

    #[error("Batch size was zero; must be at least 1")]
    EmptyBatch,

    #[error("terminated")]
    Terminated,
}

impl From<SynthesisError> for SNARKError {
    fn from(error: SynthesisError) -> Self {
        SNARKError::SynthesisError(error)
    }
}

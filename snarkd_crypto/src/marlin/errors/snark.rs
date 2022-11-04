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

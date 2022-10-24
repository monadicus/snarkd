#[derive(Debug, Error)]
pub enum ConstraintFieldError {
    #[error("{}", _0)]
    AnyhowError(#[from] anyhow::Error),

    #[error("{}: {}", _0, _1)]
    Crate(&'static str, String),

    #[error("{}", _0)]
    Message(&'static str),
}

impl From<std::io::Error> for ConstraintFieldError {
    fn from(error: std::io::Error) -> Self {
        ConstraintFieldError::Crate("std::io", format!("{:?}", error))
    }
}

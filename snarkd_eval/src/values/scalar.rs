use std::fmt;

use crate::Todo;

#[derive(Clone, Debug)]
pub struct ConstrainedScalar(Todo);

impl fmt::Display for ConstrainedScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "scalar")
    }
}

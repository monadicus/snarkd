use super::*;
use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
pub struct ConstrainedField<F: Field>(pub F);

impl<F: Field> Display for ConstrainedField<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "field")
    }
}

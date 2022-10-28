use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
pub struct FieldType<F>(F);


impl<F> Display for FieldType<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "field")
    }
}
use std::fmt::{Debug, Display};

#[derive(Clone, Debug)]
pub struct ConstrainedBool(pub bool);

impl Display for ConstrainedBool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "boolean")
    }
}

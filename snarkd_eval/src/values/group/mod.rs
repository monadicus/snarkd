use std::fmt::{Debug, Display};

use snarkd_crypto::Parameters;

#[derive(Clone, Debug)]
pub struct ConstrainedGroup<G: Parameters>(pub G);

impl<G: Parameters> Display for ConstrainedGroup<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "group")
    }
}

use std::fmt::{Debug, Display};

use snarkd_crypto::Group;

#[derive(Clone, Debug)]
pub struct ConstrainedGroup<G: Group>(pub G);

impl<G: Group> Display for ConstrainedGroup<G> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "group")
    }
}

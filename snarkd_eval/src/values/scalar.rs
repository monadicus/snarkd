use std::fmt;

use snarkd_crypto::Scalar;

#[derive(Clone, Debug)]
pub struct ConstrainedScalar(Scalar);

impl fmt::Display for ConstrainedScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "scalar")
    }
}

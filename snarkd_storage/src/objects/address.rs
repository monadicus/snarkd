use snarkd_common::Digest;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Address(pub Digest);

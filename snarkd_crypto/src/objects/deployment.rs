use super::{Certificate, Identifier, Program, VerifyingKey};
use indexmap::IndexMap;

#[derive(Clone, PartialEq, Eq)]
pub struct Deployment {
    pub edition: u16,
    pub program: Program,
    pub verifying_keys: IndexMap<Identifier, (VerifyingKey, Certificate)>,
}

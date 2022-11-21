use super::{Identifier, Locator, PlaintextType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RegisterType {
    Plaintext(PlaintextType),
    Record(Identifier),
    ExternalRecord(Locator),
}

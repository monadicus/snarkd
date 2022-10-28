use super::{Identifier, Locator, PlaintextType};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum RegisterType {
    Plaintext(PlaintextType),
    Record(Identifier),
    ExternalRecord(Locator),
}

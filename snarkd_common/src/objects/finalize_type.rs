use super::{Identifier, Locator, PlaintextType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinalizeType {
    Public(PlaintextType),
    Record(Identifier),
    ExternalRecord(Locator),
}

use super::{Identifier, Locator, PlaintextType};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ValueType {
    Constant(PlaintextType),
    Public(PlaintextType),
    Private(PlaintextType),
    Record(Identifier),
    ExternalRecord(Locator),
}

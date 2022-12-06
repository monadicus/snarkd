use super::PlaintextType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryType {
    Constant(PlaintextType),
    Public(PlaintextType),
    Private(PlaintextType),
}

use super::{Identifier, ProgramID};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Locator {
    pub id: ProgramID,
    pub resource: Identifier,
}

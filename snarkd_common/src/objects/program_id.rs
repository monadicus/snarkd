use super::Identifier;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProgramID {
    pub name: Identifier,
    pub network: Identifier,
}

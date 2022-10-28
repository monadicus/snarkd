use super::Identifier;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ProgramID {
    pub name: Identifier,
    pub network: Identifier,
}

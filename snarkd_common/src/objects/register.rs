use super::Identifier;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Register {
    Locator(u64),
    Member(u64, Vec<Identifier>),
}

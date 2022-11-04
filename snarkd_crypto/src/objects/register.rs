use super::Identifier;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Register {
    Locator(u64),
    Member(u64, Vec<Identifier>),
}

use super::{Identifier, LiteralType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlaintextType {
    Literal(LiteralType),
    Interface(Identifier),
}

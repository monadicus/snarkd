use super::{FinalizeType, Identifier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapObject {
    pub name: Identifier,
    pub finalize_type: FinalizeType,
}

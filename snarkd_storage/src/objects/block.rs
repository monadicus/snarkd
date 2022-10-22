use super::{SerialBlockHeader, SerialTransaction};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerialBlock {
    pub header: SerialBlockHeader,
    pub transactions: Vec<SerialTransaction>,
}

use super::Record;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Plaintext(Vec<u8>),
    Record(Box<Record>),
}

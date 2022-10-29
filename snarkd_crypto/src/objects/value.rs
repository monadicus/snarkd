use super::Record;

#[derive(Clone, PartialEq, Eq)]
pub enum Value {
    Plaintext(Vec<u8>),
    Record(Record),
}

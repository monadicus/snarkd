use super::{Register, ValueType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValueEntry {
    pub register: Register,
    pub register_type: ValueType,
}

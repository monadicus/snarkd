use super::{Register, ValueType};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ValueEntry {
    register: Register,
    register_type: ValueType,
}

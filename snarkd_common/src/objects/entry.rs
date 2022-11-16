use super::{Register, RegisterType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entry {
    pub register: Register,
    pub register_type: RegisterType,
}

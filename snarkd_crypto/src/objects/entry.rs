use super::{Register, RegisterType};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Entry {
    register: Register,
    register_type: RegisterType,
}

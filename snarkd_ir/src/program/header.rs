use crate::{ir, Error, InputType, Result};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Header {
    pub snarkd_major: u32,
    pub snarkd_minor: u32,
    pub snarkd_patch: u32,
    pub constant_inputs: Vec<InputType>,
    pub private_inputs: Vec<InputType>,
    pub public_inputs: Vec<InputType>,
    pub register_inputs: Vec<InputType>,
    pub public_states: Vec<InputType>,
    pub private_record_states: Vec<InputType>,
    pub private_leaf_states: Vec<InputType>,
}

impl TryFrom<ir::Header> for Header {
    type Error = Error;

    fn try_from(value: ir::Header) -> Result<Self, Self::Error> {
        let get_inputs = |inputs: Vec<ir::InputType>| -> Result<Vec<InputType>> {
            inputs.into_iter().map(|i| i.try_into()).collect()
        };
        Ok(Self {
            snarkd_major: value.snarkd_major,
            snarkd_minor: value.snarkd_minor,
            snarkd_patch: value.snarkd_patch,
            constant_inputs: get_inputs(value.constant_inputs)?,
            private_inputs: get_inputs(value.private_inputs)?,
            public_inputs: get_inputs(value.public_inputs)?,
            public_states: get_inputs(value.public_states)?,
            register_inputs: get_inputs(value.register_inputs)?,
            private_record_states: get_inputs(value.private_record_states)?,
            private_leaf_states: get_inputs(value.private_leaf_states)?,
        })
    }
}

impl From<Header> for ir::Header {
    fn from(value: Header) -> Self {
        let get_inputs = |inputs: Vec<InputType>| -> Vec<ir::InputType> {
            inputs.into_iter().map(|i| i.into()).collect()
        };
        Self {
            snarkd_major: value.snarkd_major,
            snarkd_minor: value.snarkd_minor,
            snarkd_patch: value.snarkd_patch,
            constant_inputs: get_inputs(value.constant_inputs),
            private_inputs: get_inputs(value.private_inputs),
            public_inputs: get_inputs(value.public_inputs),
            register_inputs: get_inputs(value.register_inputs),
            public_states: get_inputs(value.public_states),
            private_record_states: get_inputs(value.private_record_states),
            private_leaf_states: get_inputs(value.private_leaf_states),
        }
    }
}

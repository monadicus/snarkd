use crate::{ir, Type};

use anyhow::{anyhow, Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    pub variable: u32,
    pub name: String,
    pub type_: Type,
}

impl TryFrom<ir::Input> for Input {
    type Error = Error;

    fn try_from(value: ir::Input) -> Result<Self> {
        Ok(Self {
            variable: value.variable,
            type_: value
                .r#type
                .ok_or_else(|| anyhow!("no type set on input `{}`", value.name))?
                .try_into()?,
            name: value.name,
        })
    }
}

impl From<Input> for ir::Input {
    fn from(value: Input) -> Self {
        Self {
            variable: value.variable,
            name: value.name,
            r#type: Some(value.type_.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Header {
    pub snarkd_major: u32,
    pub snarkd_minor: u32,
    pub snarkd_patch: u32,
    pub main_inputs: Vec<Input>,
    pub constant_inputs: Vec<Input>,
    pub register_inputs: Vec<Input>,
    pub public_states: Vec<Input>,
    pub private_record_states: Vec<Input>,
    pub private_leaf_states: Vec<Input>,
}

impl TryFrom<ir::Header> for Header {
    type Error = Error;

    fn try_from(value: ir::Header) -> Result<Self, Self::Error> {
        let get_inputs = |inputs: Vec<ir::Input>| -> Result<Vec<Input>> {
            inputs.into_iter().map(|i| i.try_into()).collect()
        };
        Ok(Self {
            snarkd_major: value.snarkd_major,
            snarkd_minor: value.snarkd_minor,
            snarkd_patch: value.snarkd_patch,
            main_inputs: get_inputs(value.main_inputs)?,
            constant_inputs: get_inputs(value.constant_inputs)?,
            register_inputs: get_inputs(value.register_inputs)?,
            public_states: get_inputs(value.public_states)?,
            private_record_states: get_inputs(value.private_record_states)?,
            private_leaf_states: get_inputs(value.private_leaf_states)?,
        })
    }
}

impl From<Header> for ir::Header {
    fn from(value: Header) -> Self {
        let get_inputs = |inputs: Vec<Input>| -> Vec<ir::Input> {
            inputs.into_iter().map(|i| i.into()).collect()
        };
        Self {
            snarkd_major: value.snarkd_major,
            snarkd_minor: value.snarkd_minor,
            snarkd_patch: value.snarkd_patch,
            main_inputs: get_inputs(value.main_inputs),
            constant_inputs: get_inputs(value.constant_inputs),
            register_inputs: get_inputs(value.register_inputs),
            public_states: get_inputs(value.public_states),
            private_record_states: get_inputs(value.private_record_states),
            private_leaf_states: get_inputs(value.private_leaf_states),
        }
    }
}

use snarkd_errors::IRError;

use crate::{ir, Error, Operand, Result, Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputType {
    pub variable: u32,
    pub name: String,
    pub type_: Type,
}

impl TryFrom<ir::InputType> for InputType {
    type Error = Error;

    fn try_from(value: ir::InputType) -> Result<Self> {
        Ok(Self {
            variable: value.variable,
            type_: value
                .r#type
                .ok_or_else(|| IRError::unset("Input type"))?
                .try_into()?,
            name: value.name,
        })
    }
}

impl From<InputType> for ir::InputType {
    fn from(value: InputType) -> Self {
        Self {
            variable: value.variable,
            name: value.name,
            r#type: Some(value.type_.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputValue {
    name: String,
    value: Operand,
}

impl TryFrom<ir::InputValue> for InputValue {
    type Error = Error;

    fn try_from(value: ir::InputValue) -> Result<Self> {
        Ok(Self {
            name: value.name,
            value: value
                .value
                .ok_or_else(|| IRError::unset("Input value"))?
                .try_into()?,
        })
    }
}

impl From<InputValue> for ir::InputValue {
    fn from(value: InputValue) -> Self {
        Self {
            name: value.name,
            value: Some(value.value.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputData {
    pub constant_inputs: Vec<InputValue>,
    pub private_inputs: Vec<InputValue>,
    pub public_inputs: Vec<InputValue>,
    pub register_inputs: Vec<InputValue>,
    pub public_states: Vec<InputValue>,
    pub private_record_states: Vec<InputValue>,
    pub private_leaf_states: Vec<InputValue>,
}

impl TryFrom<ir::InputData> for InputData {
    type Error = Error;

    fn try_from(value: ir::InputData) -> Result<Self, Self::Error> {
        let get_inputs = |inputs: Vec<ir::InputValue>| -> Result<Vec<InputValue>> {
            inputs.into_iter().map(|i| i.try_into()).collect()
        };
        Ok(Self {
            constant_inputs: get_inputs(value.constants)?,
            private_inputs: get_inputs(value.privates)?,
            public_inputs: get_inputs(value.publics)?,
            register_inputs: get_inputs(value.registers)?,
            public_states: get_inputs(value.public_state)?,
            private_record_states: get_inputs(value.private_record_state)?,
            private_leaf_states: get_inputs(value.private_leaf_state)?,
        })
    }
}

impl From<InputData> for ir::InputData {
    fn from(value: InputData) -> Self {
        let get_inputs = |inputs: Vec<InputValue>| -> Vec<ir::InputValue> {
            inputs.into_iter().map(|i| i.into()).collect()
        };
        Self {
            constants: get_inputs(value.constant_inputs),
            privates: get_inputs(value.private_inputs),
            publics: get_inputs(value.public_inputs),
            registers: get_inputs(value.register_inputs),
            public_state: get_inputs(value.public_states),
            private_record_state: get_inputs(value.private_record_states),
            private_leaf_state: get_inputs(value.private_leaf_states),
        }
    }
}

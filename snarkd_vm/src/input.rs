use std::fmt;

use crate::{
    ir::{self, ProtoBuf},
    Type, Value,
};

use anyhow::*;
use indexmap::IndexMap;
use prost::Message;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Input {
    pub variable: u32,
    pub name: String,
    pub type_: Type,
}

impl ProtoBuf for Input {
    type Target = ir::Input;

    fn decode(input: Self::Target) -> Result<Self> {
        Ok(Self {
            variable: input.variable,
            name: input.name,
            type_: Type::decode(
                input
                    .r#type
                    .ok_or_else(|| anyhow!("missing type for input"))?,
            )?,
        })
    }

    fn encode(&self) -> Self::Target {
        ir::Input {
            variable: self.variable,
            name: self.name.clone(),
            r#type: Some(self.type_.encode()),
        }
    }
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}: {}", self.variable, self.name, self.type_)
    }
}

/// Concrete input data
#[derive(Clone, Debug, Default)]
pub struct InputData {
    pub main: IndexMap<String, Value>,
    pub constants: IndexMap<String, Value>,
    pub registers: IndexMap<String, Value>,
    pub public_states: IndexMap<String, Value>,
    pub private_record_states: IndexMap<String, Value>,
    pub private_leaf_states: IndexMap<String, Value>,
}

fn encode_map_key((key, value): (&String, &Value)) -> ir::InputDataItem {
    ir::InputDataItem {
        name: key.clone(),
        value: Some(value.encode()),
    }
}

fn decode_map_key(data: ir::InputDataItem) -> Result<(String, Value)> {
    Ok((
        data.name,
        Value::decode(
            data.value
                .ok_or_else(|| anyhow!("missing value from input data"))?,
        )?,
    ))
}

impl InputData {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut out = vec![];
        self.encode().encode(&mut out)?;
        Ok(out)
    }

    pub fn deserialize(input: &[u8]) -> Result<Self> {
        Self::decode(ir::InputData::decode(input)?)
    }

    pub(crate) fn decode(input: ir::InputData) -> Result<Self> {
        Ok(Self {
            main: input
                .main
                .into_iter()
                .map(decode_map_key)
                .collect::<Result<_>>()?,
            constants: input
                .constants
                .into_iter()
                .map(decode_map_key)
                .collect::<Result<_>>()?,
            registers: input
                .registers
                .into_iter()
                .map(decode_map_key)
                .collect::<Result<_>>()?,
            public_states: input
                .public_state
                .into_iter()
                .map(decode_map_key)
                .collect::<Result<_>>()?,
            private_record_states: input
                .private_record_state
                .into_iter()
                .map(decode_map_key)
                .collect::<Result<_>>()?,
            private_leaf_states: input
                .private_leaf_state
                .into_iter()
                .map(decode_map_key)
                .collect::<Result<_>>()?,
        })
    }

    pub(crate) fn encode(&self) -> ir::InputData {
        ir::InputData {
            main: self.main.iter().map(encode_map_key).collect(),
            constants: self.constants.iter().map(encode_map_key).collect(),
            registers: self.registers.iter().map(encode_map_key).collect(),
            public_state: self.public_states.iter().map(encode_map_key).collect(),
            private_record_state: self
                .private_record_states
                .iter()
                .map(encode_map_key)
                .collect(),
            private_leaf_state: self
                .private_leaf_states
                .iter()
                .map(encode_map_key)
                .collect(),
        }
    }
}

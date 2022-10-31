use std::fmt;

use crate::{
    ir::{self, ProtoBuf},
    Type,
};

use anyhow::*;
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

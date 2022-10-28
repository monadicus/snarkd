use std::fmt;

use crate::ir::{self, ProtoBuf};
use anyhow::Result;

use serde::Serialize;

/// An unbounded field value
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Field {
    pub negate: bool,
    pub values: Vec<u64>,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let negate = if self.negate { "-" } else { "" };
        write!(f, "{}{:?}", negate, self.values)
    }
}

impl ProtoBuf for Field {
    type Target = ir::Field;

    fn decode(from: Self::Target) -> Result<Self> {
        Ok(Self {
            negate: from.negate,
            values: from.values,
        })
    }

    fn encode(&self) -> Self::Target {
        Self::Target {
            negate: self.negate,
            values: self.values.clone(),
        }
    }
}

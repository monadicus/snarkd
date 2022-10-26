use std::fmt;

use crate::{ir, Value};

use anyhow::{anyhow, Result};
use serde::Serialize;

pub type AssertData = PredicateData<2>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PredicateData<const N: usize> {
    pub values: Vec<Value>,
}

impl<const N: usize> fmt::Display for PredicateData<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.values.first() {
            write!(f, "{}", value)?;
        }
        for value in self.values.iter().skip(1) {
            write!(f, ", {}", value)?;
        }
        Ok(())
    }
}

impl<const N: usize> PredicateData<N> {
    pub(crate) fn decode(operands: Vec<ir::Operand>) -> Result<Self> {
        if operands.len() != N {
            return Err(anyhow!(
                "illegal operand count for PredicateData: {}",
                operands.len()
            ));
        }
        Ok(Self {
            values: operands
                .into_iter()
                .map(Value::decode)
                .collect::<Result<Vec<Value>>>()?,
        })
    }

    pub(crate) fn encode(&self) -> Vec<ir::Operand> {
        self.values.iter().map(|x| x.encode()).collect()
    }
}

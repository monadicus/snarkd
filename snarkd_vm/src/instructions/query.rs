use std::fmt;

use crate::{ir, Value};

use anyhow::{anyhow, Result};
use serde::Serialize;

use super::decode_control_u32;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct QueryData<const N: usize> {
    pub destination: u32,
    pub values: Vec<Value>,
}

impl<const N: usize> fmt::Display for QueryData<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "&v{}", self.destination)?;
        for value in &self.values {
            write!(f, ", {}", value)?;
        }
        Ok(())
    }
}

pub type UnaryData = QueryData<1>;
pub type BinaryData = QueryData<2>;

impl<const N: usize> QueryData<N> {
    pub(crate) fn decode(operands: Vec<ir::Operand>) -> Result<Self> {
        if operands.len() != N + 1 {
            return Err(anyhow!(
                "illegal operand count for VarData: {}",
                operands.len()
            ));
        }
        let mut operands = operands.into_iter();
        let destination = decode_control_u32(operands.next().unwrap())?;
        let values = operands
            .map(Value::decode)
            .collect::<Result<Vec<Value>>>()?;
        Ok(Self {
            destination,
            values,
        })
    }

    pub(crate) fn encode(&self) -> Vec<ir::Operand> {
        let mut operands = vec![];
        operands.push(ir::Operand {
            u32: Some(ir::U32 {
                u32: self.destination,
            }),
            ..Default::default()
        });
        for value in self.values.iter() {
            operands.push(value.encode());
        }
        operands
    }
}
use std::fmt;

use anyhow::{anyhow, bail, Result};
use serde::Serialize;

use crate::{
    ir::{self, ProtoBuf},
    visibility::Visibility,
    Integer, Value,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct VisibleData {
    pub value: Box<Value>,
    pub visibility: Visibility,
}

impl ProtoBuf for VisibleData {
    type Target = ir::Data;

    fn decode(from: Self::Target) -> Result<Self> {
        Ok(Self {
            visibility: Visibility::decode(from.visibility())?,
            value: Box::new(Value::decode(
                *from.value.ok_or_else(|| anyhow!("no value specified"))?,
            )?),
        })
    }

    fn encode(&self) -> Self::Target {
        Self::Target {
            value: Some(Box::new(self.value.encode())),
            visibility: self.visibility.encode() as i32,
        }
    }
}

impl fmt::Display for VisibleData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.value, self.visibility)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Record {
    /// address
    pub owner: VisibleData,
    /// u64
    pub gates: VisibleData,
    /// any type
    pub data: Vec<VisibleData>,
    /// group
    pub nonce: VisibleData,
}

impl ProtoBuf for Record {
    type Target = ir::Record;

    fn decode(from: ir::Record) -> Result<Self> {
        let owner = VisibleData::decode(*from.owner.ok_or_else(|| anyhow!("no value specified"))?)?;
        if !matches!(*owner.value, Value::Address(_)) {
            bail!("owner must be an address");
        }
        let gates = VisibleData::decode(*from.gates.ok_or_else(|| anyhow!("no value specified"))?)?;
        if !matches!(*gates.value, Value::Integer(Integer::U64(_))) {
            bail!("gates must be a u64");
        }
        let nonce = VisibleData::decode(*from.nonce.ok_or_else(|| anyhow!("no value specified"))?)?;
        if !matches!(*nonce.value, Value::Group(_)) {
            bail!("nonce must be a group");
        }

        Ok(Self {
            owner,
            gates,
            data: from
                .data
                .into_iter()
                .map(VisibleData::decode)
                .collect::<Result<Vec<_>>>()?,
            nonce,
        })
    }

    fn encode(&self) -> ir::Record {
        let data = self
            .data
            .clone()
            .into_iter()
            .map(|d| d.encode())
            .collect::<Vec<_>>();
        ir::Record {
            owner: Some(Box::new(self.owner.encode())),
            gates: Some(Box::new(self.gates.encode())),
            data,
            nonce: Some(Box::new(self.nonce.encode())),
        }
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "record(owner: {}, gates: {}, data: (",
            self.owner, self.gates
        )?;
        for (i, item) in self.data.iter().enumerate() {
            write!(
                f,
                "{item}{}",
                if i == self.data.len() - 1 { "" } else { ", " }
            )?;
        }
        write!(f, "), nonce: {})", self.nonce)
    }
}
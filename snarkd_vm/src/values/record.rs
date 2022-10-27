use std::fmt;

use anyhow::{anyhow, bail, Result};
use serde::Serialize;

use crate::{ir, visibility::Visibility, Integer, Value};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Data {
    pub value: Value,
    pub visibility: Visibility,
}

impl Data {
    pub fn decode(from: ir::Data) -> Result<Self> {
        Ok(Self {
            visibility: Visibility::decode(from.visibility())?,
            value: Value::decode(*from.value.ok_or_else(|| anyhow!("no value specified"))?)?,
        })
    }

    pub fn encode(&self) -> ir::Data {
        ir::Data {
            value: Some(Box::new(self.value.encode())),
            visibility: self.visibility.encode() as i32,
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.value, self.visibility)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Record {
    /// address
    pub owner: Data,
    /// u64
    pub gates: Data,
    /// any
    pub data: Vec<Data>,
    /// u64
    pub nonce: Data,
}

impl Record {
    pub fn decode(from: ir::Record) -> Result<Box<Self>> {
        let owner = Data::decode(*from.owner.ok_or_else(|| anyhow!("no value specified"))?)?;
        if !matches!(owner.value, Value::Address(_)) {
            bail!("owner must be an address");
        }
        let gates = Data::decode(*from.gates.ok_or_else(|| anyhow!("no value specified"))?)?;
        if !matches!(gates.value, Value::Integer(Integer::U64(_))) {
            bail!("owner must be an address");
        }
        let nonce = Data::decode(*from.nonce.ok_or_else(|| anyhow!("no value specified"))?)?;
        if !matches!(nonce.value, Value::Group(_)) {
            bail!("owner must be an address");
        }

        Ok(Box::new(Self {
            owner,
            gates,
            data: from
                .data
                .into_iter()
                .map(Data::decode)
                .collect::<Result<Vec<_>>>()?,
            nonce,
        }))
    }

    pub fn encode(&self) -> Box<ir::Record> {
        let data = self
            .data
            .clone()
            .into_iter()
            .map(|d| d.encode())
            .collect::<Vec<_>>();
        Box::new(ir::Record {
            owner: Some(Box::new(self.owner.encode())),
            gates: Some(Box::new(self.gates.encode())),
            data,
            nonce: Some(Box::new(self.nonce.encode())),
        })
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

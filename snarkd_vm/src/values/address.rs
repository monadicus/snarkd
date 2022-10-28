use std::fmt;

use anyhow::{anyhow, Result};
use bech32::ToBase32;
use serde::Serialize;

use crate::ir::{self, ProtoBuf};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Address {
    pub address: Vec<u8>,
}

impl ProtoBuf for Address {
    type Target = ir::Address;

    fn decode(address: Self::Target) -> Result<Self> {
        if address.address.is_empty() {
            Err(anyhow!("address can't be empty: {:?}", address))
        } else {
            Ok(Self {
                address: address.address,
            })
        }
    }

    fn encode(&self) -> Self::Target {
        ir::Address {
            address: self.address.clone(),
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            bech32::encode(
                "aleo",
                self.address.to_vec().to_base32(),
                bech32::Variant::Bech32
            )
            .unwrap_or_default()
        )
    }
}

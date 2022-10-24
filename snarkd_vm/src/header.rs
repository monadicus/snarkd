use crate::{ir, Input};

use anyhow::*;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SnarkdVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Default for SnarkdVersion {
    fn default() -> Self {
        Self {
            major: env!("CARGO_PKG_VERSION_MAJOR")
                .parse()
                .expect("invalid major version"),
            minor: env!("CARGO_PKG_VERSION_MINOR")
                .parse()
                .expect("invalid minor version"),
            patch: env!("CARGO_PKG_VERSION_PATCH")
                .parse()
                .expect("invalid patch version"),
        }
    }
}

impl SnarkdVersion {
    pub fn check_compatible(&self) -> bool {
        self == &Self::default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Header {
    pub version: SnarkdVersion,
    pub main_inputs: Vec<Input>,
    pub constant_inputs: Vec<Input>,
    pub register_inputs: Vec<Input>,
    pub public_states: Vec<Input>,
    pub private_record_states: Vec<Input>,
    pub private_leaf_states: Vec<Input>,
    pub inline_limit: u32,
}

impl Header {
    pub(crate) fn decode(header: ir::Header) -> Result<Self> {
        Ok(Self {
            version: SnarkdVersion {
                major: header.snarkd_major,
                minor: header.snarkd_minor,
                patch: header.snarkd_patch,
            },
            main_inputs: header
                .main_inputs
                .into_iter()
                .map(Input::decode)
                .collect::<Result<Vec<Input>>>()?,
            constant_inputs: header
                .constant_inputs
                .into_iter()
                .map(Input::decode)
                .collect::<Result<Vec<Input>>>()?,
            register_inputs: header
                .register_inputs
                .into_iter()
                .map(Input::decode)
                .collect::<Result<Vec<Input>>>()?,
            public_states: header
                .public_states
                .into_iter()
                .map(Input::decode)
                .collect::<Result<Vec<Input>>>()?,
            private_record_states: header
                .private_record_states
                .into_iter()
                .map(Input::decode)
                .collect::<Result<Vec<Input>>>()?,
            private_leaf_states: header
                .private_leaf_states
                .into_iter()
                .map(Input::decode)
                .collect::<Result<Vec<Input>>>()?,
            inline_limit: header.inline_limit,
        })
    }

    pub(crate) fn encode(&self) -> ir::Header {
        ir::Header {
            snarkd_major: self.version.major,
            snarkd_minor: self.version.minor,
            snarkd_patch: self.version.patch,
            main_inputs: self.main_inputs.iter().map(|x| x.encode()).collect(),
            constant_inputs: self.constant_inputs.iter().map(|x| x.encode()).collect(),
            register_inputs: self.register_inputs.iter().map(|x| x.encode()).collect(),
            public_states: self.public_states.iter().map(|x| x.encode()).collect(),
            private_record_states: self
                .private_record_states
                .iter()
                .map(|x| x.encode())
                .collect(),
            private_leaf_states: self
                .private_leaf_states
                .iter()
                .map(|x| x.encode())
                .collect(),
            inline_limit: self.inline_limit,
        }
    }
}

use std::{
    fmt::{self},
    io::{Read, Write},
    path::Path,
};

use prost::Message;

use crate::{
    ir::{self, ProtoBuf},
    Function, Header,
};

use anyhow::{anyhow, Result};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Program {
    pub header: Header,
    pub functions: Vec<Function>,
}

impl Program {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let mut out = vec![];
        self.encode().encode(&mut out)?;
        Ok(out)
    }

    pub fn deserialize(input: &[u8]) -> Result<Self> {
        Self::decode(ir::Program::decode(input)?)
    }

    pub fn from_read<R: Read>(buf: &mut R) -> Result<Self> {
        let mut tmp = Vec::new();
        buf.read_to_end(&mut tmp)?;
        Self::deserialize(&tmp)
    }

    pub fn read_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = std::fs::OpenOptions::new().read(true).open(path)?;
        Self::from_read(&mut file)
    }

    pub fn write_bytes<W: Write>(&self, buf: &mut W) -> Result<()> {
        let content = self.serialize()?;
        buf.write_all(&content)?;
        Ok(())
    }

    pub fn write_ops<W: Write>(&self, buf: &mut W) -> Result<()> {
        let content = format!("{}", self);
        buf.write_all(content.as_bytes())?;
        Ok(())
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for function in &self.functions {
            writeln!(f, "{function}")?;
        }
        Ok(())
    }
}

impl ProtoBuf for Program {
    type Target = ir::Program;

    fn encode(&self) -> Self::Target {
        ir::Program {
            header: Some(self.header.encode()),
            functions: self.functions.iter().map(|x| x.encode()).collect(),
        }
    }

    fn decode(program: Self::Target) -> Result<Self> {
        Ok(Self {
            header: Header::decode(
                program
                    .header
                    .ok_or_else(|| anyhow!("missing header for program"))?,
            )?,
            functions: program
                .functions
                .into_iter()
                .map(Function::decode)
                .collect::<Result<Vec<Function>>>()?,
        })
    }
}

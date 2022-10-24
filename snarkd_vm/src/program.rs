use std::{
    fmt::{self},
    io::{Read, Write},
    path::Path,
};

use prost::Message;

use crate::{ir, Function, Header};

use anyhow::{anyhow, Result};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Program {
    pub header: Header,
    pub functions: Vec<Function>,
}

impl Program {
    pub(crate) fn encode(&self) -> ir::Program {
        ir::Program {
            header: Some(self.header.encode()),
            functions: self.functions.iter().map(|x| x.encode()).collect(),
        }
    }

    pub(crate) fn decode(program: ir::Program) -> Result<Self> {
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
        for (i, function) in self.functions.iter().enumerate() {
            writeln!(f, "decl f{}: <{}>", i, function.argument_start_variable)?;
            for (i, instruction) in function.instructions.iter().enumerate() {
                write!(f, "{i}: ")?;
                instruction.fmt(f)?;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

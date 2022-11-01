mod header;
pub use header::*;

use std::{
    fmt,
    io::{Read, Write},
    mem,
    path::Path,
};

use crate::{ir, Instruction};

use anyhow::{anyhow, Error, Result};
use prost::Message;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub argument_start_variable: u32,
    pub instructions: Vec<Instruction>,
}

impl TryFrom<ir::Function> for Function {
    type Error = Error;

    fn try_from(value: ir::Function) -> Result<Self> {
        Ok(Self {
            argument_start_variable: value.argument_start_variable,
            instructions: value
                .instructions
                .into_iter()
                .map(|f| f.try_into())
                .collect::<Result<_>>()?,
        })
    }
}

impl From<Function> for ir::Function {
    fn from(value: Function) -> Self {
        Self {
            argument_start_variable: value.argument_start_variable,
            instructions: value.instructions.into_iter().map(|f| f.into()).collect(),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "decl f{}", self.argument_start_variable)?;
        for (i, instruction) in self.instructions.iter().enumerate() {
            writeln!(f, "{i}: {instruction}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Program {
    pub header: Header,
    pub functions: Vec<Function>,
}

impl Program {
    pub fn serialize(&mut self) -> Result<Vec<u8>> {
        let mut out = vec![];
        let converted = ir::Program::try_from(mem::take(self))?;
        converted.encode(&mut out)?;
        *self = converted.try_into().unwrap();
        Ok(out)
    }

    pub fn deserialize(input: &[u8]) -> Result<Self> {
        ir::Program::decode(input)?.try_into()
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

    pub fn write_bytes<W: Write>(&mut self, buf: &mut W) -> Result<()> {
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

impl TryFrom<ir::Program> for Program {
    type Error = Error;

    fn try_from(value: ir::Program) -> Result<Self, Self::Error> {
        Ok(Self {
            header: value
                .header
                .ok_or_else(|| anyhow!("missing header"))?
                .try_into()?,
            functions: value
                .functions
                .into_iter()
                .map(|f| f.try_into())
                .collect::<Result<_>>()?,
        })
    }
}

impl From<Program> for ir::Program {
    fn from(value: Program) -> Self {
        Self {
            header: Some(value.header.into()),
            functions: value.functions.into_iter().map(|f| f.into()).collect(),
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for function in &self.functions {
            function.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

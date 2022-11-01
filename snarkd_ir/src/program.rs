use std::{
    fmt,
    io::{Read, Write},
    mem,
    path::Path,
};

use crate::{ir, Instruction, Type};

use anyhow::{anyhow, Error, Result};
use prost::Message;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Input {
    pub variable: u32,
    pub name: String,
    pub type_: Type,
}

impl TryFrom<ir::Input> for Input {
    type Error = Error;

    fn try_from(value: ir::Input) -> Result<Self> {
        Ok(Self {
            variable: value.variable,
            type_: value
                .r#type
                .ok_or_else(|| anyhow!("no type set on input `{}`", value.name))?
                .try_into()?,
            name: value.name,
        })
    }
}

impl From<Input> for ir::Input {
    fn from(value: Input) -> Self {
        Self {
            variable: value.variable,
            name: value.name,
            r#type: Some(value.type_.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Header {
    pub snarkd_major: u32,
    pub snarkd_minor: u32,
    pub snarkd_patch: u32,
    pub main_inputs: Vec<Input>,
    pub constant_inputs: Vec<Input>,
    pub register_inputs: Vec<Input>,
    pub public_states: Vec<Input>,
    pub private_record_states: Vec<Input>,
    pub private_leaf_states: Vec<Input>,
}

impl TryFrom<ir::Header> for Header {
    type Error = Error;

    fn try_from(value: ir::Header) -> Result<Self, Self::Error> {
        let get_inputs = |inputs: Vec<ir::Input>| -> Result<Vec<Input>> {
            inputs.into_iter().map(|i| i.try_into()).collect()
        };
        Ok(Self {
            snarkd_major: value.snarkd_major,
            snarkd_minor: value.snarkd_minor,
            snarkd_patch: value.snarkd_patch,
            main_inputs: get_inputs(value.main_inputs)?,
            constant_inputs: get_inputs(value.constant_inputs)?,
            register_inputs: get_inputs(value.register_inputs)?,
            public_states: get_inputs(value.public_states)?,
            private_record_states: get_inputs(value.private_record_states)?,
            private_leaf_states: get_inputs(value.private_leaf_states)?,
        })
    }
}

impl From<Header> for ir::Header {
    fn from(value: Header) -> Self {
        let get_inputs = |inputs: Vec<Input>| -> Vec<ir::Input> {
            inputs.into_iter().map(|i| i.into()).collect()
        };
        Self {
            snarkd_major: value.snarkd_major,
            snarkd_minor: value.snarkd_minor,
            snarkd_patch: value.snarkd_patch,
            main_inputs: get_inputs(value.main_inputs),
            constant_inputs: get_inputs(value.constant_inputs),
            register_inputs: get_inputs(value.register_inputs),
            public_states: get_inputs(value.public_states),
            private_record_states: get_inputs(value.private_record_states),
            private_leaf_states: get_inputs(value.private_leaf_states),
        }
    }
}

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

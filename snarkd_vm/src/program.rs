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

#[cfg(test)]
mod tests {
    use crate::{
        BinaryData, Function, Header, Input, Instruction, Program, SnarkVMVersion, Type, Value,
    };

    fn example_program() -> Program {
        Program {
            header: Header {
                version: SnarkVMVersion::default(),
                main_inputs: vec![Input {
                    variable: 0,
                    name: "a".into(),
                    type_: Type::U8,
                }],
                constant_inputs: vec![Input {
                    variable: 1,
                    name: "b".into(),
                    type_: Type::U8,
                }],
                register_inputs: vec![Input {
                    variable: 2,
                    name: "c".into(),
                    type_: Type::U8,
                }],
                public_states: vec![Input {
                    variable: 3,
                    name: "d".into(),
                    type_: Type::U8,
                }],
                private_record_states: vec![Input {
                    variable: 4,
                    name: "e".into(),
                    type_: Type::U8,
                }],
                private_leaf_states: vec![Input {
                    variable: 5,
                    name: "f".into(),
                    type_: Type::U8,
                }],
                inline_limit: 1,
            },
            functions: vec![Function {
                argument_start_variable: 0,
                instructions: vec![Instruction::Add(BinaryData {
                    destination: 2,
                    values: vec![Value::Ref(0), Value::Ref(1)],
                })],
            }],
        }
    }

    #[test]
    fn encode_decode_test() {
        let input = example_program();
        let bytes = input.serialize().unwrap();
        let output = Program::deserialize(&bytes).unwrap();
        assert_eq!(input, output);
    }
}

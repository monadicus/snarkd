use std::fmt;

use crate::{
    ir::{self, ProtoBuf},
    Instruction,
};

use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Function {
    pub argument_start_variable: u32,
    pub instructions: Vec<Instruction>,
}

impl ProtoBuf for Function {
    type Target = ir::Function;

    fn decode(function: Self::Target) -> Result<Self> {
        Ok(Self {
            argument_start_variable: function.argument_start_variable,
            instructions: function
                .instructions
                .into_iter()
                .map(Instruction::decode)
                .collect::<Result<Vec<_>>>()?,
        })
    }

    fn encode(&self) -> Self::Target {
        ir::Function {
            argument_start_variable: self.argument_start_variable,
            instructions: self.instructions.iter().map(|x| x.encode()).collect(),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "decl f{}", self.argument_start_variable)?;
        for (i, instruction) in self.instructions.iter().enumerate() {
            write!(f, "{i}: ")?;
            instruction.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

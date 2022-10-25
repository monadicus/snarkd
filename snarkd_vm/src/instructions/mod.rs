use anyhow::{anyhow, Result};
use serde::Serialize;

mod code;
pub mod op;

use std::fmt;

mod query;
use crate::ir;
pub use query::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Instruction {
    Abs(BinaryData),
    AbsWrapped(BinaryData),
    Add(BinaryData),
    AddWrapped(BinaryData),
    And(BinaryData),
    AssertEq(BinaryData),
    AssertNeq(BinaryData),
    CommitBHP256(BinaryData),
    CommitBHP512(BinaryData),
    CommitBHP768(BinaryData),
    CommitBHP1024(BinaryData),
    CommitPED64(BinaryData),
    CommitPED128(BinaryData),
    Div(BinaryData),
    DivWrapped(BinaryData),
    Double(BinaryData),
    Gt(BinaryData),
    Gte(BinaryData),
    HashBHP256(BinaryData),
    HashBHP512(BinaryData),
    HashBHP768(BinaryData),
    HashBHP1024(BinaryData),
    HashPED64(BinaryData),
    HashPED128(BinaryData),
    HashPSD2(BinaryData),
    HashPSD4(BinaryData),
    HashPSD8(BinaryData),
    Inv(BinaryData),
    IsEq(BinaryData),
    Lt(BinaryData),
    Lte(BinaryData),
    Mod(BinaryData),
    Mul(BinaryData),
    MulWrapped(BinaryData),
    Nand(BinaryData),
    Neg(BinaryData),
    Nor(BinaryData),
    Not(BinaryData),
    Or(BinaryData),
    Pow(BinaryData),
    PowWrapped(BinaryData),
    Rem(BinaryData),
    RemWrapped(BinaryData),
    Shl(BinaryData),
    ShlWrapped(BinaryData),
    Shr(BinaryData),
    ShrWrapped(BinaryData),
    Sqrt(BinaryData),
    Square(BinaryData),
    Sub(BinaryData),
    SubWrapped(BinaryData),
    Ternary(BinaryData),
    Xor(BinaryData),
}

fn decode_control_u32(operand: ir::Operand) -> Result<u32> {
    match operand {
        ir::Operand { u32: Some(u32), .. } => Ok(u32.u32),
        _ => Err(anyhow!("illegal value for control operand: {:?}", operand)),
    }
}

// fn decode_control_bool(operand: ir::Operand) -> Result<bool> {
//     match operand {
//         ir::Operand {
//             boolean: Some(bool),
//             ..
//         } => Ok(bool.boolean),
//         _ => Err(anyhow!("illegal value for control operand: {:?}", operand)),
//     }
// }

// fn decode_control_string(operand: ir::Operand) -> Result<String> {
//     match operand {
//         ir::Operand {
//             string: Some(string),
//             ..
//         } => Ok(string.string),
//         _ => Err(anyhow!("illegal value for control operand: {:?}", operand)),
//     }
// }

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;
        write!(f, "{} ", self.opcode().mnemonic())?;
        match self {
            Abs(x) => x.fmt(f),
            AbsWrapped(x) => x.fmt(f),
            Add(x) => x.fmt(f),
            AddWrapped(x) => x.fmt(f),
            And(x) => x.fmt(f),
            AssertEq(x) => x.fmt(f),
            AssertNeq(x) => x.fmt(f),
            CommitBHP256(x) => x.fmt(f),
            CommitBHP512(x) => x.fmt(f),
            CommitBHP768(x) => x.fmt(f),
            CommitBHP1024(x) => x.fmt(f),
            CommitPED64(x) => x.fmt(f),
            CommitPED128(x) => x.fmt(f),
            Div(x) => x.fmt(f),
            DivWrapped(x) => x.fmt(f),
            Double(x) => x.fmt(f),
            Gt(x) => x.fmt(f),
            Gte(x) => x.fmt(f),
            HashBHP256(x) => x.fmt(f),
            HashBHP512(x) => x.fmt(f),
            HashBHP768(x) => x.fmt(f),
            HashBHP1024(x) => x.fmt(f),
            HashPED64(x) => x.fmt(f),
            HashPED128(x) => x.fmt(f),
            HashPSD2(x) => x.fmt(f),
            HashPSD4(x) => x.fmt(f),
            HashPSD8(x) => x.fmt(f),
            Inv(x) => x.fmt(f),
            IsEq(x) => x.fmt(f),
            Lt(x) => x.fmt(f),
            Lte(x) => x.fmt(f),
            Mod(x) => x.fmt(f),
            Mul(x) => x.fmt(f),
            MulWrapped(x) => x.fmt(f),
            Nand(x) => x.fmt(f),
            Neg(x) => x.fmt(f),
            Nor(x) => x.fmt(f),
            Not(x) => x.fmt(f),
            Or(x) => x.fmt(f),
            Pow(x) => x.fmt(f),
            PowWrapped(x) => x.fmt(f),
            Rem(x) => x.fmt(f),
            RemWrapped(x) => x.fmt(f),
            Shl(x) => x.fmt(f),
            ShlWrapped(x) => x.fmt(f),
            Shr(x) => x.fmt(f),
            ShrWrapped(x) => x.fmt(f),
            Sqrt(x) => x.fmt(f),
            Square(x) => x.fmt(f),
            Sub(x) => x.fmt(f),
            SubWrapped(x) => x.fmt(f),
            Ternary(x) => x.fmt(f),
            Xor(x) => x.fmt(f),
        }
    }
}

mod assert_data;
pub use assert_data::*;
mod binary_data;
pub use binary_data::*;
mod commit_data;
pub use commit_data::*;
mod hash_data;
pub use hash_data::*;
mod ternary_data;
pub use ternary_data::*;
mod unary_data;
pub use unary_data::*;

use std::fmt;

use super::ir;
use snarkd_errors::{Error, IRError, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Abs(UnaryData),
    AbsWrapped(UnaryData),
    Add(BinaryData),
    AddWrapped(BinaryData),
    And(BinaryData),
    AssertEq(AssertData),
    AssertNeq(AssertData),
    Commit(CommitData),
    Div(BinaryData),
    DivWrapped(BinaryData),
    Double(UnaryData),
    Gt(BinaryData),
    Gte(BinaryData),
    Hash(HashData),
    Inv(UnaryData),
    IsEq(BinaryData),
    IsNeq(BinaryData),
    Lt(BinaryData),
    Lte(BinaryData),
    Mod(BinaryData),
    Mul(BinaryData),
    MulWrapped(BinaryData),
    Nand(BinaryData),
    Neg(UnaryData),
    Nor(BinaryData),
    Not(UnaryData),
    Or(BinaryData),
    Pow(BinaryData),
    PowWrapped(BinaryData),
    Rem(BinaryData),
    RemWrapped(BinaryData),
    Shl(BinaryData),
    ShlWrapped(BinaryData),
    Shr(BinaryData),
    ShrWrapped(BinaryData),
    Sqrt(UnaryData),
    Square(UnaryData),
    Sub(BinaryData),
    SubWrapped(BinaryData),
    Ternary(TernaryData),
    Xor(BinaryData),
}

impl Instruction {
    pub const fn mnemonic(&self) -> &'static str {
        use Instruction::*;
        match self {
            Abs(_) => "abs",
            AbsWrapped(_) => "abs.w",
            Add(_) => "add",
            AddWrapped(_) => "add.w",
            And(_) => "and",
            AssertEq(_) => "assert.eq",
            AssertNeq(_) => "assert.neq",
            Commit(data) => match data.method {
                CommitMethod::CommitBhp256 => "commit.bhp256",
                CommitMethod::CommitBhp512 => "commit.bhp512",
                CommitMethod::CommitBhp768 => "commit.bhp768",
                CommitMethod::CommitBhp1024 => "commit.bhp1024",
                CommitMethod::CommitPed64 => "commit.ped64",
                CommitMethod::CommitPed128 => "commit.ped128",
            },
            Div(_) => "div",
            DivWrapped(_) => "div.wrapped",
            Double(_) => "double",
            Gt(_) => "gt",
            Gte(_) => "gte",
            Hash(data) => match data.method {
                HashMethod::HashBhp256 => "data.bhp256",
                HashMethod::HashBhp512 => "hash.hbp512",
                HashMethod::HashBhp768 => "hash.hbp768",
                HashMethod::HashBhp1024 => "hash.hbp1024",
                HashMethod::HashPed64 => "hash.ped64",
                HashMethod::HashPed128 => "hash.ped128",
                HashMethod::HashPsd2 => "hash.psd2",
                HashMethod::HashPsd4 => "hash.psd4",
                HashMethod::HashPsd8 => "hash.psd8",
            },
            Inv(_) => "inv",
            IsEq(_) => "is.eq",
            IsNeq(_) => "is.neq",
            Lt(_) => "lt",
            Lte(_) => "lte",
            Mod(_) => "mod",
            Mul(_) => "mul",
            MulWrapped(_) => "mul.w",
            Nand(_) => "nand",
            Neg(_) => "neg",
            Nor(_) => "nor",
            Not(_) => "not",
            Or(_) => "or",
            Pow(_) => "pow",
            PowWrapped(_) => "pow.w",
            Rem(_) => "rem",
            RemWrapped(_) => "rem.w",
            Shl(_) => "shl",
            ShlWrapped(_) => "shl.w",
            Shr(_) => "shr",
            ShrWrapped(_) => "shr.w",
            Sqrt(_) => "sqrt",
            Square(_) => "square",
            Sub(_) => "sub",
            SubWrapped(_) => "sub.w",
            Ternary(_) => "ternary",
            Xor(_) => "xor",
        }
    }
}

impl TryFrom<ir::Instruction> for Instruction {
    type Error = Error;

    fn try_from(value: ir::Instruction) -> Result<Self> {
        Ok(
            match value
                .instruction
                .ok_or_else(|| IRError::unset("Instruction"))?
            {
                ir::instruction::Instruction::Abs(v) => Self::Abs(v.try_into()?),
                ir::instruction::Instruction::AbsWrapped(v) => Self::AbsWrapped(v.try_into()?),
                ir::instruction::Instruction::Add(v) => Self::Add(v.try_into()?),
                ir::instruction::Instruction::AddWrapped(v) => Self::AddWrapped(v.try_into()?),
                ir::instruction::Instruction::And(v) => Self::And(v.try_into()?),
                ir::instruction::Instruction::AssertEq(v) => Self::AssertEq(v.try_into()?),
                ir::instruction::Instruction::AssertNeq(v) => Self::AssertNeq(v.try_into()?),
                ir::instruction::Instruction::Commit(v) => Self::Commit(v.try_into()?),
                ir::instruction::Instruction::Div(v) => Self::Div(v.try_into()?),
                ir::instruction::Instruction::DivWrapped(v) => Self::DivWrapped(v.try_into()?),
                ir::instruction::Instruction::Double(v) => Self::Double(v.try_into()?),
                ir::instruction::Instruction::Gt(v) => Self::Gt(v.try_into()?),
                ir::instruction::Instruction::Gte(v) => Self::Gte(v.try_into()?),
                ir::instruction::Instruction::Hash(v) => Self::Hash(v.try_into()?),
                ir::instruction::Instruction::Inv(v) => Self::Inv(v.try_into()?),
                ir::instruction::Instruction::IsEq(v) => Self::IsEq(v.try_into()?),
                ir::instruction::Instruction::IsNeq(v) => Self::IsNeq(v.try_into()?),
                ir::instruction::Instruction::Lt(v) => Self::Lt(v.try_into()?),
                ir::instruction::Instruction::Lte(v) => Self::Lte(v.try_into()?),
                ir::instruction::Instruction::Mod(v) => Self::Mod(v.try_into()?),
                ir::instruction::Instruction::Mul(v) => Self::Mul(v.try_into()?),
                ir::instruction::Instruction::MulWrapped(v) => Self::MulWrapped(v.try_into()?),
                ir::instruction::Instruction::Nand(v) => Self::Nand(v.try_into()?),
                ir::instruction::Instruction::Neg(v) => Self::Neg(v.try_into()?),
                ir::instruction::Instruction::Nor(v) => Self::Nor(v.try_into()?),
                ir::instruction::Instruction::Not(v) => Self::Not(v.try_into()?),
                ir::instruction::Instruction::Or(v) => Self::Or(v.try_into()?),
                ir::instruction::Instruction::Pow(v) => Self::Pow(v.try_into()?),
                ir::instruction::Instruction::PowWrapped(v) => Self::PowWrapped(v.try_into()?),
                ir::instruction::Instruction::Rem(v) => Self::Rem(v.try_into()?),
                ir::instruction::Instruction::RemWrapped(v) => Self::RemWrapped(v.try_into()?),
                ir::instruction::Instruction::Shl(v) => Self::Shl(v.try_into()?),
                ir::instruction::Instruction::ShlWrapped(v) => Self::ShlWrapped(v.try_into()?),
                ir::instruction::Instruction::Shr(v) => Self::Shr(v.try_into()?),
                ir::instruction::Instruction::ShrWrapped(v) => Self::ShrWrapped(v.try_into()?),
                ir::instruction::Instruction::Sqrt(v) => Self::Sqrt(v.try_into()?),
                ir::instruction::Instruction::Square(v) => Self::Square(v.try_into()?),
                ir::instruction::Instruction::Sub(v) => Self::Sub(v.try_into()?),
                ir::instruction::Instruction::SubWrapped(v) => Self::SubWrapped(v.try_into()?),
                ir::instruction::Instruction::Ternary(v) => Self::Ternary(v.try_into()?),
                ir::instruction::Instruction::Xor(v) => Self::Xor(v.try_into()?),
            },
        )
    }
}

impl From<Instruction> for ir::Instruction {
    fn from(value: Instruction) -> Self {
        Self {
            instruction: Some(match value {
                Instruction::Abs(v) => ir::instruction::Instruction::Abs(v.into()),
                Instruction::AbsWrapped(v) => ir::instruction::Instruction::AbsWrapped(v.into()),
                Instruction::Add(v) => ir::instruction::Instruction::Add(v.into()),
                Instruction::AddWrapped(v) => ir::instruction::Instruction::AddWrapped(v.into()),
                Instruction::And(v) => ir::instruction::Instruction::And(v.into()),
                Instruction::AssertEq(v) => ir::instruction::Instruction::AssertEq(v.into()),
                Instruction::AssertNeq(v) => ir::instruction::Instruction::AssertNeq(v.into()),
                Instruction::Commit(v) => ir::instruction::Instruction::Commit(v.into()),

                Instruction::Div(v) => ir::instruction::Instruction::Div(v.into()),
                Instruction::DivWrapped(v) => ir::instruction::Instruction::DivWrapped(v.into()),
                Instruction::Double(v) => ir::instruction::Instruction::Double(v.into()),
                Instruction::Gt(v) => ir::instruction::Instruction::Gt(v.into()),
                Instruction::Gte(v) => ir::instruction::Instruction::Gte(v.into()),
                Instruction::Hash(v) => ir::instruction::Instruction::Hash(v.into()),
                Instruction::Inv(v) => ir::instruction::Instruction::Inv(v.into()),
                Instruction::IsEq(v) => ir::instruction::Instruction::IsEq(v.into()),
                Instruction::IsNeq(v) => ir::instruction::Instruction::IsNeq(v.into()),
                Instruction::Lt(v) => ir::instruction::Instruction::Lt(v.into()),
                Instruction::Lte(v) => ir::instruction::Instruction::Lte(v.into()),
                Instruction::Mod(v) => ir::instruction::Instruction::Mod(v.into()),
                Instruction::Mul(v) => ir::instruction::Instruction::Mul(v.into()),
                Instruction::MulWrapped(v) => ir::instruction::Instruction::MulWrapped(v.into()),
                Instruction::Nand(v) => ir::instruction::Instruction::Nand(v.into()),
                Instruction::Neg(v) => ir::instruction::Instruction::Neg(v.into()),
                Instruction::Nor(v) => ir::instruction::Instruction::Nor(v.into()),
                Instruction::Not(v) => ir::instruction::Instruction::Not(v.into()),
                Instruction::Or(v) => ir::instruction::Instruction::Or(v.into()),
                Instruction::Pow(v) => ir::instruction::Instruction::Pow(v.into()),
                Instruction::PowWrapped(v) => ir::instruction::Instruction::PowWrapped(v.into()),
                Instruction::Rem(v) => ir::instruction::Instruction::Rem(v.into()),
                Instruction::RemWrapped(v) => ir::instruction::Instruction::RemWrapped(v.into()),
                Instruction::Shl(v) => ir::instruction::Instruction::Shl(v.into()),
                Instruction::ShlWrapped(v) => ir::instruction::Instruction::ShlWrapped(v.into()),
                Instruction::Shr(v) => ir::instruction::Instruction::Shr(v.into()),
                Instruction::ShrWrapped(v) => ir::instruction::Instruction::ShrWrapped(v.into()),
                Instruction::Sqrt(v) => ir::instruction::Instruction::Sqrt(v.into()),
                Instruction::Square(v) => ir::instruction::Instruction::Square(v.into()),
                Instruction::Sub(v) => ir::instruction::Instruction::Sub(v.into()),
                Instruction::SubWrapped(v) => ir::instruction::Instruction::SubWrapped(v.into()),
                Instruction::Ternary(v) => ir::instruction::Instruction::Ternary(v.into()),
                Instruction::Xor(v) => ir::instruction::Instruction::Xor(v.into()),
            }),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Instruction::*;
        write!(f, "{} ", self.mnemonic())?;
        match self {
            Abs(x) => x.fmt(f),
            AbsWrapped(x) => x.fmt(f),
            Add(x) => x.fmt(f),
            AddWrapped(x) => x.fmt(f),
            And(x) => x.fmt(f),
            AssertEq(x) => x.fmt(f),
            AssertNeq(x) => x.fmt(f),
            Commit(x) => x.fmt(f),
            Div(x) => x.fmt(f),
            DivWrapped(x) => x.fmt(f),
            Double(x) => x.fmt(f),
            Gt(x) => x.fmt(f),
            Gte(x) => x.fmt(f),
            Hash(x) => x.fmt(f),
            Inv(x) => x.fmt(f),
            IsEq(x) => x.fmt(f),
            IsNeq(x) => x.fmt(f),
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

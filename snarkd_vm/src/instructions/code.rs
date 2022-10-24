use num_enum::TryFromPrimitive;

use super::{op::InstructionOp, *};

impl Instruction {
    pub(crate) fn opcode(&self) -> InstructionOp {
        match self {
            Self::Abs(_) => InstructionOp::Abs,
            Self::AbsWrapped(_) => InstructionOp::AbsWrapped,
            Self::Add(_) => InstructionOp::Add,
            Self::AddWrapped(_) => InstructionOp::AddWrapped,
            Self::And(_) => InstructionOp::And,
            Self::AssertEq(_) => InstructionOp::AssertEq,
            Self::AssertNeq(_) => InstructionOp::AssertNeq,
            Self::CommitBHP256(_) => InstructionOp::CommitBHP256,
            Self::CommitBHP512(_) => InstructionOp::CommitBHP512,
            Self::CommitBHP768(_) => InstructionOp::CommitBHP768,
            Self::CommitBHP1024(_) => InstructionOp::CommitBHP1024,
            Self::CommitPED64(_) => InstructionOp::CommitPED64,
            Self::CommitPED128(_) => InstructionOp::CommitPED128,
            Self::Div(_) => InstructionOp::Div,
            Self::DivWrapped(_) => InstructionOp::DivWrapped,
            Self::Double(_) => InstructionOp::Double,
            Self::Gt(_) => InstructionOp::Gt,
            Self::Gte(_) => InstructionOp::Gte,
            Self::HashBHP256(_) => InstructionOp::HashBHP256,
            Self::HashBHP512(_) => InstructionOp::HashBHP512,
            Self::HashBHP768(_) => InstructionOp::HashBHP768,
            Self::HashBHP1024(_) => InstructionOp::HashBHP1024,
            Self::HashPED64(_) => InstructionOp::HashPED64,
            Self::HashPED128(_) => InstructionOp::HashPED128,
            Self::HashPSD2(_) => InstructionOp::HashPSD2,
            Self::HashPSD4(_) => InstructionOp::HashPSD4,
            Self::HashPSD8(_) => InstructionOp::HashPSD8,
            Self::Inv(_) => InstructionOp::Inv,
            Self::IsEq(_) => InstructionOp::IsEq,
            Self::Lt(_) => InstructionOp::Lt,
            Self::Lte(_) => InstructionOp::Lte,
            Self::Mod(_) => InstructionOp::Mod,
            Self::Mul(_) => InstructionOp::Mul,
            Self::MulWrapped(_) => InstructionOp::MulWrapped,
            Self::Nand(_) => InstructionOp::Nand,
            Self::Neg(_) => InstructionOp::Neg,
            Self::Nor(_) => InstructionOp::Nor,
            Self::Not(_) => InstructionOp::Not,
            Self::Or(_) => InstructionOp::Or,
            Self::Pow(_) => InstructionOp::Pow,
            Self::PowWrapped(_) => InstructionOp::PowWrapped,
            Self::Rem(_) => InstructionOp::Rem,
            Self::RemWrapped(_) => InstructionOp::RemWrapped,
            Self::Shl(_) => InstructionOp::Shl,
            Self::ShlWrapped(_) => InstructionOp::ShlWrapped,
            Self::Shr(_) => InstructionOp::Shr,
            Self::ShrWrapped(_) => InstructionOp::ShrWrapped,
            Self::Sqrt(_) => InstructionOp::Sqrt,
            Self::Square(_) => InstructionOp::Square,
            Self::Sub(_) => InstructionOp::Sub,
            Self::SubWrapped(_) => InstructionOp::SubWrapped,
            Self::Ternary(_) => InstructionOp::Ternary,
            Self::Xor(_) => InstructionOp::Xor,
        }
    }

    fn encode_operands(&self) -> Vec<ir::Operand> {
        match self {
            Self::Abs(x) => x.encode(),
            Self::AbsWrapped(x) => x.encode(),
            Self::Add(x) => x.encode(),
            Self::AddWrapped(x) => x.encode(),
            Self::And(x) => x.encode(),
            Self::AssertEq(x) => x.encode(),
            Self::AssertNeq(x) => x.encode(),
            Self::CommitBHP256(x) => x.encode(),
            Self::CommitBHP512(x) => x.encode(),
            Self::CommitBHP768(x) => x.encode(),
            Self::CommitBHP1024(x) => x.encode(),
            Self::CommitPED64(x) => x.encode(),
            Self::CommitPED128(x) => x.encode(),
            Self::Div(x) => x.encode(),
            Self::DivWrapped(x) => x.encode(),
            Self::Double(x) => x.encode(),
            Self::Gt(x) => x.encode(),
            Self::Gte(x) => x.encode(),
            Self::HashBHP256(x) => x.encode(),
            Self::HashBHP512(x) => x.encode(),
            Self::HashBHP768(x) => x.encode(),
            Self::HashBHP1024(x) => x.encode(),
            Self::HashPED64(x) => x.encode(),
            Self::HashPED128(x) => x.encode(),
            Self::HashPSD2(x) => x.encode(),
            Self::HashPSD4(x) => x.encode(),
            Self::HashPSD8(x) => x.encode(),
            Self::Inv(x) => x.encode(),
            Self::IsEq(x) => x.encode(),
            Self::Lt(x) => x.encode(),
            Self::Lte(x) => x.encode(),
            Self::Mod(x) => x.encode(),
            Self::Mul(x) => x.encode(),
            Self::MulWrapped(x) => x.encode(),
            Self::Nand(x) => x.encode(),
            Self::Neg(x) => x.encode(),
            Self::Nor(x) => x.encode(),
            Self::Not(x) => x.encode(),
            Self::Or(x) => x.encode(),
            Self::Pow(x) => x.encode(),
            Self::PowWrapped(x) => x.encode(),
            Self::Rem(x) => x.encode(),
            Self::RemWrapped(x) => x.encode(),
            Self::Shl(x) => x.encode(),
            Self::ShlWrapped(x) => x.encode(),
            Self::Shr(x) => x.encode(),
            Self::ShrWrapped(x) => x.encode(),
            Self::Sqrt(x) => x.encode(),
            Self::Square(x) => x.encode(),
            Self::Sub(x) => x.encode(),
            Self::SubWrapped(x) => x.encode(),
            Self::Ternary(x) => x.encode(),
            Self::Xor(x) => x.encode(),
        }
    }

    pub(crate) fn encode(&self) -> ir::Instruction {
        ir::Instruction {
            opcode: self.opcode() as u32,
            operands: self.encode_operands(),
        }
    }

    pub(crate) fn decode(instruction: ir::Instruction) -> Result<Self> {
        Ok(
            match InstructionOp::try_from_primitive(instruction.opcode)
                .map_err(|_| anyhow!("unknown instruction opcode: {}", instruction.opcode))?
            {
                InstructionOp::Abs => Self::Abs(QueryData::decode(instruction.operands)?),
                InstructionOp::AbsWrapped => {
                    Self::AbsWrapped(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::Add => Self::Add(QueryData::decode(instruction.operands)?),
                InstructionOp::AddWrapped => {
                    Self::AddWrapped(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::And => Self::And(QueryData::decode(instruction.operands)?),
                InstructionOp::AssertEq => Self::AssertEq(QueryData::decode(instruction.operands)?),
                InstructionOp::AssertNeq => {
                    Self::AssertNeq(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::CommitBHP256 => {
                    Self::CommitBHP256(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::CommitBHP512 => {
                    Self::CommitBHP512(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::CommitBHP768 => {
                    Self::CommitBHP768(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::CommitBHP1024 => {
                    Self::CommitBHP1024(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::CommitPED64 => {
                    Self::CommitPED64(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::CommitPED128 => {
                    Self::CommitPED128(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::Div => Self::Div(QueryData::decode(instruction.operands)?),
                InstructionOp::DivWrapped => {
                    Self::DivWrapped(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::Double => Self::Double(QueryData::decode(instruction.operands)?),
                InstructionOp::Gt => Self::Gt(QueryData::decode(instruction.operands)?),
                InstructionOp::Gte => Self::Gte(QueryData::decode(instruction.operands)?),
                InstructionOp::HashBHP256 => {
                    Self::HashBHP256(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::HashBHP512 => {
                    Self::HashBHP512(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::HashBHP768 => {
                    Self::HashBHP768(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::HashBHP1024 => {
                    Self::HashBHP1024(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::HashPED64 => {
                    Self::HashPED64(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::HashPED128 => {
                    Self::HashPED128(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::HashPSD2 => Self::HashPSD2(QueryData::decode(instruction.operands)?),
                InstructionOp::HashPSD4 => Self::HashPSD4(QueryData::decode(instruction.operands)?),
                InstructionOp::HashPSD8 => Self::HashPSD8(QueryData::decode(instruction.operands)?),
                InstructionOp::Inv => Self::Inv(QueryData::decode(instruction.operands)?),
                InstructionOp::IsEq => Self::IsEq(QueryData::decode(instruction.operands)?),
                InstructionOp::Lt => Self::Lt(QueryData::decode(instruction.operands)?),
                InstructionOp::Lte => Self::Lte(QueryData::decode(instruction.operands)?),
                InstructionOp::Mod => Self::Mod(QueryData::decode(instruction.operands)?),
                InstructionOp::Mul => Self::Mul(QueryData::decode(instruction.operands)?),
                InstructionOp::MulWrapped => {
                    Self::MulWrapped(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::Nand => Self::Nand(QueryData::decode(instruction.operands)?),
                InstructionOp::Neg => Self::Neg(QueryData::decode(instruction.operands)?),
                InstructionOp::Nor => Self::Nor(QueryData::decode(instruction.operands)?),
                InstructionOp::Not => Self::Not(QueryData::decode(instruction.operands)?),
                InstructionOp::Or => Self::Or(QueryData::decode(instruction.operands)?),
                InstructionOp::Pow => Self::Pow(QueryData::decode(instruction.operands)?),
                InstructionOp::PowWrapped => {
                    Self::PowWrapped(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::Rem => Self::Rem(QueryData::decode(instruction.operands)?),
                InstructionOp::RemWrapped => {
                    Self::RemWrapped(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::Shl => Self::Shl(QueryData::decode(instruction.operands)?),
                InstructionOp::ShlWrapped => {
                    Self::ShlWrapped(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::Shr => Self::Shr(QueryData::decode(instruction.operands)?),
                InstructionOp::ShrWrapped => {
                    Self::ShrWrapped(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::Sqrt => Self::Sqrt(QueryData::decode(instruction.operands)?),
                InstructionOp::Square => Self::Square(QueryData::decode(instruction.operands)?),
                InstructionOp::Sub => Self::Sub(QueryData::decode(instruction.operands)?),
                InstructionOp::SubWrapped => {
                    Self::SubWrapped(QueryData::decode(instruction.operands)?)
                }
                InstructionOp::Ternary => Self::Ternary(QueryData::decode(instruction.operands)?),
                InstructionOp::Xor => Self::Xor(QueryData::decode(instruction.operands)?),
            },
        )
    }
}

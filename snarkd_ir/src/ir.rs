use anyhow::Result;
use std::fmt;

include!(concat!(env!("OUT_DIR"), "/_includes.rs"));
pub use ir::*;
pub use opcode::*;

pub trait ProtoBuf: fmt::Display {
    type Target;

    fn encode(&self) -> Self::Target;
    fn decode(target: Self::Target) -> Result<Self>
    where
        Self: Sized;
}

// impl fmt::Display for Opcode {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         use Opcode::*;
//         match self {
//             Abs => write!(f, "abs"),
//             AbsWrapped => write!(f, "abs.w"),
//             Add => write!(f, "add"),
//             AddWrapped => write!(f, "add.w"),
//             And => write!(f, "and"),
//             AssertEq => write!(f, "assert.eq"),
//             AssertNeq => write!(f, "assert.neq"),
//             CommitBHP256 => write!(f, "commit.bhp256"),
//             CommitBHP512 => write!(f, "commit.bhp512"),
//             CommitBHP768 => write!(f, "commit.bhp768"),
//             CommitBHP1024 => write!(f, "commit.bhp1024"),
//             CommitPED64 => write!(f, "commit.ped64"),
//             CommitPED128 => write!(f, "commit.ped128"),
//             Div => write!(f, "div"),
//             DivWrapped => write!(f, "div.wrapped"),
//             Double => write!(f, "double"),
//             Gt => write!(f, "gt"),
//             Gte => write!(f, "gte"),
//             HashBHP256 => write!(f, "hash.bhp256"),
//             HashBHP512 => write!(f, "hash.bhp512"),
//             HashBHP768 => write!(f, "hash.bhp768"),
//             HashBHP1024 => write!(f, "hash.bhp1024"),
//             HashPED64 => write!(f, "hash.ped64"),
//             HashPED128 => write!(f, "hash.ped128"),
//             HashPSD2 => write!(f, "hash.psd2"),
//             HashPSD4 => write!(f, "hash.psd4"),
//             HashPSD8 => write!(f, "hash.psd8"),
//             Inv => write!(f, "inv"),
//             IsEq => write!(f, "is.eq"),
//             IsNeq => write!(f, "is.neq"),
//             Lt => write!(f, "lt"),
//             Lte => write!(f, "lte"),
//             Mod => write!(f, "mod"),
//             Mul => write!(f, "mul"),
//             MulWrapped => write!(f, "mul.w"),
//             Nand => write!(f, "nand"),
//             Neg => write!(f, "neg"),
//             Nor => write!(f, "nor"),
//             Not => write!(f, "not"),
//             Or => write!(f, "or"),
//             Pow => write!(f, "pow"),
//             PowWrapped => write!(f, "pow.w"),
//             Rem => write!(f, "rem"),
//             RemWrapped => write!(f, "rem.w"),
//             Shl => write!(f, "shl"),
//             ShlWrapped => write!(f, "shl.w"),
//             Shr => write!(f, "shr"),
//             ShrWrapped => write!(f, "shr.w"),
//             Sqrt => write!(f, "sqrt"),
//             Square => write!(f, "square"),
//             Sub => write!(f, "sub"),
//             SubWrapped => write!(f, "sub.w"),
//             Ternary => write!(f, "ternary"),
//             Xor => write!(f, "xor"),
//         }
//     }
// }

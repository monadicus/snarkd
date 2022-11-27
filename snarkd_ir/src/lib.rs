#![warn(clippy::todo, clippy::dbg_macro)]

mod ir;

mod program;
pub use program::*;

mod opcode;
pub use opcode::*;

mod operand;
pub use operand::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test;

#![warn(clippy::todo, clippy::use_debug)]

mod ir;

mod program;
pub use program::*;

mod opcode;
pub use opcode::*;

mod operand;
pub use operand::*;

#[path = ""]
#[cfg(test)]
mod tests {
    use super::*;

    mod program_test;
}

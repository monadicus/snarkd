#![warn(clippy::todo, clippy::use_debug)]

mod function;
pub use function::*;

mod header;
pub use header::*;

mod input;
pub use input::*;

mod instructions;
pub use instructions::*;

mod ir;

mod program;
pub use program::*;

mod types;
pub use types::*;

mod values;
pub use values::*;

mod visibility;
pub use visibility::*;
#[path = ""]
#[cfg(test)]
mod tests {
    use super::*;

    mod program_test;
}

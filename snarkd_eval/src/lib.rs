#![warn(clippy::todo, clippy::use_debug)]

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Todo;

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo")
    }
}

mod constraint_dummy;
mod evaluator;
mod operations;
mod setup;
mod values;

pub use constraint_dummy::*;
pub use evaluator::*;
pub use operations::*;
pub use setup::*;
pub use values::*;

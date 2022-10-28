#![warn(clippy::todo, clippy::use_debug)]

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Todo;

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "todo")
    }
}

mod evaluator;
mod operations;
mod values;

pub use evaluator::*;
pub use operations::*;
pub use values::*;

mod data;
pub use data::*;
mod evaluator;
pub use evaluator::*;
mod function;
pub use function::*;

use std::{borrow::Cow, fmt, mem, rc::Rc};

use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use snarkd_crypto::{Field, Parameters};
use snarkd_ir::{Function, InputType, InputValue, Instruction, Operand, Program, Type};

use crate::{
    bool_from_input, ConstrainedAddress, ConstrainedBool, ConstrainedField, ConstrainedValue,
    ConstraintSystem, Todo, ValueError,
};

/// the possible outcomes of evaluating an instruction
#[derive(Debug)]
pub enum ControlFlow<'a> {
    Continue,
    Return,
    Recurse(&'a Instruction),
}

/// data concerning the parent instruction of a scope
#[derive(Debug)]
pub enum ParentInstruction<'a> {
    None,
    Call(&'a Todo),
    Mask(ConstrainedBool),
    Repeat(u32),
}

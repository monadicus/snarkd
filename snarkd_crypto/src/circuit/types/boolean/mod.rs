mod and;
mod equal;
mod nand;
mod nor;
mod not;
mod or;
mod ternary;
mod xor;

use std::{fmt, ops::Deref};

use crate::{
    bls12_377::{Field, Fp},
    circuit::{
        circuit::Circuit,
        helpers::{LinearCombination, Mode},
        traits::{Eject, Inject},
        Environment,
    },
};

#[derive(Clone)]
pub struct Boolean(LinearCombination);

// impl BooleanTrait for Boolean {}

impl Inject for Boolean {
    type Primitive = bool;

    /// Initializes a new instance of a boolean from a primitive boolean value.
    fn new(mode: Mode, value: Self::Primitive) -> Self {
        let variable = Circuit::new_variable(
            mode,
            match value {
                true => Fp::ONE,
                false => Fp::ZERO,
            },
        );

        // Ensure (1 - a) * a = 0
        // `a` must be either 0 or 1.
        Circuit::enforce(|| (Circuit::one() - &variable, &variable, Circuit::zero()));

        Self(variable.into())
    }

    /// Initializes a constant boolean circuit from a primitive boolean value.
    fn constant(value: Self::Primitive) -> Self {
        match value {
            true => Self(Circuit::one()),
            false => Self(Circuit::zero()),
        }
    }
}

impl Eject for Boolean {
    type Primitive = bool;

    /// Ejects the mode of the boolean.
    fn eject_mode(&self) -> Mode {
        // Perform a software-level safety check that the boolean is well-formed.
        match self.0.is_boolean_type() {
            true => self.0.mode(),
            false => Circuit::halt("Boolean variable is not well-formed"),
        }
    }

    /// Ejects the boolean as a constant boolean value.
    fn eject_value(&self) -> Self::Primitive {
        let value = self.0.value();
        debug_assert!(value.is_zero() || value.is_one());
        value.is_one()
    }
}

impl fmt::Debug for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.eject_value(), self.eject_mode())
    }
}

impl Deref for Boolean {
    type Target = LinearCombination;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Boolean> for LinearCombination {
    fn from(boolean: Boolean) -> Self {
        boolean.0
    }
}

impl From<&Boolean> for LinearCombination {
    fn from(boolean: &Boolean) -> Self {
        boolean.0.clone()
    }
}

#[cfg(test)]
#[path = ""]
mod test {
    mod and_tests;
    mod boolean_tests;
    mod equal_tests;
    mod nand_tests;
    mod nor_tests;
    mod not_tests;
    mod or_tests;
    mod ternary_tests;
    mod xor_tests;
}

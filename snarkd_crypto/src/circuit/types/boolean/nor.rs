use crate::{
    bls12_377::{Field, Fp},
    circuit::{
        circuit::Circuit,
        helpers::Mode,
        traits::{Eject, Nor},
        Environment,
    },
};

#[cfg(test)]
use crate::circuit::{
    helpers::{CircuitType, Count},
    traits::{Metrics, OutputMode},
};

use super::Boolean;

impl Nor<Self> for Boolean {
    type Output = Boolean;

    /// Returns `(NOT a) AND (NOT b)`.
    fn nor(&self, other: &Self) -> Self::Output {
        // Constant `self`
        if self.is_constant() {
            match self.eject_value() {
                true => !self.clone(),
                false => !other.clone(),
            }
        }
        // Constant `other`
        else if other.is_constant() {
            match other.eject_value() {
                true => !other.clone(),
                false => !self.clone(),
            }
        }
        // Variable NOR Variable
        else {
            // Declare a new variable with the expected output as witness.
            // Note: The constraint below will ensure `output` is either 0 or 1,
            // assuming `self` and `other` are well-formed (they are either 0 or 1).
            let output = Boolean(
                Circuit::new_variable(
                    Mode::Private,
                    match !self.eject_value() & !other.eject_value() {
                        true => Fp::ONE,
                        false => Fp::ZERO,
                    },
                )
                .into(),
            );

            // Ensure (1 - `self`) * (1 - `other`) = `output`
            // `output` is `1` iff `self` and `other` are both `0`, otherwise `output` is `0`.
            Circuit::enforce(|| (Circuit::one() - &self.0, Circuit::one() - &other.0, &output));

            output
        }
    }
}

#[cfg(test)]
impl Metrics<dyn Nor<Boolean, Output = Boolean>> for Boolean {
    type Case = (Mode, Mode);

    fn count(case: &Self::Case) -> Count {
        match case.0.is_constant() || case.1.is_constant() {
            true => Count::is(0, 0, 0, 0),
            false => Count::is(0, 0, 1, 1),
        }
    }
}

#[cfg(test)]
impl OutputMode<dyn Nor<Boolean, Output = Boolean>> for Boolean {
    type Case = (CircuitType<Boolean>, CircuitType<Boolean>);

    fn output_mode(case: &Self::Case) -> Mode {
        match (case.0.mode(), case.1.mode()) {
            (Mode::Constant, Mode::Constant) => Mode::Constant,
            (_, Mode::Constant) => match &case.1 {
                CircuitType::Constant(constant) => match constant.eject_value() {
                    true => Mode::Constant,
                    false => Mode::Private,
                },
                _ => Circuit::halt(
                    "The constant is required to determine the output mode of Public NOR Constant",
                ),
            },
            (Mode::Constant, _) => match &case.0 {
                CircuitType::Constant(constant) => match constant.eject_value() {
                    true => Mode::Constant,
                    false => Mode::Private,
                },
                _ => Circuit::halt(
                    "The constant is required to determine the output mode of Constant NOR Public",
                ),
            },
            (_, _) => Mode::Private,
        }
    }
}

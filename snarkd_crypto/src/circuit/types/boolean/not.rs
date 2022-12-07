use std::{ops::Not, rc::Rc};

use crate::{
    bls12_377::{Field, Fp},
    circuit::{circuit::Circuit, helpers::Variable, Environment},
};

#[cfg(test)]
use crate::circuit::{
    helpers::Count,
    traits::{Metrics, OutputMode},
    types::boolean::Mode,
};

use super::Boolean;

impl Not for Boolean {
    type Output = Self;

    /// Returns `(NOT a)`.
    fn not(self) -> Self::Output {
        (&self).not()
    }
}

impl Not for &Boolean {
    type Output = Boolean;

    /// Returns `(NOT a)`.
    fn not(self) -> Self::Output {
        // The `NOT` operation behaves as follows:
        //     Case 1: If `(self == 0)`, then `(1 - self) == 1`.
        //     Case 2: If `(self == 1)`, then `(1 - self) == 0`.
        match self.is_constant() {
            // Constant case.
            true => Boolean(Circuit::one() - &self.0),
            // Public and private cases.
            false => Boolean(Variable::Public(0, Rc::new(Fp::ONE)) - &self.0),
        }
    }
}

#[cfg(test)]
impl Metrics<dyn Not<Output = Boolean>> for Boolean {
    type Case = Mode;

    fn count(_case: &Self::Case) -> Count {
        Count::is(0, 0, 0, 0)
    }
}

#[cfg(test)]
impl OutputMode<dyn Not<Output = Boolean>> for Boolean {
    type Case = Mode;

    fn output_mode(case: &Self::Case) -> Mode {
        match case {
            Mode::Constant => Mode::Constant,
            _ => Mode::Private,
        }
    }
}

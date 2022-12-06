use std::ops::Neg;

#[cfg(test)]
use crate::circuit::{
    helpers::{Count, Mode},
    traits::{Metrics, OutputMode},
};

use super::Field;

impl Neg for Field {
    type Output = Self;

    /// Performs the unary `-` operation.
    fn neg(self) -> Self::Output {
        (&self).neg()
    }
}

impl Neg for &Field {
    type Output = Field;

    /// Performs the unary `-` operation.
    fn neg(self) -> Self::Output {
        (-&self.linear_combination).into()
    }
}

#[cfg(test)]
impl Metrics<dyn Neg<Output = Field>> for Field {
    type Case = Mode;

    fn count(_case: &Self::Case) -> Count {
        Count::is(0, 0, 0, 0)
    }
}

#[cfg(test)]
impl OutputMode<dyn Neg<Output = Field>> for Field {
    type Case = Mode;

    fn output_mode(case: &Self::Case) -> Mode {
        match case {
            Mode::Constant => Mode::Constant,
            _ => Mode::Private,
        }
    }
}

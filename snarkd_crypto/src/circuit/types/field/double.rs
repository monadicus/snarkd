use crate::circuit::traits::{Metrics, OutputMode};

use super::*;

impl Double for Field {
    type Output = Field;

    fn double(&self) -> Self::Output {
        self + self
    }
}

impl Metrics<dyn Double<Output = Field>> for Field {
    type Case = Mode;

    fn count(_parameter: &Self::Case) -> Count {
        Count::is(0, 0, 0, 0)
    }
}

impl OutputMode<dyn Double<Output = Field>> for Field {
    type Case = Mode;

    fn output_mode(input: &Self::Case) -> Mode {
        match input.is_constant() {
            true => Mode::Constant,
            false => Mode::Private,
        }
    }
}

use std::ops::{Add, AddAssign};

use crate::{
    bls12_377::Field as FieldTrait,
    circuit::{
        circuit::Circuit,
        helpers::{CircuitType, Count, Mode},
        traits::{Eject, Metrics, OutputMode},
        Environment,
    },
};

use super::Field;

impl Add<Field> for Field {
    type Output = Field;

    fn add(self, other: Field) -> Self::Output {
        self + &other
    }
}

impl Add<&Field> for Field {
    type Output = Field;

    fn add(self, other: &Field) -> Self::Output {
        let mut result = self;
        result += other;
        result
    }
}

impl Add<Field> for &Field {
    type Output = Field;

    fn add(self, other: Field) -> Self::Output {
        self + &other
    }
}

impl Add<&Field> for &Field {
    type Output = Field;

    fn add(self, other: &Field) -> Self::Output {
        let mut result = self.clone();
        result += other;
        result
    }
}

impl AddAssign<Field> for Field {
    fn add_assign(&mut self, other: Field) {
        *self += &other;
    }
}

impl AddAssign<&Field> for Field {
    fn add_assign(&mut self, other: &Field) {
        self.linear_combination += &other.linear_combination;
        self.bits_le = Default::default();
    }
}

impl Metrics<dyn Add<Field, Output = Field>> for Field {
    type Case = (Mode, Mode);

    fn count(_case: &Self::Case) -> Count {
        Count::is(0, 0, 0, 0)
    }
}

impl OutputMode<dyn Add<Field, Output = Field>> for Field {
    type Case = (CircuitType<Field>, CircuitType<Field>);

    fn output_mode(case: &Self::Case) -> Mode {
        match (case.0.mode(), case.1.mode()) {
            (Mode::Constant, Mode::Constant) => Mode::Constant,
            (Mode::Constant, Mode::Public) => match &case.0 {
                CircuitType::Constant(constant) => match constant.eject_value().is_zero() {
                    true => Mode::Public,
                    false => Mode::Private,
                },
                _ => Circuit::halt(
                    "The constant is required to determine the output mode of Public + Constant",
                ),
            },
            (Mode::Public, Mode::Constant) => match &case.1 {
                CircuitType::Constant(constant) => match constant.eject_value().is_zero() {
                    true => Mode::Public,
                    false => Mode::Private,
                },
                _ => Circuit::halt(
                    "The constant is required to determine the output mode of Public + Constant",
                ),
            },
            (_, _) => Mode::Private,
        }
    }
}

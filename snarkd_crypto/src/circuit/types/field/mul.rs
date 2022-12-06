use std::ops::{Mul, MulAssign};

use crate::circuit::{circuit::Circuit, helpers::Mode, traits::Eject, Environment};

#[cfg(test)]
use crate::{
    bls12_377::Field as FieldTrait,
    circuit::{
        helpers::{CircuitType, Count},
        traits::{Metrics, OutputMode},
    },
};

use super::Field;

impl Mul<Field> for Field {
    type Output = Field;

    fn mul(self, other: Field) -> Self::Output {
        self * &other
    }
}

impl Mul<&Field> for Field {
    type Output = Field;

    fn mul(self, other: &Field) -> Self::Output {
        let mut output = self;
        output *= other;
        output
    }
}

impl Mul<Field> for &Field {
    type Output = Field;

    fn mul(self, other: Field) -> Self::Output {
        other * self
    }
}

impl Mul<&Field> for &Field {
    type Output = Field;

    fn mul(self, other: &Field) -> Self::Output {
        let mut output = self.clone();
        output *= other;
        output
    }
}

impl MulAssign<Field> for Field {
    fn mul_assign(&mut self, other: Field) {
        *self *= &other;
    }
}

impl MulAssign<&Field> for Field {
    fn mul_assign(&mut self, other: &Field) {
        match (self.is_constant(), other.is_constant()) {
            (true, true) | (false, true) => {
                *self = (&self.linear_combination * other.eject_value()).into()
            }
            (true, false) => *self = (&other.linear_combination * self.eject_value()).into(),
            (false, false) => {
                let mode = Mode::from(self.is_constant());
                let product = Circuit::new_witness(mode, || {
                    let _self = self.eject_value();
                    let other = other.eject_value();
                    {
                        _self * other
                    }
                });

                // Ensure self * other == product.
                Circuit::enforce(|| (&*self, other, &product));

                *self = product;
            }
        }
    }
}

#[cfg(test)]
impl Metrics<dyn Mul<Field, Output = Field>> for Field {
    type Case = (Mode, Mode);

    fn count(case: &Self::Case) -> Count {
        match case.0.is_constant() || case.1.is_constant() {
            true => Count::is(0, 0, 0, 0),
            false => Count::is(0, 0, 1, 1),
        }
    }
}

#[cfg(test)]
impl OutputMode<dyn Mul<Field, Output = Field>> for Field {
    type Case = (CircuitType<Field>, CircuitType<Field>);

    fn output_mode(case: &Self::Case) -> Mode {
        match (case.0.mode(), case.1.mode()) {
            (Mode::Constant, Mode::Constant) => Mode::Constant,
            (Mode::Constant, Mode::Public) => match &case.0 {
                CircuitType::Constant(constant) => match constant.eject_value() {
                    // TODO: Should this be a constant?
                    //value if value.is_zero() => Mode::Constant,
                    value if value.is_one() => Mode::Public,
                    _ => Mode::Private,
                },
                _ => Circuit::halt(
                    "The constant is required to determine the output mode of Public * Constant",
                ),
            },
            (Mode::Public, Mode::Constant) => match &case.1 {
                CircuitType::Constant(constant) => match constant.eject_value() {
                    // TODO: Should this be a constant?
                    //value if value.is_zero() => Mode::Constant,
                    value if value.is_one() => Mode::Public,
                    _ => Mode::Private,
                },
                _ => Circuit::halt(
                    "The constant is required to determine the output mode of Public * Constant",
                ),
            },
            (_, _) => Mode::Private,
        }
    }
}

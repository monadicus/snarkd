mod add;
// mod compare;
mod div;
// mod double;
mod helpers;
mod mul;
mod neg;
mod sub;

use std::fmt;

use once_cell::unsync::OnceCell;

use crate::{
    bls12_377::Fp,
    circuit::{
        circuit::Circuit,
        helpers::{LinearCombination, Mode},
        traits::{Eject, Inject},
        Environment,
    },
};

use super::Boolean;

#[derive(Clone)]
pub struct Field {
    /// The linear combination contains the primary representation of the field.
    linear_combination: LinearCombination,
    /// An optional secondary representation in little-endian bits is provided,
    /// so that calls to `ToBits` only incur constraint costs once.
    bits_le: OnceCell<Vec<Boolean>>,
}

impl Field {
    pub fn zero() -> Self {
        Circuit::zero().into()
    }

    pub fn one() -> Self {
        Circuit::one().into()
    }
}

impl Inject for Field {
    type Primitive = Fp;

    /// Initializes a field circuit from a console field.
    fn new(mode: Mode, field: Self::Primitive) -> Self {
        Self {
            linear_combination: Circuit::new_variable(mode, field).into(),
            bits_le: Default::default(),
        }
    }
}

impl Eject for Field {
    type Primitive = Fp;

    /// Ejects the mode of the field circuit.
    fn eject_mode(&self) -> Mode {
        self.linear_combination.mode()
    }

    /// Ejects the field circuit as a console field.
    fn eject_value(&self) -> Self::Primitive {
        self.linear_combination.value()
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.eject_value(), self.eject_mode())
    }
}

impl From<LinearCombination> for Field {
    fn from(linear_combination: LinearCombination) -> Self {
        Self {
            linear_combination,
            bits_le: Default::default(),
        }
    }
}

impl From<&LinearCombination> for Field {
    fn from(linear_combination: &LinearCombination) -> Self {
        From::from(linear_combination.clone())
    }
}

impl From<Field> for LinearCombination {
    fn from(field: Field) -> Self {
        From::from(&field)
    }
}

impl From<&Field> for LinearCombination {
    fn from(field: &Field) -> Self {
        field.linear_combination.clone()
    }
}

#[cfg(test)]
#[path = ""]
mod test {
    use super::*;

    mod add_tests;
    // mod compare_tests;
    mod div_tests;
    mod mul_tests;
    mod neg_tests;
    mod sub_tests;
}

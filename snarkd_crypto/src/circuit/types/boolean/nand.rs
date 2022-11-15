use crate::{
    bls12_377::{Field, Fp},
    circuit::{
        circuit::Circuit,
        helpers::Mode,
        traits::{Eject, Nand},
        Environment,
    },
};

use super::Boolean;

impl Nand<Self> for Boolean {
    type Output = Boolean;

    /// Returns `NOT (a AND b)`.
    fn nand(&self, other: &Self) -> Self::Output {
        // Constant `self`
        if self.is_constant() {
            match self.eject_value() {
                true => !other.clone(),
                false => !self.clone(),
            }
        }
        // Constant `other`
        else if other.is_constant() {
            match other.eject_value() {
                true => !self.clone(),
                false => !other.clone(),
            }
        }
        // Variable NAND Variable
        else {
            // Declare a new variable with the expected output as witness.
            // Note: The constraint below will ensure `output` is either 0 or 1,
            // assuming `self` and `other` are well-formed (they are either 0 or 1).
            let output = Boolean(
                Circuit::new_variable(
                    Mode::Private,
                    match !(self.eject_value() & other.eject_value()) {
                        true => Fp::ONE,
                        false => Fp::ZERO,
                    },
                )
                .into(),
            );

            // Ensure `self` * `other` = (1 - `output`)
            // `output` is `1` iff `self` or `other` is `0`, otherwise `output` is `0`.
            Circuit::enforce(|| (self, other, Circuit::one() - &output.0));

            output
        }
    }
}

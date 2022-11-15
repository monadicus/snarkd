use std::ops::{BitAnd, BitAndAssign};

use crate::{
    bls12_377::{Field, Fp},
    circuit::{circuit::Circuit, helpers::Mode, traits::Eject, Environment},
};

use super::Boolean;

impl BitAnd<Boolean> for Boolean {
    type Output = Boolean;

    /// Returns `(self AND other)`.
    fn bitand(self, other: Boolean) -> Self::Output {
        self & &other
    }
}

impl BitAnd<Boolean> for &Boolean {
    type Output = Boolean;

    /// Returns `(self AND other)`.
    fn bitand(self, other: Boolean) -> Self::Output {
        self & &other
    }
}

impl BitAnd<&Boolean> for Boolean {
    type Output = Boolean;

    /// Returns `(self AND other)`.
    fn bitand(self, other: &Boolean) -> Self::Output {
        &self & other
    }
}

impl BitAnd<&Boolean> for &Boolean {
    type Output = Boolean;

    /// Returns `(self AND other)`.
    fn bitand(self, other: &Boolean) -> Self::Output {
        let mut output = self.clone();
        output &= other;
        output
    }
}

impl BitAndAssign<Boolean> for Boolean {
    /// Sets `self` as `(self AND other)`.
    fn bitand_assign(&mut self, other: Boolean) {
        *self &= &other;
    }
}

impl BitAndAssign<&Boolean> for Boolean {
    /// Sets `self` as `(self AND other)`.
    fn bitand_assign(&mut self, other: &Boolean) {
        // Stores the bitwise AND of `self` and `other` in `self`.
        *self =
            // Constant `self`
            if self.is_constant() {
                match self.eject_value() {
                    true => other.clone(),
                    false => self.clone(),
                }
            }
            // Constant `other`
            else if other.is_constant() {
                match other.eject_value() {
                    true => self.clone(),
                    false => other.clone(),
                }
            }
            // Variable AND Variable
            else {
                // Declare a new variable with the expected output as witness.
                // Note: The constraint below will ensure `output` is either 0 or 1,
                // assuming `self` and `other` are well-formed (they are either 0 or 1).
                let output = Boolean(
                    Circuit::new_variable(Mode::Private, match self.eject_value() & other.eject_value() {
                        true => Fp::ONE,
                        false => Fp::ZERO,
                    })
                        .into(),
                );

                // Ensure `self` * `other` = `output`
                // `output` is `1` iff `self` AND `other` are both `1`.
                Circuit::enforce(|| (&*self, other, &output));

                output
            }
    }
}

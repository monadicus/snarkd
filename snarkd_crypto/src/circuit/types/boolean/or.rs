use std::ops::{BitOr, BitOrAssign};

use crate::{
    bls12_377::{Field, Fp},
    circuit::{circuit::Circuit, helpers::Mode, traits::Eject, Environment},
};

use super::Boolean;

impl BitOr<Self> for Boolean {
    type Output = Self;

    /// Returns `(self OR other)`.
    fn bitor(self, other: Boolean) -> Self::Output {
        self | &other
    }
}

impl BitOr<Boolean> for &Boolean {
    type Output = Boolean;

    /// Returns `(self OR other)`.
    fn bitor(self, other: Boolean) -> Self::Output {
        self | &other
    }
}

impl BitOr<&Boolean> for Boolean {
    type Output = Boolean;

    /// Returns `(self OR other)`.
    fn bitor(self, other: &Boolean) -> Self::Output {
        &self | other
    }
}

impl BitOr<&Boolean> for &Boolean {
    type Output = Boolean;

    /// Returns `(self OR other)`.
    fn bitor(self, other: &Boolean) -> Self::Output {
        let mut output = self.clone();
        output |= other;
        output
    }
}

impl BitOrAssign<Self> for Boolean {
    /// Sets `self` as `(self OR other)`.
    fn bitor_assign(&mut self, other: Self) {
        *self |= &other;
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl BitOrAssign<&Boolean> for Boolean {
    /// Sets `self` as `(self OR other)`.
    fn bitor_assign(&mut self, other: &Boolean) {
        // Stores the bitwise OR of `self` and `other` in `self`.
        *self =
          // Constant `self`
          if self.is_constant() {
              match self.eject_value() {
                  true => self.clone(),
                  false => other.clone(),
              }
          }
          // Constant `other`
          else if other.is_constant() {
              match other.eject_value() {
                  true => other.clone(),
                  false => self.clone(),
              }
          }
          // Variable OR Variable
          else {
              // Declare a new variable with the expected output as witness.
              // Note: The constraint below will ensure `output` is either 0 or 1,
              // assuming `self` and `other` are well-formed (they are either 0 or 1).
              let output = Boolean(
                  Circuit::new_variable(Mode::Private, match self.eject_value() | other.eject_value() {
                      true => Fp::ONE,
                      false => Fp::ZERO,
                  })
                      .into(),
              );

              // Ensure (1 - `self`) * (1 - `other`) = (1 - `output`)
              // `output` is `1` iff `self` OR `other` is `1`.
              Circuit::enforce(|| (Circuit::one() - &self.0, Circuit::one() - &other.0, Circuit::one() - &output.0));

              output
          }
    }
}

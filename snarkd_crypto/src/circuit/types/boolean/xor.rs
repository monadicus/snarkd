use std::ops::{BitXor, BitXorAssign};

use crate::{
    bls12_377::{Field, Fp},
    circuit::{circuit::Circuit, helpers::Mode, traits::Eject, Environment},
};

use super::Boolean;

impl BitXor<Boolean> for Boolean {
    type Output = Boolean;

    /// Returns `(self != other)`.
    fn bitxor(self, other: Boolean) -> Self::Output {
        self ^ &other
    }
}

impl BitXor<Boolean> for &Boolean {
    type Output = Boolean;

    /// Returns `(self != other)`.
    fn bitxor(self, other: Boolean) -> Self::Output {
        self ^ &other
    }
}

impl BitXor<&Boolean> for Boolean {
    type Output = Boolean;

    /// Returns `(self != other)`.
    fn bitxor(self, other: &Boolean) -> Self::Output {
        &self ^ other
    }
}

impl BitXor<&Boolean> for &Boolean {
    type Output = Boolean;

    /// Returns `(self != other)`.
    fn bitxor(self, other: &Boolean) -> Self::Output {
        let mut output = self.clone();
        output ^= other;
        output
    }
}

impl BitXorAssign<Boolean> for Boolean {
    /// Sets `self` as `(self != other)`.
    fn bitxor_assign(&mut self, other: Boolean) {
        *self ^= &other;
    }
}

impl BitXorAssign<&Boolean> for Boolean {
    /// Sets `self` as `(self != other)`.
    fn bitxor_assign(&mut self, other: &Boolean) {
        // Stores the bitwise XOR of `self` and `other` in `self`.
        *self =
          // Constant `self`
          if self.is_constant() {
              match self.eject_value() {
                  true => !other.clone(),
                  false => other.clone(),
              }
          }
          // Constant `other`
          else if other.is_constant() {
              match other.eject_value() {
                  true => !self.clone(),
                  false => self.clone(),
              }
          }
          // Variable != Variable
          else {
              // Declare a new variable with the expected output as witness.
              // Note: The constraint below will ensure `output` is either 0 or 1,
              // assuming `self` and `other` are well-formed (they are either 0 or 1).
              let output = Boolean(
                  Circuit::new_variable(Mode::Private, match self.eject_value() ^ other.eject_value() {
                      true => Fp::ONE,
                      false => Fp::ZERO,
                  })
                      .into(),
              );

              //
              // Ensure (`self` + `self`) * (`other`) = (`self` + `other` - `output`)
              // `output` is `1` iff `self` != `other`.
              //
              // As `self` and `other` are enforced to be `Boolean` types,
              // if they are equal, then the `output` is 0,
              // and if they are different, then `output` must be 1.
              //
              // ¬(a ∧ b) ∧ ¬(¬a ∧ ¬b) = c
              //
              // (1 - (a * b)) * (1 - ((1 - a) * (1 - b))) = c
              // (1 - ab) * (1 - (1 - a - b + ab)) = c
              // (1 - ab) * (a + b - ab) = c
              // a + b - ab - (a^2)b - (b^2)a + (a^2)(b^2) = c
              // a + b - ab - ab - ab + ab = c
              // a + b - 2ab = c
              // -2a * b = c - a - b
              // 2a * b = a + b - c
              // (a + a) * b = a + b - c
              //
              Circuit::enforce(|| ((&self.0 + &self.0), other, (&self.0 + &other.0 - &output.0)));

              output
          }
    }
}

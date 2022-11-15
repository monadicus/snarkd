use crate::circuit::{
    traits::{Compare, Eject, Inject, Ternary},
    types::Boolean,
};

use super::Field;

impl Compare<Field> for Field {
    type Output = Boolean;

    /// Returns `true` if `self` is less than `other`.
    fn is_less_than(&self, other: &Self) -> Self::Output {
        // Case 1: Constant < Constant
        if self.is_constant() && other.is_constant() {
            Boolean::constant(self.eject_value() < other.eject_value())
        }
        // Case 2: Constant < Variable
        else if self.is_constant() {
            // See the `else` case below for the truth table and description of the logic.
            self.to_bits_le()
                .into_iter()
                .zip_eq(other.to_bits_le())
                .fold(
                    Boolean::constant(false),
                    |is_less_than, (this, that)| match this.eject_value() {
                        true => that.bitand(&is_less_than),
                        false => that.bitor(&is_less_than),
                    },
                )
        }
        // Case 3: Variable < Constant
        else if other.is_constant() {
            // See the `else` case below for the truth table and description of the logic.
            self.to_bits_le()
                .into_iter()
                .zip_eq(other.to_bits_le())
                .fold(
                    Boolean::constant(false),
                    |is_less_than, (this, that)| match that.eject_value() {
                        true => (!this).bitor(is_less_than),
                        false => (!this).bitand(&is_less_than),
                    },
                )
        }
        // Case 4: Variable < Variable
        else {
            // Check each bitwise pair of `(self, other)` from MSB to LSB as follows:
            //   - If `this` != `that`, and if `this` is `true`, return `false`.
            //   - If `this` != `that`, and if `this` is `false`, return `true`.
            //   - If `this` == `that`, return `is_less_than`.
            //
            // The following is the truth table:
            //
            // | this    | that    | is_less_than | result |
            // |---------+---------+--------------+--------|
            // | `true`  | `true`  | `true`       | `true` |
            // | `true`  | `true`  | `false`      | `false`|
            // | `true`  | `false` | `true`       | `true` |
            // | `true`  | `false` | `false`      | `true` |
            // | `false` | `true`  | `true`       | `false`|
            // | `false` | `true`  | `false`      | `false`|
            // | `false` | `false` | `true`       | `true` |
            // | `false` | `false` | `false`      | `false`|
            //
            self.to_bits_le().iter().zip_eq(other.to_bits_le()).fold(
                Boolean::constant(false),
                |is_less_than, (this, that)| {
                    Boolean::ternary(&this.bitxor(&that), &that, &is_less_than)
                },
            )
        }
    }

    /// Returns `true` if `self` is greater than `other`.
    fn is_greater_than(&self, other: &Self) -> Self::Output {
        other.is_less_than(self)
    }

    /// Returns `true` if `self` is less than or equal to `other`.
    fn is_less_than_or_equal(&self, other: &Self) -> Self::Output {
        other.is_greater_than_or_equal(self)
    }

    /// Returns `true` if `self` is greater than or equal to `other`.
    fn is_greater_than_or_equal(&self, other: &Self) -> Self::Output {
        !self.is_less_than(other)
    }
}

use crate::circuit::{helpers::Mode, traits::Compare, types::Boolean};

use super::Scalar;

// impl Compare<Scalar> for Scalar {
//     /// Returns `true` if `self` is less than `other`.
//     fn is_less_than(&self, other: &Self) -> Boolean {
//         // Case 1: Constant < Constant
//         if self.is_constant() && other.is_constant() {
//             Boolean::new(Mode::Constant, self.eject_value() < other.eject_value())
//         }
//         // Case 2: Constant < Variable | Variable < Constant | Variable < Variable
//         else {
//             // If all scalar field elements are less than (MODULUS - 1)/2 on the base field,
//             // we can perform an optimized check for `is_less_than` by casting the scalars onto the base field.
//             debug_assert!(ScalarField::modulus() < BaseField::modulus_minus_one_div_two());

//             // Intuition: Check the parity of 2 * (`self` - `other`) mod MODULUS.
//             //   - If `self` < `other`, then 2 * (`self` - `other`) mod MODULUS is odd.
//             //   - If `self` >= `other`, then 2 * (`self` - `other`) mod MODULUS is even.

//             // Compute 2 * (`self` - `other`).
//             let outcome = (self.to_field() - other.to_field()).double();
//             // Retrieve the LSB from the computation to determine even / odd parity.
//             outcome
//                 .to_bits_be()
//                 .pop()
//                 .unwrap_or_else(|| E::halt("Failed to retrieve the LSB from the field element."))
//         }
//     }

//     /// Returns `true` if `self` is greater than `other`.
//     fn is_greater_than(&self, other: &Self) -> Boolean {
//         other.is_less_than(self)
//     }

//     /// Returns `true` if `self` is less than or equal to `other`.
//     fn is_less_than_or_equal(&self, other: &Self) -> Boolean {
//         other.is_greater_than_or_equal(self)
//     }

//     /// Returns `true` if `self` is greater than or equal to `other`.
//     fn is_greater_than_or_equal(&self, other: &Self) -> Boolean {
//         !self.is_less_than(other)
//     }
// }

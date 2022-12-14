use std::ops::{Add, AddAssign};

use once_cell::unsync::OnceCell;

use crate::{
    bls12_377::scalar,
    circuit::traits::ToField,
    circuit::{traits::Ternary, types::Field},
};

use super::Scalar;

// impl Add<Scalar> for Scalar {
//     type Output = Scalar;

//     fn add(self, other: Scalar) -> Self::Output {
//         self + &other
//     }
// }

// impl Add<Scalar> for &Scalar {
//     type Output = Scalar;

//     fn add(self, other: Scalar) -> Self::Output {
//         self + &other
//     }
// }

// impl Add<&Scalar> for Scalar {
//     type Output = Scalar;

//     fn add(self, other: &Scalar) -> Self::Output {
//         &self + other
//     }
// }

// impl Add<&Scalar> for &Scalar {
//     type Output = Scalar;

//     fn add(self, other: &Scalar) -> Self::Output {
//         let mut result = self.clone();
//         result += other;
//         result
//     }
// }

// impl AddAssign<Scalar> for Scalar {
//     fn add_assign(&mut self, other: Scalar) {
//         *self += &other;
//     }
// }

// impl AddAssign<&Scalar> for Scalar {
//     fn add_assign(&mut self, other: &Scalar) {
//         // Determine the variable mode.
//         if self.is_constant() && other.is_constant() {
//             // Compute the sum and set the new constant in `self`.
//             *self = witness!(|self, other| self + other);
//         } else {
//             // Instead of adding the bits of `self` and `other` directly, the scalars are
//             // converted into a field elements, and summed, before converting back to scalars.
//             // Note: This is safe as the base field is larger than the scalar field.
//             let sum = self.to_field() + other.to_field();

//             // Extract the scalar field bits from the field element, with a carry bit.
//             // (For advanced users) This operation saves us 2 private variables and 2 constraints.
//             let bits_le = sum.to_lower_bits_le(E::ScalarField::size_in_bits() + 1);

//             // Recover the sanitized (truncated) sum on the base field.
//             // (For advanced users) This operation saves us 2 private variables and 2 constraints.
//             let sum = Field::from_bits_le(&bits_le);

//             // Initialize the scalar field modulus as a constant base field variable.
//             //
//             // Note: We are reconstituting the scalar field into a base field here in order to
//             // compute the difference between the sum and modulus. This is safe as the scalar field modulus
//             // is less that the base field modulus, and thus will always fit in a base field element.
//             let modulus = Field::constant(
//                 match console::FromBits::from_bits_le(&E::ScalarField::modulus().to_bits_le()) {
//                     Ok(modulus) => modulus,
//                     Err(error) => E::halt(format!(
//                         "Failed to retrieve the scalar modulus as bytes: {error}"
//                     )),
//                 },
//             );

//             // Determine the wrapping sum, by computing the difference between the sum and modulus, if `sum` < `modulus`.
//             let wrapping_sum =
//                 Ternary::ternary(&sum.is_less_than(&modulus), &sum, &(&sum - &modulus));

//             // Retrieve the bits of the wrapping sum.
//             let bits_le = wrapping_sum.to_lower_bits_le(scalar::MODULUS_BITS);

//             // Set the sum of `self` and `other`, in `self`.
//             *self = Scalar {
//                 field: wrapping_sum,
//                 bits_le: OnceCell::with_value(bits_le),
//             };
//         }
//     }
// }

// impl Metrics<dyn Add<Scalar, Output = Scalar>> for Scalar {
//     type Case = (Mode, Mode);

//     fn count(case: &Self::Case) -> Count {
//         match (case.0, case.1) {
//             (Mode::Constant, Mode::Constant) => Count::is(1, 0, 0, 0),
//             (_, _) => Count::is(254, 0, 755, 757),
//         }
//     }
// }

// impl OutputMode<dyn Add<Scalar, Output = Scalar>> for Scalar {
//     type Case = (Mode, Mode);

//     fn output_mode(case: &Self::Case) -> Mode {
//         match (case.0, case.1) {
//             (Mode::Constant, Mode::Constant) => Mode::Constant,
//             (_, _) => Mode::Private,
//         }
//     }
// }

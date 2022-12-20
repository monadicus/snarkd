use crate::{
    circuit::{traits::ToBits, Environment},
    witness,
};

use super::*;

impl ToBits for Field {
    /// Outputs the little-endian bit representation of `self` *without* trailing zeros.
    fn to_bits_le(&self) -> Vec<Boolean> {
        (&self).to_bits_le()
    }

    /// Outputs the big-endian bit representation of `self` *without* leading zeros.
    fn to_bits_be(&self) -> Vec<Boolean> {
        (&self).to_bits_be()
    }
}

impl ToBits for &Field {
    /// Outputs the little-endian bit representation of `self` *without* trailing zeros.
    fn to_bits_le(&self) -> Vec<Boolean> {
        self.bits_le
            .get_or_init(|| {
                // Construct a vector of `Boolean`s comprising the bits of the field value.
                let bits_le = witness!(|self| self.to_bits_le());

                // Reconstruct the bits as a linear combination representing the original field value.
                let mut accumulator = Field::zero();
                let mut coefficient = Field::one();
                for bit in &bits_le {
                    accumulator += Field::from(**bit) * &coefficient;
                    coefficient = coefficient.double();
                }

                // Ensure value * 1 == (2^i * b_i + ... + 2^0 * b_0)
                Circuit::assert_eq(*self, accumulator);

                bits_le
            })
            .clone()
    }

    /// Outputs the big-endian bit representation of `self` *without* leading zeros.
    fn to_bits_be(&self) -> Vec<Boolean> {
        let mut bits_le = self.to_bits_le();
        bits_le.reverse();
        bits_le
    }
}

// impl Metrics<dyn ToBits<Boolean = Boolean<E>>> for Field {
//     type Case = Mode;

//     fn count(case: &Self::Case) -> Count {
//         match case {
//             Mode::Constant => Count::is(253, 0, 0, 0),
//             _ => Count::is(0, 0, 253, 254),
//         }
//     }
// }

// impl OutputMode<dyn ToBits<Boolean = Boolean<E>>> for Field {
//     type Case = Mode;

//     fn output_mode(case: &Self::Case) -> Mode {
//         match case {
//             Mode::Constant => Mode::Constant,
//             _ => Mode::Private,
//         }
//     }
// }

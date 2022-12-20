use crate::{
    bls12_377::{fp, Field as FieldTrait},
    circuit::traits::FromBits,
};

use super::*;

impl FromBits for Field {
    /// Initializes a new base field element from a list of **little-endian** bits.
    ///   - If `bits_le` is longer than `E::BaseField::size_in_bits()`, the excess bits are enforced to be `0`s.
    ///   - If `bits_le` is shorter than `E::BaseField::size_in_bits()`, it is padded with `0`s up to base field size.
    fn from_bits_le(bits_le: &[Boolean]) -> Self {
        // Retrieve the data and base field size.
        let size_in_data_bits = fp::CAPACITY as usize;
        let size_in_bits = fp::MODULUS_BITS as usize;

        // Ensure the list of booleans is within the allowed size in bits.
        let num_bits = bits_le.len();
        if num_bits > size_in_bits {
            // Check if all excess bits are zero.
            let should_be_zero = bits_le[size_in_bits..]
                .iter()
                .fold(Boolean::constant(false), |acc, bit| acc | bit);
            // Ensure `should_be_zero` is zero.
            Circuit::assert_eq(Circuit::zero(), should_be_zero);
        }

        // If `num_bits` is greater than `size_in_data_bits`, check it is less than `BaseField::MODULUS`.
        if num_bits > size_in_data_bits {
            // Retrieve the modulus & subtract by 1 as we'll check `bits_le` is less than or *equal* to this value.
            // (For advanced users) BaseField::MODULUS - 1 is equivalent to -1 in the field.
            let modulus_minus_one = -Fp::ONE;

            // As `bits_le[size_in_bits..]` is guaranteed to be zero from the above logic,
            // and `bits_le` is greater than `size_in_data_bits`, it is safe to truncate `bits_le` to `size_in_bits`.
            let bits_le = &bits_le[..size_in_bits];

            // Compute `!((BaseField::MODULUS - 1) < bits_le)`, which is equivalent to `bits_le < BaseField::MODULUS`.
            let is_less_than_modulus = !modulus_minus_one.to_bits_le().iter().zip_eq(bits_le).fold(
                Boolean::constant(false),
                |rest_is_less, (this, that)| {
                    if *this {
                        that.bitand(&rest_is_less)
                    } else {
                        that.bitor(&rest_is_less)
                    }
                },
            );

            // Ensure the field element is less than `BaseField::MODULUS`.
            Circuit::assert(is_less_than_modulus);
        }

        // Reconstruct the bits as a linear combination representing the original field value.
        // `output` := (2^i * b_i + ... + 2^0 * b_0)
        let mut output = Field::zero();
        let mut coefficient = Field::one();
        for bit in bits_le.iter().take(size_in_bits) {
            output += Field::from(**bit) * &coefficient;
            coefficient = coefficient.double();
        }

        // Construct the sanitized list of bits, resizing up if necessary.
        let mut bits_le = bits_le
            .iter()
            .take(size_in_bits)
            .cloned()
            .collect::<Vec<_>>();
        bits_le.resize(size_in_bits, Boolean::constant(false));

        // Store the little-endian bits in the output.
        if output.bits_le.set(bits_le).is_err() {
            Circuit::halt("Detected corrupt internal state for the bits of a field element")
        }

        output
    }

    /// Initializes a new base field element from a list of big-endian bits *without* leading zeros.
    fn from_bits_be(bits_be: &[Boolean]) -> Self {
        // Reverse the given bits from big-endian into little-endian.
        // Note: This is safe as the bit representation is consistent (there are no leading zeros).
        let mut bits_le = bits_be.to_vec();
        bits_le.reverse();

        Self::from_bits_le(&bits_le)
    }
}

// impl Metrics<dyn FromBits> for Field {
//     type Case = Vec<Mode>;

//     fn count(_modes: &Self::Case) -> Count {
//         todo!()
//     }
// }

// impl OutputMode<dyn FromBits> for Field {
//     type Case = Vec<Mode>;

//     fn output_mode(case: &Self::Case) -> Mode {
//         match case.iter().all(|mode| mode.is_constant()) {
//             true => Mode::Constant,
//             false => Mode::Private,
//         }
//     }
// }

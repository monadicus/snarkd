use crate::circuit::{
    traits::FromBits,
    types::{Boolean, Scalar},
};

impl FromBits for Scalar {
    /// Initializes a new scalar field element from a list of **little-endian** bits.
    ///   - If `bits_le` is longer than `E::ScalarField::size_in_bits()`, the excess bits are enforced to be `0`s.
    ///   - If `bits_le` is shorter than `E::ScalarField::size_in_bits()`, it is padded with `0`s up to scalar field size.
    fn from_bits_le(bits_le: &[Boolean]) -> Self {
        // // Note: We are reconstituting the scalar field into a base field.
        // // This is safe as the scalar field modulus is less than the base field modulus,
        // // and thus will always fit within a single base field element.
        // debug_assert!(console::Scalar::size_in_bits() < console::Field::size_in_bits());

        // // Retrieve the data and scalar field size.
        // let size_in_data_bits = console::Scalar::size_in_data_bits();
        // let size_in_bits = console::Scalar::size_in_bits();

        // // Ensure the list of booleans is within the allowed size in bits.
        // let num_bits = bits_le.len();
        // if num_bits > size_in_bits {
        //     // Check if all excess bits are zero.
        //     let should_be_zero = bits_le[size_in_bits..]
        //         .iter()
        //         .fold(Boolean::constant(false), |acc, bit| acc | bit);
        //     // Ensure `should_be_zero` is zero.
        //     Fp::assert_eq(Fp::zero(), should_be_zero);
        // }

        // // If `num_bits` is greater than `size_in_data_bits`, check it is less than `ScalarField::MODULUS`.
        // if num_bits > size_in_data_bits {
        //     // Retrieve the modulus & subtract by 1 as we'll check `bits_le` is less than or *equal* to this value.
        //     // (For advanced users) ScalarField::MODULUS - 1 is equivalent to -1 in the field.
        //     let modulus_minus_one = Scalar::constant(-console::Scalar::one());

        //     // Reconstruct the bits as a linear combination representing the original scalar as a field.
        //     let mut accumulator = Field::zero();
        //     let mut coefficient = Field::one();
        //     for bit in &bits_le[..size_in_bits] {
        //         accumulator += Field::from_boolean(bit) * &coefficient;
        //         coefficient = coefficient.double();
        //     }

        //     // As `bits_le[size_in_bits..]` is guaranteed to be zero from the above logic,
        //     // and `bits_le` is greater than `size_in_data_bits`, it is safe to truncate `bits_le` to `size_in_bits`.
        //     let scalar = Scalar {
        //         field: accumulator,
        //         bits_le: OnceCell::with_value(bits_le[..size_in_bits].to_vec()),
        //     };

        //     // Ensure the scalar is less than `ScalarField::MODULUS`.
        //     Fp::assert(scalar.is_less_than_or_equal(&modulus_minus_one));

        //     // Return the scalar.
        //     scalar
        // } else {
        //     // Construct the sanitized list of bits, resizing up if necessary.
        //     let mut bits_le = bits_le
        //         .iter()
        //         .take(size_in_bits)
        //         .cloned()
        //         .collect::<Vec<_>>();
        //     bits_le.resize(size_in_bits, Boolean::constant(false));

        //     // Reconstruct the bits as a linear combination representing the original scalar as a field.
        //     let mut accumulator = Field::zero();
        //     let mut coefficient = Field::one();
        //     for bit in &bits_le {
        //         accumulator += Field::from_boolean(bit) * &coefficient;
        //         coefficient = coefficient.double();
        //     }

        //     // Return the scalar.
        //     Scalar {
        //         field: accumulator,
        //         bits_le: OnceCell::with_value(bits_le),
        //     }
        // }
        todo!()
    }

    /// Initializes a new scalar field element from a list of big-endian bits *without* leading zeros.
    fn from_bits_be(bits_be: &[Boolean]) -> Self {
        // Reverse the given bits from big-endian into little-endian.
        // Note: This is safe as the bit representation is consistent (there are no leading zeros).
        let mut bits_le = bits_be.to_vec();
        bits_le.reverse();

        Self::from_bits_le(&bits_le)
    }
}

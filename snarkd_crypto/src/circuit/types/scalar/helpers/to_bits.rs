use crate::{
    bls12_377::{fp, scalar},
    circuit::{
        traits::ToBits,
        types::{Boolean, Scalar},
    },
};

impl ToBits for Scalar {
    /// Outputs the little-endian bit representation of `self` *without* trailing zeros.
    fn to_bits_le(&self) -> Vec<Boolean> {
        (&self).to_bits_le()
    }

    /// Outputs the big-endian bit representation of `self` *without* leading zeros.
    fn to_bits_be(&self) -> Vec<Boolean> {
        (&self).to_bits_be()
    }
}

impl ToBits for &Scalar {
    /// Outputs the little-endian bit representation of `self` *without* trailing zeros.
    fn to_bits_le(&self) -> Vec<Boolean> {
        // self.bits_le
        //     .get_or_init(|| {
        //         // Note: We are reconstituting the scalar field into a base field.
        //         // This is safe as the scalar field modulus is less than the base field modulus,
        //         // and thus will always fit within a single base field element.
        //         debug_assert!(scalar::MODULUS_BITS < fp::MODULUS_BITS);

        //         // Construct a vector of `Boolean`s comprising the bits of the scalar value.
        //         self.field.to_lower_bits_le(scalar::MODULUS_BITS)
        //     })
        //     .clone()
        todo!()
    }

    /// Outputs the big-endian bit representation of `self` *without* leading zeros.
    fn to_bits_be(&self) -> Vec<Boolean> {
        let mut bits_le = self.to_bits_le();
        bits_le.reverse();
        bits_le
    }
}

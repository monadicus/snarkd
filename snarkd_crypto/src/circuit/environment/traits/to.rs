use crate::circuit::types::{Boolean, Field};

/// Unary operator for converting to `k` number of bits.
pub trait ToLowerBits {
    ///
    /// Outputs the lower `k` bits of an `n`-bit element in little-endian representation.
    /// Enforces that the upper `n - k` bits are zero.
    ///
    fn to_lower_bits_le(&self, k: usize) -> Vec<Boolean>;

    ///
    /// Outputs the lower `k` bits of an `n`-bit element in big-endian representation.
    /// Enforces that the upper `n - k` bits are zero.
    ///
    fn to_lower_bits_be(&self, k: usize) -> Vec<Boolean>;
}

/// Unary operator for converting to `k` number of bits.
pub trait ToUpperBits {
    ///
    /// Outputs the upper `k` bits of an `n`-bit element in little-endian representation.
    /// Enforces that the lower `n - k` bits are zero.
    ///
    fn to_upper_bits_le(&self, k: usize) -> Vec<Boolean>;

    ///
    /// Outputs the upper `k` bits of an `n`-bit element in big-endian representation.
    /// Enforces that the lower `n - k` bits are zero.
    ///
    fn to_upper_bits_be(&self, k: usize) -> Vec<Boolean>;
}

/// Unary operator for converting to a base field.
pub trait ToField {
    /// Returns a circuit as a base field element.
    fn to_field(&self) -> Field;
}

/// Unary operator for converting to a list of base fields.
pub trait ToFields {
    /// Returns the circuit as a list of base field elements.
    fn to_fields(&self) -> Vec<Field>;
}

// TODO uncomment once Group type exists
// /// Unary operator for converting to an affine group.
// pub trait ToGroup {
//     /// Returns the circuit as a list of affine group elements.
//     fn to_group(&self) -> Group;
// }

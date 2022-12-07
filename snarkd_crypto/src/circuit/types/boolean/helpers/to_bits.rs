use crate::circuit::traits::ToBits;

use super::*;

impl ToBits for Boolean {
    /// Outputs `self` as a single-element vector.
    fn to_bits_le(&self) -> Vec<Boolean> {
        (&self).to_bits_le()
    }

    /// Outputs `self` as a single-element vector.
    fn to_bits_be(&self) -> Vec<Boolean> {
        (&self).to_bits_be()
    }
}

impl ToBits for &Boolean {
    /// Outputs `self` as a single-element vector.
    fn to_bits_le(&self) -> Vec<Boolean> {
        vec![(*self).clone()]
    }

    /// Outputs `self` as a single-element vector.
    fn to_bits_be(&self) -> Vec<Boolean> {
        vec![(*self).clone()]
    }
}

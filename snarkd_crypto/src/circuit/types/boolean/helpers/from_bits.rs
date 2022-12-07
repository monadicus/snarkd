use crate::circuit::traits::FromBits;

use super::*;

impl FromBits for Boolean {
    /// Returns a boolean circuit given a mode and single boolean.
    fn from_bits_le(bits_le: &[Boolean]) -> Self {
        // Ensure there is exactly one boolean in the list of booleans.
        match bits_le.len() == 1 {
            true => bits_le[0].clone(),
            false => Circuit::halt(format!(
                "Attempted to instantiate a boolean with {} bits",
                bits_le.len()
            )),
        }
    }

    /// Returns a boolean circuit given a mode and single boolean.
    fn from_bits_be(bits_be: &[Boolean]) -> Self {
        // Ensure there is exactly one boolean in the list of booleans.
        match bits_be.len() == 1 {
            true => bits_be[0].clone(),
            false => Circuit::halt(format!(
                "Attempted to instantiate a boolean with {} bits",
                bits_be.len()
            )),
        }
    }
}

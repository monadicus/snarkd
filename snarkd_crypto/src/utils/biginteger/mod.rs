use super::{
    bits::{FromBits, ToBits},
    rand::Uniform,
};

use std::fmt::{Debug, Display};

mod bigint_256;
pub use bigint_256::*;

mod bigint_384;
pub use bigint_384::*;

#[cfg(test)]
mod tests;

/// Calculate a + b + carry, returning the sum and modifying the
/// carry value.
#[inline(always)]
pub fn adc(a: &mut u64, b: u64, carry: u64) -> u64 {
    let tmp = u128::from(*a) + u128::from(b) + u128::from(carry);
    *a = tmp as u64;
    (tmp >> 64) as u64
}

/// set a = a - b - borrow, and returning the borrow value.
#[inline(always)]
pub fn sbb(a: &mut u64, b: u64, borrow: u64) -> u64 {
    let tmp = (1u128 << 64) + u128::from(*a) - u128::from(b) - u128::from(borrow);
    let carry = u64::from(tmp >> 64 == 0);
    *a = tmp as u64;
    carry
}

/// Calculate a + (b * c) + carry, returning the least significant digit
/// and setting carry to the most significant digit.
#[inline(always)]
pub fn mac_with_carry(a: u64, b: u64, c: u64, carry: &mut u64) -> u64 {
    let tmp = (u128::from(a)) + u128::from(b) * u128::from(c) + u128::from(*carry);

    *carry = (tmp >> 64) as u64;

    tmp as u64
}

/// Calculate a + b * c, returning the lower 64 bits of the result and setting
/// `carry` to the upper 64 bits.
#[inline(always)]
pub fn mac(a: u64, b: u64, c: u64, carry: &mut u64) -> u64 {
    let tmp = (u128::from(a)) + u128::from(b) * u128::from(c);

    *carry = (tmp >> 64) as u64;

    tmp as u64
}

/// Calculate a + b * c, discarding the lower 64 bits of the result and setting
/// `carry` to the upper 64 bits.
#[inline(always)]
pub fn mac_discard(a: u64, b: u64, c: u64, carry: &mut u64) {
    let tmp = (u128::from(a)) + u128::from(b) * u128::from(c);

    *carry = (tmp >> 64) as u64;
}

use crate::circuit::types::Boolean;

/// Unary operator for retrieving the inverse value.
pub trait Inverse {
    fn inverse(&self) -> Self;
}

/// Unary operator for retrieving the square root of the value.
pub trait SquareRoot {
    fn square_root(&self) -> Self;
}

///
/// A single-bit binary adder with a carry bit.
///
/// https://en.wikipedia.org/wiki/Adder_(electronics)#Full_adder
///
/// sum = (a XOR b) XOR carry
/// carry = a AND b OR carry AND (a XOR b)
/// return (sum, carry)
///
pub trait Adder {
    /// Returns the sum of `self` and `other` as a sum bit and carry bit.
    fn adder(&self, other: &Self, carry: &Self) -> (Self, Self)
    where
        Self: Sized;
}

///
/// A single-bit binary subtractor with a borrow bit.
///
/// https://en.wikipedia.org/wiki/Subtractor#Full_subtractor
///
/// difference = (a XOR b) XOR borrow
/// borrow = ((NOT a) AND b) OR (borrow AND (NOT (a XOR b)))
/// return (difference, borrow)
///
pub trait Subtractor {
    /// Returns the difference of `self` and `other` as a difference bit and borrow bit.
    fn subtractor(&self, other: &Self, borrow: &Self) -> (Self, Self)
    where
        Self: Sized;
}

/// Representation of the zero value.
pub trait Zero {
    /// Returns a new zero constant.
    fn zero() -> Self
    where
        Self: Sized;

    /// Returns `true` if `self` is zero.
    fn is_zero(&self) -> Boolean;
}

/// Representation of the one value.
pub trait One {
    /// Returns a new one constant.
    fn one() -> Self
    where
        Self: Sized;

    /// Returns `true` if `self` is one.
    fn is_one(&self) -> Boolean;
}

/// Unary operator for retrieving the most-significant bit.
pub trait MSB {
    /// Returns the MSB of the value.
    fn msb(&self) -> Boolean;
}

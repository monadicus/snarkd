use bitvec::prelude::*;
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Field:
    Add + AddAssign + Sub + SubAssign + Mul + MulAssign + Neg + Div + DivAssign + Sized
{
    const PHI: Self;

    /// Returns the additive identity of the field.
    fn zero() -> Self;

    /// Returns whether or not the given element is zero.
    fn is_zero(&self) -> bool;

    /// Returns the multiplicative identity of the field.
    fn one() -> Self;

    /// Returns whether or not the given element is one.
    fn is_one(&self) -> bool;

    /// Returns the characteristic of the field.
    fn characteristic<'a>() -> Self;

    /// Returns `self + self`.
    #[must_use]
    fn double(&self) -> Self;

    /// Doubles `self` in place.
    fn double_in_place(&mut self);

    /// Returns `self * self`.
    #[must_use]
    fn square(&self) -> Self;

    /// Squares `self` in place.
    fn square_in_place(&mut self) -> &mut Self;

    fn sum_of_products(
        a: impl Iterator<Item = Self> + Clone,
        b: impl Iterator<Item = Self> + Clone,
    ) -> Self {
        a.zip(b).map(|(a, b)| *a * b).sum::<Self>()
    }

    /// Computes the multiplicative inverse of `self` if `self` is nonzero.
    #[must_use]
    fn inverse(&self) -> Option<Self>;

    /// Sets `self` to `self`'s inverse if it exists. Otherwise it is a no-op.
    fn inverse_in_place(&mut self) -> Option<&mut Self>;

    /// Exponentiates this element by a power of the base prime modulus via
    /// the Frobenius automorphism.
    fn frobenius_map(&mut self, power: usize);

    /// Performs the GLV endomorphism.
    fn glv_endomorphism(&self) -> Self;

    /// Exponentiates this element by a number represented with `u64` limbs,
    /// least significant limb first.
    #[must_use]
    fn pow(&self, exp: Self) -> Self {
        let mut res = Self::one();

        let mut found_one = false;

        for i in BitVec::<Msb0, u8>::from(exp.to_be_bytes()) {
            if !found_one {
                if i {
                    found_one = true;
                } else {
                    continue;
                }
            }

            res.square_in_place();

            if i {
                res *= self;
            }
        }
        res
    }
}

use bitvec::prelude::*;
use core::{
    fmt::Debug,
    hash::Hash,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};
use std::ops::Deref;

pub trait Field:
    Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> MulAssign<&'a Self>
    + for<'a> Div<&'a Self, Output = Self>
    + for<'a> DivAssign<&'a Self>
    + Neg<Output = Self>
    + Sum<Self>
    + Copy
    + Eq
    + Hash
    + Debug
    + Sized
    + Send
    + Sync
{
    const PHI: Self;
    const ZERO: Self;
    const ONE: Self;

    /// Returns whether or not the given element is zero.
    fn is_zero(&self) -> bool;

    /// Returns whether or not the given element is one.
    fn is_one(&self) -> bool;

    fn half() -> Self;

    /// Returns a random field element.
    fn rand() -> Self;

    /// Returns the characteristic of the field.
    fn characteristic() -> Vec<u64>;

    /// Returns `self + self`.
    #[must_use]
    fn double(&self) -> Self;

    /// Doubles `self` in place.
    fn double_in_place(&mut self);

    /// Returns `self * self`.
    #[must_use]
    fn square(&self) -> Self;

    /// Squares `self` in place.
    fn square_in_place(&mut self);

    fn sum_of_products(a: impl Iterator<Item = Self>, b: impl Iterator<Item = Self>) -> Self {
        a.zip(b).map(|(a, b)| a * b).sum::<Self>()
    }

    /// Computes the multiplicative inverse of `self` if `self` is nonzero.
    #[must_use]
    fn inverse(&self) -> Option<Self>;

    /// Sets `self` to `self`'s inverse if it exists. Otherwise it is a no-op.
    fn inverse_in_place(&mut self) -> Option<&mut Self>;

    /// Computes the square root of `self`, and returns `None` if it does not exist.
    fn sqrt(&self) -> Option<Self>;

    /// Performs the GLV endomorphism.
    fn glv_endomorphism(&self) -> Self;

    /// Exponentiates this element by a number represented with `u64` limbs,
    /// least significant limb first.
    #[must_use]
    fn pow(&self, exp: &[u64]) -> Self {
        exp.iter()
            .flat_map(|limb| limb.view_bits::<Lsb0>())
            .rev()
            .skip_while(|i| !i.deref())
            .fold(Self::ONE, |mut res, i| {
                res.square_in_place();
                if *i {
                    res * self
                } else {
                    res
                }
            })
    }
}

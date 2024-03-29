use super::{Affine, Parameters, Scalar};
use core::{
    fmt::{Debug, Display},
    iter,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub trait Projective:
    Neg<Output = Self>
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> SubAssign<&'a Self>
    + Mul<Scalar, Output = Self>
    + MulAssign<Scalar>
    + iter::Sum
    + Copy
    + Clone
    + PartialEq
    + Eq
    + Debug
    + Display
    + Sized
    + Send
    + Sync
{
    type Affine: Affine<Projective = Self>;
    type Parameters: Parameters;

    const ZERO: Self;

    fn rand() -> Self;

    fn is_zero(&self) -> bool;

    fn prime_subgroup_generator() -> Self;

    fn cofactor() -> &'static [u64];

    fn is_normalized(&self) -> bool;

    fn batch_normalization(v: &mut [Self]);

    fn add_mixed(&self, other: &Self::Affine) -> Self;

    fn add_assign_mixed(&mut self, other: &Self::Affine);

    fn double(&self) -> Self;

    fn double_in_place(&mut self);

    fn to_affine(&self) -> Self::Affine;
}

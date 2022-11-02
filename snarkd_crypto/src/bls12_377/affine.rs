use super::{Fr, Group, Projective};
use bitvec::prelude::*;
use core::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub trait Affine:
    Neg<Output = Self> + Mul<Fr, Output = Self::Projective> + Debug + Display + PartialEq + Eq + Sized
{
    type Projective: Projective<Affine = Self>;
    type Parameters: Group;

    fn zero() -> Self;

    fn is_zero(&self) -> bool;

    fn from_coordinates(
        x: <Self::Parameters as Group>::BaseField,
        y: <Self::Parameters as Group>::BaseField,
        infinity: bool,
    ) -> Self;

    fn rand() -> Self;

    fn cofactor() -> &'static [u64];

    fn prime_subgroup_generator() -> Self;

    fn from_x_coordinate(x: <Self::Parameters as Group>::BaseField, greatest: bool)
        -> Option<Self>;

    fn from_y_coordinate(y: <Self::Parameters as Group>::BaseField, greatest: bool)
        -> Option<Self>;

    fn mul_bits(&self, bits: &BitSlice<u8, Msb0>) -> Self::Projective;

    fn mul_by_cofactor_to_projective(&self) -> Self::Projective;

    fn mul_by_cofactor(&self) -> Self;

    fn mul_by_cofactor_inv(&self) -> Self;

    fn to_projective(&self) -> Self::Projective;

    fn is_on_curve(&self) -> bool;

    fn batch_add_loop_1(
        a: &mut Self,
        b: &mut Self,
        half: &<Self::Parameters as Group>::BaseField,
        inversion_tmp: &mut <Self::Parameters as Group>::BaseField,
    );

    fn batch_add_loop_2(
        a: &mut Self,
        b: Self,
        inversion_tmp: &mut <Self::Parameters as Group>::BaseField,
    );
}

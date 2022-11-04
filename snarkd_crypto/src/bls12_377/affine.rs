use super::{Group, Projective, Scalar};

use core::{
    fmt::{Debug, Display},
    ops::{Mul, Neg},
};

pub trait Affine:
    Neg<Output = Self>
    + Mul<Scalar, Output = Self::Projective>
    + Debug
    + Display
    + PartialEq
    + Eq
    + Sized
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

    fn mul_bits(&self, bits: Vec<bool>) -> Self::Projective;

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

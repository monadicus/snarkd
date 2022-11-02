use super::{field::Field, Fr};
use core::{fmt::Display, hash::Hash};

pub trait Group: Hash + Clone + Copy {
    type BaseField: Field + Ord + Display;

    const COFACTOR: &'static [u64];
    const COFACTOR_INV: Fr;
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField);
    const A: Self::BaseField;
    const B: Self::BaseField;
}

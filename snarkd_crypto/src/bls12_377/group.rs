use super::{field::Field, Fr};
use core::{
    fmt::{Debug, Display},
    hash::Hash,
};

pub trait Group: Hash + Clone + Copy + Debug + PartialEq + Eq {
    type BaseField: Field + Ord + Display;

    const COFACTOR: &'static [u64];
    const COFACTOR_INV: Fr;
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField);
    const B: Self::BaseField;
}

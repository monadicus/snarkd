use super::{field::Field, Scalar};
use core::{
    fmt::{Debug, Display},
    hash::Hash,
};

pub trait Parameters: Hash + Clone + Copy + Debug + PartialEq + Eq {
    #[cfg(feature = "fuzz")]
    type BaseField: Field + Ord + Display + for<'a> arbitrary::Arbitrary<'a>;
    #[cfg(not(feature = "fuzz"))]
    type BaseField: Field + Ord + Display;

    const COFACTOR: &'static [u64];
    const COFACTOR_INV: Scalar;
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField);
    const B: Self::BaseField;
}

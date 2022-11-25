use super::{field::Field, Scalar};
use core::{
    fmt::{Debug, Display},
    hash::Hash,
};

pub trait Parameters: Hash + Clone + Copy + Debug + PartialEq + Eq {
    #[cfg(feature = "arbitrary")]
    type BaseField: Field + Ord + Display + for<'a> arbitrary::Arbitrary<'a>;
    #[cfg(feature = "test")]
    type BaseField: Field + Ord + Display + serde::Serialize + serde::de::DeserializeOwned;
    #[cfg(not(feature = "arbitrary"))]
    type BaseField: Field + Ord + Display;

    const COFACTOR: &'static [u64];
    const COFACTOR_INV: Scalar;
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField);
    const B: Self::BaseField;
}

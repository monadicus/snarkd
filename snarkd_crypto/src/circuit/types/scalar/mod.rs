use super::{Boolean, Field};
use crate::circuit::{helpers::LinearCombination, traits::ToField};
use once_cell::unsync::OnceCell;

pub mod add;
pub mod compare;
pub mod equal;
mod helpers;
pub mod ternary;

#[derive(Clone)]
pub struct Scalar {
    /// The primary representation of the scalar element.
    field: Field,
    /// An optional secondary representation in little-endian bits is provided,
    /// so that calls to `ToBits` only incur constraint costs once.
    bits_le: OnceCell<Vec<Boolean>>,
}

impl From<Scalar> for LinearCombination {
    fn from(scalar: Scalar) -> Self {
        From::from(&scalar)
    }
}

impl From<&Scalar> for LinearCombination {
    fn from(scalar: &Scalar) -> Self {
        scalar.to_field().into()
    }
}

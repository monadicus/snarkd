// use crate::{BooleanTrait, FieldTrait, GroupTrait, ScalarTrait};

// /// Unary operator for instantiating from a boolean.
// pub trait FromBoolean {
//     type Boolean: BooleanTrait;

//     fn from_boolean(boolean: &Self::Boolean) -> Self
//     where
//         Self: Sized;
// }

use crate::circuit::types::Boolean;

/// Unary operator for instantiating from bits.
pub trait FromBits {
    fn from_bits_le(bits_le: &[Boolean]) -> Self
    where
        Self: Sized;

    fn from_bits_be(bits_be: &[Boolean]) -> Self
    where
        Self: Sized;
}

// /// Unary operator for converting from a base field element.
// pub trait FromField {
//     type Field: FieldTrait;

//     /// Casts a circuit from a base field element.
//     fn from_field(field: Self::Field) -> Self;
// }

// /// Unary operator for converting from a list of base elements.
// pub trait FromFields {
//     type Field: FieldTrait;

//     /// Casts a circuit from a list of base field elements.
//     fn from_fields(fields: &[Self::Field]) -> Self;
// }

// /// Unary operator for converting from an affine group element.
// pub trait FromGroup {
//     type Group: GroupTrait<Self::Scalar>;
//     type Scalar: ScalarTrait;

//     /// Casts a circuit from an affine group element.
//     fn from_group(group: Self::Group) -> Self;
// }

use crate::circuit::{
    traits::Ternary,
    types::{Boolean, Field},
};

use super::Scalar;

// impl Ternary for Scalar {
//     /// Returns `first` if `condition` is `true`, otherwise returns `second`.
//     fn ternary(condition: &Boolean, first: &Self, second: &Self) -> Self {
//         // Compute the ternary over the field representation (for efficiency).
//         let field = Field::ternary(condition, &first.field, &second.field);
//         // Return the result.
//         Self {
//             field,
//             bits_le: Default::default(),
//         }
//     }
// }

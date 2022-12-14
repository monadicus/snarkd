use crate::circuit::{
    traits::ToField,
    types::{Field, Scalar},
};

impl ToField for Scalar {
    /// Casts a scalar field element into a base field element.
    fn to_field(&self) -> Field {
        self.field.clone()
    }
}

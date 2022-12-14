use crate::circuit::{
    traits::{ToField, ToFields},
    types::{Field, Scalar},
};

impl ToFields for Scalar {
    /// Casts a string into a list of base fields.
    fn to_fields(&self) -> Vec<Field> {
        vec![self.to_field()]
    }
}

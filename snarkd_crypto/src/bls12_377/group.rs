use super::field::Field;

pub trait Group {
    type BaseField: Field;

    const COFACTOR: &'static [u64];
    const COFACTOR_INV: &'static [u64];
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField);
    const A: Self::BaseField;
    const B: Self::BaseField;
}

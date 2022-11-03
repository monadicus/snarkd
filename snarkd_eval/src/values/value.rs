use super::*;

#[derive(Clone, Debug)]
pub enum ConstrainedValue<F: Field, G: Group> {
    Address(ConstrainedAddress<G>),
    Boolean(ConstrainedBool),
    Field(ConstrainedField<F>),
    Group(ConstrainedGroup<G>),
    Integer(ConstrainedInteger),
    Scalar(ConstrainedScalar),
    String(ConstrainedString),
    Struct(ConstrainedStructure<F, G>),
    Record(ConstrainedRecord<F, G>),
}

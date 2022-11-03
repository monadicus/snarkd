use indexmap::IndexMap;

use super::*;

#[derive(Clone, Debug)]
pub struct ConstrainedStructure<F: Field, G: Group> {
    values: IndexMap<String, ConstrainedValue<F, G>>,
}

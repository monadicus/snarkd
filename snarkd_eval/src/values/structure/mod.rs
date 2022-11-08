use indexmap::IndexMap;

use super::*;

#[derive(Clone, Debug)]
pub struct ConstrainedStructure<F: Field, G: Parameters> {
    values: IndexMap<String, ConstrainedValue<F, G>>,
}

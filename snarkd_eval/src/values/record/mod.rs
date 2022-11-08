use indexmap::IndexMap;
use snarkd_ir::Visibility;

use super::*;

#[derive(Clone, Debug)]
pub struct ConstrainedRecord<F: Field, G: Parameters> {
    values: IndexMap<String, (ConstrainedValue<F, G>, Visibility)>,
}

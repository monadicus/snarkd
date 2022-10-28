use indexmap::IndexMap;
use snarkd_ir::Visibility;

use super::{group::GroupType, value::ConstrainedValue};

#[derive(Clone, Debug)]
pub struct Record<F, G: GroupType<F>> {
    values: IndexMap<String, (ConstrainedValue<F, G>, Visibility)>,
}

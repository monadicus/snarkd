use indexmap::IndexMap;

use super::{value::ConstrainedValue, group::GroupType};

#[derive(Clone, Debug)]
pub struct Structure<F, G: GroupType<F>> {
    values: IndexMap<String, ConstrainedValue<F, G>>,
}

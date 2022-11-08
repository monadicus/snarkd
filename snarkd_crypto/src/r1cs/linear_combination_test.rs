use crate::{Fp, Index};

use super::*;

#[test]
fn linear_combination_append() {
    let mut combo = LinearCombination::<Fp>::zero();
    for i in 0..100u64 {
        combo += (i.into(), Variable::new_unchecked(Index::Public(0)));
    }
    assert_eq!(combo.0.len(), 1);
}

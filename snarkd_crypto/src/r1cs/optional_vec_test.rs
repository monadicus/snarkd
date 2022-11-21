use crate::{optional_vec, OptionalVec};

#[test]
fn insert_remove() {
    let mut v = optional_vec!(0, 1, 2, 3, 4);
    v.remove(1);
    v.remove(3);
    v.insert(5);

    assert!(!v.is_empty());
    assert_eq!(v.len(), 4);
    assert_eq!(v.next_idx(), 1);

    v[0] = 1;
    assert_eq!(v[0], 1);

    let vals = v.iter().copied().collect::<Vec<_>>();
    assert_eq!(vals, [1, 2, 5, 4]);
}

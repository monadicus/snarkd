use crate::r1cs::Variable;

#[test]
fn test_size() {
    assert_eq!(16, std::mem::size_of::<Variable>());
}

#[test]
fn test_size() {
    assert_eq!(
        24,
        std::mem::size_of::<Variable<<Circuit as Environment>::BaseField>>()
    );
}

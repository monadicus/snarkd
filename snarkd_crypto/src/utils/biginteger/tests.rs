use crate::{
    biginteger::*,
    rand::{TestRng, Uniform},
};

#[allow(clippy::eq_op)]
fn biginteger_arithmetic_test<B: BigInteger>(a: B, b: B, zero: B) {
    // zero == zero
    assert!(zero == zero);

    // zero.is_zero() == true
    assert!(zero.is_zero());

    // a == a
    assert!(a == a);

    // a + 0 = a
    let mut a0_add = a;
    a0_add.add_nocarry(&zero);
    assert_eq!(a0_add, a);

    // a - 0 = a
    let mut a0_sub = a;
    a0_sub.sub_noborrow(&zero);
    assert_eq!(a0_sub, a);

    // a - a = 0
    let mut aa_sub = a;
    aa_sub.sub_noborrow(&a);
    assert_eq!(aa_sub, zero);

    // a + b = b + a
    let mut ab_add = a;
    ab_add.add_nocarry(&b);
    let mut ba_add = b;
    ba_add.add_nocarry(&a);
    assert_eq!(ab_add, ba_add);
}

fn biginteger_bits_test<B: BigInteger>() {
    let mut one = B::from(1u64);
    assert!(one.get_bit(0));
    assert!(!one.get_bit(1));
    one.muln(5);

    let thirty_two = one;
    assert!(!thirty_two.get_bit(0));
    assert!(!thirty_two.get_bit(1));
    assert!(!thirty_two.get_bit(2));
    assert!(!thirty_two.get_bit(3));
    assert!(!thirty_two.get_bit(4));
    assert!(thirty_two.get_bit(5), "{:?}", thirty_two);
}

fn biginteger_bytes_test<B: BigInteger>(rng: &mut TestRng) {
    let mut bytes = [0u8; 256];
    let x: B = Uniform::rand(rng);
    x.write_le(bytes.as_mut()).unwrap();
    let y = B::read_le(bytes.as_ref()).unwrap();
    assert_eq!(x, y);
}

fn biginteger_to_string_test<B: BigInteger>(rng: &mut TestRng) {
    const ITERATIONS: u64 = 1_000_000;

    // Sanity check the integers starting from 0 to ITERATIONS.
    for integer in 0..ITERATIONS {
        assert_eq!(format!("{}", integer), B::from(integer).to_string());
    }

    // If the BigInteger has more than one limb, sanity check
    // that the second limb conversion is also correct.
    if B::NUM_LIMBS > 1 {
        let start = u64::MAX as u128;
        for integer in start..(start + ITERATIONS as u128) {
            let mut buffer = vec![0u8; 8 * B::NUM_LIMBS];
            buffer
                .iter_mut()
                .zip(&(integer as u128).to_le_bytes())
                .for_each(|(buf, val)| {
                    *buf = *val;
                });
            assert_eq!(
                format!("{}", integer),
                B::read_le(&*buffer).unwrap().to_string()
            );
        }
    }
}

fn test_biginteger<B: BigInteger>(zero: B) {
    let mut rng = TestRng::default();

    let a: B = Uniform::rand(&mut rng);
    let b: B = Uniform::rand(&mut rng);
    biginteger_arithmetic_test(a, b, zero);
    biginteger_bytes_test::<B>(&mut rng);
    biginteger_bits_test::<B>();
    biginteger_to_string_test::<B>(&mut rng);
}

#[test]
fn test_biginteger256() {
    test_biginteger(BigInteger256::new([0u64; 4]));
}

#[test]
fn test_biginteger384() {
    test_biginteger(BigInteger384::new([0u64; 6]));
}

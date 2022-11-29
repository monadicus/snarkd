mod fp;
pub use fp::*;

mod fp2;
pub use fp2::*;

mod fp6;
pub use fp6::*;

mod fp12;
pub use fp12::*;

mod scalar;
pub use scalar::*;

use test_runner::TestResult;

use crate::bls12_377::Field;

pub fn neg<F: Field>(a: F) -> TestResult {
    let mut outputs = Vec::new();
    let mut b = -a;
    outputs.push(b);
    b += &a;

    assert!(b.is_zero());
    outputs.push(b);
    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn add<F: Field>(a: F, b: F, c: F) -> TestResult {
    let mut outputs = Vec::new();

    let t0 = (a + b) + c; // (a + b) + c
    outputs.push(t0);
    let t1 = (a + c) + b; // (a + c) + b
    outputs.push(t1);
    let t2 = (b + c) + a; // (b + c) + a
    outputs.push(t2);

    assert_eq!(t0, t1);
    assert_eq!(t1, t2);
    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn add_assign<F: Field>(a: F, b: F, c: F) -> TestResult {
    let mut outputs = Vec::new();

    let mut tmp1 = a;
    tmp1.add_assign(b);
    outputs.push(tmp1);
    tmp1.add_assign(c);
    outputs.push(tmp1);

    let mut tmp2 = b;
    tmp2.add_assign(c);
    outputs.push(tmp2);
    tmp2.add_assign(a);
    outputs.push(tmp2);

    // assert!(tmp1.is_valid());
    // assert!(tmp2.is_valid());
    assert_eq!(tmp1, tmp2);
    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn sub<F: Field>(a: F, b: F) -> TestResult {
    let mut outputs = Vec::new();

    let t0 = a - b; // (a - b)
    outputs.push(t0);

    let mut t1 = b; // (b - a)
    t1 -= &a;
    outputs.push(t1);

    let mut t2 = t0; // (a - b) + (b - a) = 0
    t2 += &t1;
    outputs.push(t2);

    assert!(t2.is_zero());
    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn mul<F: Field>(a: F, b: F, c: F) -> TestResult {
    let mut outputs = Vec::new();

    let mut t0 = a; // (a * b) * c
    t0 *= &b;
    outputs.push(t0);
    t0 *= &c;
    outputs.push(t0);

    let mut t1 = a; // (a * c) * b
    t1 *= &c;
    outputs.push(t1);
    t1 *= &b;
    outputs.push(t1);

    let mut t2 = b; // (b * c) * a
    t2 *= &c;
    outputs.push(t2);
    t2 *= &a;
    outputs.push(t2);

    assert_eq!(t0, t1);
    assert_eq!(t1, t2);
    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn mul_assign<F: Field>(a: F, b: F, c: F) -> TestResult {
    let mut outputs = Vec::new();

    let mut tmp1 = a;
    tmp1.mul_assign(&b);
    outputs.push(tmp1);
    tmp1.mul_assign(&c);
    outputs.push(tmp1);

    let mut tmp2 = b;
    tmp2.mul_assign(&c);
    outputs.push(tmp2);
    tmp2.mul_assign(&a);
    outputs.push(tmp2);

    assert_eq!(tmp1, tmp2);

    // let r = Fp::rand();
    //     let mut a = Fp::rand();
    //     let mut b = Fp::rand();
    //     let mut c = Fp::rand();

    //     let mut tmp1 = a;
    //     tmp1.add_assign(b);
    //     tmp1.add_assign(c);
    //     tmp1.mul_assign(&r);

    //     a.mul_assign(&r);
    //     b.mul_assign(&r);
    //     c.mul_assign(&r);

    //     a.add_assign(b);
    //     a.add_assign(c);

    //     assert_eq!(tmp1, a);

    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn inversion<F: Field>(mut a: F) -> TestResult {
    let mut outputs = Vec::new();

    let b = a.inverse(); // probablistically nonzero
    match b {
        Some(b) => outputs.push(b.to_string()),
        None => outputs.push("None".into()),
    }
    if let Some(b) = b {
        a *= &b;
        outputs.push(a.to_string());
        assert_eq!(a, F::ONE);
    }

    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn double<F: Field>(mut a: F) -> TestResult {
    let mut outputs = Vec::new();

    let mut b = a;
    a += &b;
    outputs.push(a);
    b.double_in_place();
    outputs.push(b);

    assert_eq!(a, b);
    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn square<F: Field>(mut a: F) -> TestResult {
    let mut outputs = Vec::new();

    let mut b = a;
    a *= &b;
    outputs.push(a);
    b.square_in_place();
    outputs.push(b);

    assert_eq!(a, b);
    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn expansion<F: Field>(a: F, b: F, c: F, d: F) -> TestResult {
    let mut outputs = Vec::new();

    // Compare (a + b)(c + d) and (a*c + b*c + a*d + b*d)
    let mut t0 = a;
    t0 += &b;
    outputs.push(t0);

    let mut t1 = c;
    t1 += &d;
    outputs.push(t1);
    t0 *= &t1;
    outputs.push(t0);

    let mut t2 = a;
    t2 *= &c;
    outputs.push(t2);

    let mut t3 = b;
    t3 *= &c;
    outputs.push(t3);

    let mut t4 = a;
    t4 *= &d;
    outputs.push(t4);

    let mut t5 = b;
    t5 *= &d;
    outputs.push(t5);

    t2 += &t3;
    outputs.push(t2);
    t2 += &t4;
    outputs.push(t2);
    t2 += &t5;
    outputs.push(t2);

    assert_eq!(t0, t2);

    // Compare (a + b)c and (a*c + b*c)
    let t0 = (a + b) * c;
    outputs.push(t0);
    let t2 = a * c + (b * c);
    outputs.push(t2);

    assert_eq!(t0, t2);
    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

#[macro_export]
macro_rules! frobenius {
    ($a:expr, $field:ty) => {{
        use $crate::bls12_377::field::Field;

        let mut outputs = Vec::new();

        let mut a_0 = $a;
        a_0.frobenius_map(0);
        outputs.push(a_0);

        assert_eq!($a, a_0);

        let mut a_q = $a.pow(&<$field>::characteristic());
        outputs.push(a_q);
        for power in 1..13 {
            let mut a_qi = $a;
            a_qi.frobenius_map(power);
            outputs.push(a_qi);
            assert_eq!(a_qi, a_q);

            a_q = a_q.pow(&<$field>::characteristic());
        }

        Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
    }};
}

pub fn sqrt<F: Field>(a: F) -> TestResult {
    let mut outputs = Vec::new();

    let square = a.square();
    outputs.push(square);
    let sqrt = square.sqrt().unwrap();
    outputs.push(sqrt);
    assert!(sqrt == a || sqrt == -a);
    if let Some(sqrt) = a.sqrt() {
        outputs.push(sqrt);
        assert!(sqrt.square() == a || sqrt.square() == -a);
    }

    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn pow<F: Field>(a: F) -> TestResult {
    let mut outputs = Vec::new();

    let a_cubed = a * a * a;
    let a_pow_3_one = a.pow(&[0x3, 0x0, 0x0, 0x0]);
    outputs.push(a_pow_3_one);
    let a_pow_3_two = a.pow(&[0x0, 0x3, 0x0, 0x0]);
    outputs.push(a_pow_3_two);
    assert_eq!(a_cubed, a_pow_3_one);
    assert_eq!(a_pow_3_one, a_pow_3_two);

    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn sum_of_products<F: Field>(a: Vec<F>, b: Vec<F>) -> TestResult {
    let mut outputs = Vec::new();

    let sum = F::sum_of_products(a.clone().into_iter(), b.clone().into_iter());
    outputs.push(sum);
    let actual = a.into_iter().zip(b).map(|(a, b)| a * b).sum();
    outputs.push(actual);
    assert_eq!(sum, actual);

    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

#[allow(clippy::eq_op)]
pub fn math_properties<F: Field>(a: F, b: F) -> TestResult {
    let zero = F::ZERO;
    let one = F::ONE;
    let two = one + one;

    // a == a
    assert!(a == a);
    // a + 0 = a
    assert_eq!(a + zero, a);
    // a - 0 = a
    assert_eq!(a - zero, a);
    // a - a = 0
    assert_eq!(a - a, zero);
    // 0 - a = -a
    assert_eq!(zero - a, -a);
    // a.double() = a + a
    assert_eq!(a.double(), a + a);
    // b.double() = b + b
    assert_eq!(b.double(), b + b);
    // a + b = b + a
    assert_eq!(a + b, b + a);
    // a - b = -(b - a)
    assert_eq!(a - b, -(b - a));
    // (a + b) + a = a + (b + a)
    assert_eq!((a + b) + a, a + (b + a));
    // (a + b).double() = (a + b) + (b + a)
    assert_eq!((a + b).double(), (a + b) + (b + a));

    // a * 0 = 0
    assert_eq!(a * zero, zero);
    // a * 1 = a
    assert_eq!(a * one, a);
    // a * 2 = a.double()
    assert_eq!(a * two, a.double());
    // a * a^-1 = 1
    if !a.is_zero() {
        assert_eq!(a * a.inverse().unwrap(), one);
    }
    // a * a = a^2
    assert_eq!(a * a, a.square());
    // a * a * a = a^3
    assert_eq!(a * (a * a), a.pow(&[0x3, 0x0, 0x0, 0x0]));
    // a * b = b * a
    assert_eq!(a * b, b * a);
    // (a * b) * a = a * (b * a)
    assert_eq!((a * b) * a, a * (b * a));
    // (a + b)^2 = a^2 + 2ab + b^2
    assert_eq!(
        (a + b).square(),
        a.square() + ((a * b) + (a * b)) + b.square()
    );
    // (a - b)^2 = (-(b - a))^2
    assert_eq!((a - b).square(), (-(b - a)).square());

    if !a.is_zero() {
        let mut c = a;
        c.inverse_in_place();
        assert_eq!(a * c, one);
        assert_eq!(a / a, one);
    }

    assert!(F::ZERO.is_zero());
    {
        let z = -F::ZERO;
        assert!(z.is_zero());
    }

    assert!(F::ZERO.inverse().is_none());

    // Multiplication by zero
    {
        let a = F::rand() * F::ZERO;
        assert!(a.is_zero());
    }

    // Addition by zero
    {
        let mut a = F::rand();
        let copy = a;
        a += &F::ZERO;
        assert_eq!(a, copy);
    }

    Ok("Pass".into())
}

#[allow(clippy::eq_op)]
pub fn zero_one_two<F: Field>() {
    let zero = F::ZERO;
    assert!(zero == zero);
    assert!(zero.is_zero()); // true
    assert!(!zero.is_one()); // false

    let one = F::ONE;
    assert!(one == one);
    assert!(!one.is_zero()); // false
    assert!(one.is_one()); // true
    assert_eq!(zero + one, one);

    let two = one + one;
    assert!(two == two);
    assert_ne!(zero, two);
    assert_ne!(one, two);
}

pub fn sqrt_1_to_100<F: Field>() {
    let mut c = F::ONE;
    for _ in 0..100 {
        let mut b = c.square();
        // assert_eq!(b.legendre(), LegendreSymbol::QuadraticResidue);

        b = b.sqrt().unwrap();

        if b != c {
            b = -b;
        }

        assert_eq!(b, c);

        c += &F::ONE;
    }
}

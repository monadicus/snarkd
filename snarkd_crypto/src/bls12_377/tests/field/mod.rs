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

use serde_json::Value;

use crate::bls12_377::Field;

pub fn neg<F: Field>(a: F) -> Result<Value, String> {
    let mut outputs = Vec::new();
    let mut b = -a;
    outputs.push(b);
    b += &a;

    assert!(b.is_zero());
    outputs.push(b);
    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn add<F: Field>(a: F, b: F, c: F) -> Result<Value, String> {
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

pub fn sub<F: Field>(a: F, b: F) -> Result<Value, String> {
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

pub fn mul<F: Field>(a: F, b: F, c: F) -> Result<Value, String> {
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

pub fn inversion<F: Field>(mut a: F) -> Result<Value, String> {
    let mut outputs = Vec::new();

    let b = a.inverse().unwrap(); // probablistically nonzero
    outputs.push(b);
    a *= &b;
    outputs.push(a);

    assert_eq!(a, F::ONE);
    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn double<F: Field>(mut a: F) -> Result<Value, String> {
    let mut outputs = Vec::new();

    let mut b = a;
    a += &b;
    outputs.push(a);
    b.double_in_place();
    outputs.push(b);

    assert_eq!(a, b);
    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn square<F: Field>(mut a: F) -> Result<Value, String> {
    let mut outputs = Vec::new();

    let mut b = a;
    a *= &b;
    outputs.push(a);
    b.square_in_place();
    outputs.push(b);

    assert_eq!(a, b);
    Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
}

pub fn expansion<F: Field>(a: F, b: F, c: F, d: F) -> Result<Value, String> {
    let mut outputs = Vec::new();

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

pub fn sqrt<F: Field>(a: F) -> Result<Value, String> {
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

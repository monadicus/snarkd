use serde_json::Value;
use test_runner::{Namespace, Test};

use crate::bls12_377::{field::Field, Affine, G1Projective, G2Projective, Projective, Scalar};

fn add<G: Projective>(a: G, b: G, c: G) -> Result<Value, String> {
    let mut outputs = Vec::new();

    let a_affine = a.to_affine();
    let b_affine = b.to_affine();
    let c_affine = c.to_affine();

    // a + a should equal the doubling
    {
        let mut aplusa = a;
        aplusa += a;
        outputs.push(aplusa.to_string());

        let mut aplusamixed = a;
        aplusamixed.add_assign_mixed(&a.to_affine());
        outputs.push(aplusamixed.to_string());

        let mut adouble = a;
        adouble.double_in_place();
        outputs.push(adouble.to_string());

        assert_eq!(aplusa, adouble);
        assert_eq!(aplusa, aplusamixed);
    }

    let mut tmp: G;

    // (a + b) + c
    tmp = (a + b) + c;
    outputs.push(tmp.to_string());

    // a + (b + c)
    tmp = a + (b + c);
    outputs.push(tmp.to_string());

    // (a + c) + b
    tmp = (a + c) + b;
    outputs.push(tmp.to_string());

    // Mixed addition

    // (a + b) + c
    tmp = a_affine.to_projective();
    tmp.add_assign_mixed(&b_affine);
    tmp.add_assign_mixed(&c_affine);
    outputs.push(tmp.to_string());

    // a + (b + c)
    tmp = b_affine.to_projective();
    tmp.add_assign_mixed(&c_affine);
    tmp.add_assign_mixed(&a_affine);
    outputs.push(tmp.to_string());

    // (a + c) + b
    tmp = a_affine.to_projective();
    tmp.add_assign_mixed(&c_affine);
    tmp.add_assign_mixed(&b_affine);
    outputs.push(tmp.to_string());

    Ok(Value::from(outputs))
}

fn mul<G: Projective>(mut a: G, mut b: G, s: Scalar) -> Result<Value, String> {
    let mut outputs = Vec::new();

    let a_affine = a.to_affine();
    let b_affine = b.to_affine();

    // s ( a + b )
    let mut tmp1 = a;
    tmp1 += b;
    tmp1 *= s;
    outputs.push(tmp1.to_string());

    // sa + sb
    a *= s;
    b *= s;

    let mut tmp2 = a;
    tmp2 += b;
    outputs.push(tmp2.to_string());

    // Affine multiplication
    let mut tmp3 = a_affine * s;
    tmp3.add_assign(b_affine * s);
    outputs.push(tmp3.to_string());

    assert_eq!(tmp1, tmp2);
    assert_eq!(tmp1, tmp3);
    Ok(Value::from(outputs))
}

fn double<G: Projective>(mut a: G, mut b: G) -> Result<Value, String> {
    let mut outputs = Vec::new();

    // 2(a + b)
    let mut tmp1 = a;
    tmp1.add_assign(b);
    tmp1.double_in_place();
    outputs.push(tmp1.to_string());

    // 2a + 2b
    a.double_in_place();
    b.double_in_place();

    let mut tmp2 = a;
    tmp2.add_assign(b);
    outputs.push(tmp2.to_string());

    let mut tmp3 = a;
    tmp3.add_assign_mixed(&b.to_affine());
    outputs.push(tmp3.to_string());

    assert_eq!(tmp1, tmp2);
    assert_eq!(tmp1, tmp3);
    Ok(Value::from(outputs))
}

fn neg<G: Projective>(r: G, s: Scalar) -> Result<Value, String> {
    let mut outputs = Vec::new();
    let sneg = -s;
    assert!((s + sneg).is_zero());

    let mut t1 = r;
    t1.mul_assign(s);
    outputs.push(t1.to_string());

    let mut t2 = r;
    t2.mul_assign(sneg);
    outputs.push(t2.to_string());

    let mut t3 = t1;
    t3.add_assign(t2);
    assert!(t3.is_zero());
    outputs.push(t3.to_string());

    let mut t4 = t1;
    t4.add_assign_mixed(&t2.to_affine());
    assert!(t4.is_zero());
    outputs.push(t4.to_string());

    t1 = -t1;
    outputs.push(t1.to_string());
    assert_eq!(t1, t2);
    Ok(Value::from(outputs))
}

fn transform<G: Projective>(g: G) -> Result<Value, String> {
    let g_affine = g.to_affine();
    let g_projective = g_affine.to_projective();
    assert_eq!(g, g_projective);

    Ok(Value::from(g_projective.to_string()))
}

fn batch_normalization<G: Projective>(mut batch: Vec<G>) -> Result<Value, String> {
    for i in &batch {
        assert!(!i.is_normalized());
    }

    let expected_v = batch
        .iter()
        .map(|v| v.to_affine().to_projective())
        .collect::<Vec<_>>();
    G::batch_normalization(&mut batch);

    for i in &batch {
        assert!(i.is_normalized());
    }

    assert_eq!(batch, expected_v);
    Ok(Value::from(
        expected_v
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>(),
    ))
}

pub struct G1ProjectiveNs;

impl G1ProjectiveNs {
    fn add(a: G1Projective, b: G1Projective, c: G1Projective) -> Result<Value, String> {
        add(a, b, c)
    }

    fn mul(a: G1Projective, b: G1Projective, s: Scalar) -> Result<Value, String> {
        mul(a, b, s)
    }

    fn double(a: G1Projective, b: G1Projective) -> Result<Value, String> {
        double(a, b)
    }

    fn neg(a: G1Projective, s: Scalar) -> Result<Value, String> {
        neg(a, s)
    }

    fn transform(g: G1Projective) -> Result<Value, String> {
        transform(g)
    }

    fn batch_normalization(batch: Vec<G1Projective>) -> Result<Value, String> {
        batch_normalization(batch)
    }
}

impl Namespace for G1ProjectiveNs {
    fn run_test(&self, test: Test) -> Result<Value, String> {
        match test.method.as_str() {
            "add" => {
                let (a, b, c): (G1Projective, G1Projective, G1Projective) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::add(a, b, c)
            }
            "mul" => {
                let (a, b, s): (G1Projective, G1Projective, Scalar) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::mul(a, b, s)
            }
            "double" => {
                let (a, b): (G1Projective, G1Projective) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::double(a, b)
            }
            "neg" => {
                let (a, s): (G1Projective, Scalar) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::neg(a, s)
            }
            "transform" => {
                let g: G1Projective =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::transform(g)
            }
            "batch_normalization" => {
                let batch: Vec<G1Projective> =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::batch_normalization(batch)
            }
            e => panic!("unknown method for G1ProjectiveNs: {e}"),
        }
    }
}

pub struct G2ProjectiveNs;

impl G2ProjectiveNs {
    fn add(a: G2Projective, b: G2Projective, c: G2Projective) -> Result<Value, String> {
        add(a, b, c)
    }

    fn mul(a: G2Projective, b: G2Projective, s: Scalar) -> Result<Value, String> {
        mul(a, b, s)
    }

    fn double(a: G2Projective, b: G2Projective) -> Result<Value, String> {
        double(a, b)
    }

    fn neg(a: G2Projective, s: Scalar) -> Result<Value, String> {
        neg(a, s)
    }

    fn transform(g: G2Projective) -> Result<Value, String> {
        transform(g)
    }

    fn batch_normalization(batch: Vec<G2Projective>) -> Result<Value, String> {
        batch_normalization(batch)
    }
}

impl Namespace for G2ProjectiveNs {
    fn run_test(&self, test: Test) -> Result<Value, String> {
        match test.method.as_str() {
            "add" => {
                let (a, b, c): (G2Projective, G2Projective, G2Projective) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::add(a, b, c)
            }
            "mul" => {
                let (a, b, s): (G2Projective, G2Projective, Scalar) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::mul(a, b, s)
            }
            "double" => {
                let (a, b): (G2Projective, G2Projective) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::double(a, b)
            }
            "neg" => {
                let (a, s): (G2Projective, Scalar) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::neg(a, s)
            }
            "transform" => {
                let g: G2Projective =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::transform(g)
            }
            "batch_normalization" => {
                let batch: Vec<G2Projective> =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::batch_normalization(batch)
            }
            e => panic!("unknown method for G2ProjectiveNs: {e}"),
        }
    }
}

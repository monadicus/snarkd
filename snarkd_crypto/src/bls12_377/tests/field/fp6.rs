use std::ops::MulAssign;

use test_runner::{Namespace, Test, TestResult};

use crate::{
    bls12_377::{Fp2, Fp6},
    frobenius,
};

use super::*;

pub struct Fp6Ns;

impl Fp6Ns {
    pub fn mul_by_1(c1: Fp2, mut a: Fp6) -> TestResult {
        let mut outputs = Vec::new();
        let mut b = a;

        a.mul_by_1(&c1);
        outputs.push(a);
        b.mul_assign(&Fp6::new(Fp2::ZERO, c1, Fp2::ZERO));
        outputs.push(b);
        assert_eq!(a, b);

        Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
    }

    pub fn mul_by_01(c0: Fp2, c1: Fp2, mut a: Fp6) -> TestResult {
        let mut outputs = Vec::new();
        let mut b = a;

        a.mul_by_01(&c0, &c1);
        outputs.push(a);
        b.mul_assign(&Fp6::new(c0, c1, Fp2::ZERO));
        outputs.push(b);
        assert_eq!(a, b);

        Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
    }
}

pub type Fp6Tuple = [Fp2Tuple; 3];
impl From<Fp6Tuple> for Fp6 {
    fn from(value: Fp6Tuple) -> Self {
        Self {
            c0: value[0].into(),
            c1: value[1].into(),
            c2: value[2].into(),
        }
    }
}

impl Namespace for Fp6Ns {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a: Fp6Tuple = serde_json::from_value(test.input).expect("failed to get input");
                neg::<Fp6>(a.into())
            }
            "add" => {
                let (a, b, c): (Fp6Tuple, Fp6Tuple, Fp6Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                add::<Fp6>(a.into(), b.into(), c.into())
            }
            "sub" => {
                let (a, b): (Fp6Tuple, Fp6Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                sub::<Fp6>(a.into(), b.into())
            }
            "mul" => {
                let (a, b, c): (Fp6Tuple, Fp6Tuple, Fp6Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                mul::<Fp6>(a.into(), b.into(), c.into())
            }
            "inversion" => {
                let a: Fp6Tuple = serde_json::from_value(test.input).expect("failed to get input");
                inversion::<Fp6>(a.into())
            }
            "double" => {
                let a: Fp6Tuple = serde_json::from_value(test.input).expect("failed to get input");
                double::<Fp6>(a.into())
            }
            "square" => {
                let a: Fp6Tuple = serde_json::from_value(test.input).expect("failed to get input");
                square::<Fp6>(a.into())
            }
            "expansion" => {
                let (a, b, c, d): (Fp6Tuple, Fp6Tuple, Fp6Tuple, Fp6Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                expansion::<Fp6>(a.into(), b.into(), c.into(), d.into())
            }
            "frobenius" => {
                let a: Fp6Tuple = serde_json::from_value(test.input).expect("failed to get input");
                frobenius!(Fp6::from(a), Fp6)
            }
            "sqrt" => {
                let a: Fp6Tuple = serde_json::from_value(test.input).expect("failed to get input");
                sqrt::<Fp6>(a.into())
            }
            "pow" => {
                let a: Fp6Tuple = serde_json::from_value(test.input).expect("failed to get input");
                pow::<Fp6>(a.into())
            }
            "sum_of_products" => {
                let (a, b): (Vec<Fp6Tuple>, Vec<Fp6Tuple>) =
                    serde_json::from_value(test.input).expect("failed to get input");
                let a = a.into_iter().map(|f| f.into()).collect();
                let b = b.into_iter().map(|f| f.into()).collect();
                sum_of_products::<Fp6>(a, b)
            }
            "math_properties" => {
                let (a, b): (Fp6Tuple, Fp6Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                math_properties::<Fp6>(a.into(), b.into())
            }
            "mul_by_1" => {
                let (c1, a): (Fp2Tuple, Fp6Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::mul_by_1(c1.into(), a.into())
            }
            "mul_by_01" => {
                let (c0, c1, a): (Fp2Tuple, Fp2Tuple, Fp6Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::mul_by_01(c0.into(), c1.into(), a.into())
            }
            e => panic!("unknown method for Fp6Ns: {e}"),
        }
    }
}

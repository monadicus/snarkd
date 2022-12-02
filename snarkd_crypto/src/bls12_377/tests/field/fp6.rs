use std::ops::MulAssign;

use test_runner::{Namespace, Test, TestResult};

use crate::{
    bls12_377::{Fp2, Fp6},
    frobenius,
};

use super::*;

pub struct Fp6Ns;

impl Fp6Ns {
    pub fn neg(a: Fp6) -> TestResult {
        neg(a)
    }

    pub fn add(a: Fp6, b: Fp6, c: Fp6) -> TestResult {
        add(a, b, c)
    }

    pub fn sub(a: Fp6, b: Fp6) -> TestResult {
        sub(a, b)
    }

    pub fn mul(a: Fp6, b: Fp6, c: Fp6) -> TestResult {
        mul(a, b, c)
    }

    pub fn inversion(a: Fp6) -> TestResult {
        inversion(a)
    }

    pub fn double(a: Fp6) -> TestResult {
        double(a)
    }

    pub fn square(a: Fp6) -> TestResult {
        square(a)
    }

    pub fn expansion(a: Fp6, b: Fp6, c: Fp6, d: Fp6) -> TestResult {
        expansion(a, b, c, d)
    }

    pub fn sqrt(a: Fp6) -> TestResult {
        sqrt(a)
    }

    pub fn frobenius(a: Fp6) -> TestResult {
        frobenius!(a, Fp6)
    }

    pub fn pow(a: Fp6) -> TestResult {
        pow(a)
    }

    pub fn sum_of_products(a: Vec<Fp6>, b: Vec<Fp6>) -> TestResult {
        sum_of_products(a, b)
    }

    pub fn math_properties(a: Fp6, b: Fp6) -> TestResult {
        math_properties(a, b)
    }

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
                Self::neg(a.into())
            }
            "add" => {
                let (a, b, c): (Fp6Tuple, Fp6Tuple, Fp6Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::add(a.into(), b.into(), c.into())
            }
            "sub" => {
                let (a, b): (Fp6Tuple, Fp6Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::sub(a.into(), b.into())
            }
            "mul" => {
                let (a, b, c): (Fp6Tuple, Fp6Tuple, Fp6Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::mul(a.into(), b.into(), c.into())
            }
            "inversion" => {
                let a: Fp6Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::inversion(a.into())
            }
            "double" => {
                let a: Fp6Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::double(a.into())
            }
            "square" => {
                let a: Fp6Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::square(a.into())
            }
            "expansion" => {
                let (a, b, c, d): (Fp6Tuple, Fp6Tuple, Fp6Tuple, Fp6Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::expansion(a.into(), b.into(), c.into(), d.into())
            }
            "frobenius" => {
                let a: Fp6Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::frobenius(a.into())
            }
            "sqrt" => {
                let a: Fp6Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::sqrt(a.into())
            }
            "pow" => {
                let a: Fp6Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::pow(a.into())
            }
            "sum_of_products" => {
                let (a, b): (Vec<Fp6Tuple>, Vec<Fp6Tuple>) =
                    serde_json::from_value(test.input).expect("failed to get input");
                let a = a.into_iter().map(|f| f.into()).collect();
                let b = b.into_iter().map(|f| f.into()).collect();
                Self::sum_of_products(a, b)
            }
            "math_properties" => {
                let (a, b): (Fp6Tuple, Fp6Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::math_properties(a.into(), b.into())
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

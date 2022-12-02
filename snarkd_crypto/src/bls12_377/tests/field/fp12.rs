use std::ops::MulAssign;

use test_runner::{Namespace, Test, TestResult};

use crate::{
    bls12_377::{Fp12, Fp2, Fp6},
    frobenius,
};

use super::*;

pub struct Fp12Ns;

impl Fp12Ns {
    pub fn neg(a: Fp12) -> TestResult {
        neg(a)
    }

    pub fn add(a: Fp12, b: Fp12, c: Fp12) -> TestResult {
        add(a, b, c)
    }

    pub fn sub(a: Fp12, b: Fp12) -> TestResult {
        sub(a, b)
    }

    pub fn mul(a: Fp12, b: Fp12, c: Fp12) -> TestResult {
        mul(a, b, c)
    }

    pub fn inversion(a: Fp12) -> TestResult {
        inversion(a)
    }

    pub fn double(a: Fp12) -> TestResult {
        double(a)
    }

    pub fn square(a: Fp12) -> TestResult {
        square(a)
    }

    pub fn expansion(a: Fp12, b: Fp12, c: Fp12, d: Fp12) -> TestResult {
        expansion(a, b, c, d)
    }

    pub fn sqrt(a: Fp12) -> TestResult {
        sqrt(a)
    }

    pub fn frobenius(a: Fp12) -> TestResult {
        frobenius!(a, Fp12)
    }

    pub fn pow(a: Fp12) -> TestResult {
        pow(a)
    }

    pub fn sum_of_products(a: Vec<Fp12>, b: Vec<Fp12>) -> TestResult {
        sum_of_products(a, b)
    }

    pub fn math_properties(a: Fp12, b: Fp12) -> TestResult {
        math_properties(a, b)
    }

    pub fn mul_by_014(c0: Fp2, c1: Fp2, c5: Fp2, mut a: Fp12) -> TestResult {
        let mut outputs = Vec::new();
        let mut b = a;

        a.mul_by_014(&c0, &c1, &c5);
        outputs.push(a);
        b.mul_assign(&Fp12::new(
            Fp6::new(c0, c1, Fp2::ZERO),
            Fp6::new(Fp2::ZERO, c5, Fp2::ZERO),
        ));
        outputs.push(b);
        assert_eq!(a, b);

        Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
    }

    pub fn mul_by_034(c0: Fp2, c3: Fp2, c4: Fp2, mut a: Fp12) -> TestResult {
        let mut outputs = Vec::new();
        let mut b = a;

        a.mul_by_034(&c0, &c3, &c4);
        outputs.push(a);
        b.mul_assign(&Fp12::new(
            Fp6::new(c0, Fp2::ZERO, Fp2::ZERO),
            Fp6::new(c3, c4, Fp2::ZERO),
        ));
        outputs.push(b);
        assert_eq!(a, b);

        Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
    }
}

type Fp12Tuple = [Fp6Tuple; 2];
impl From<Fp12Tuple> for Fp12 {
    fn from(value: Fp12Tuple) -> Self {
        Self {
            c0: value[0].into(),
            c1: value[1].into(),
        }
    }
}

impl Namespace for Fp12Ns {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a: Fp12Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::neg(a.into())
            }
            "add" => {
                let (a, b, c): (Fp12Tuple, Fp12Tuple, Fp12Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::add(a.into(), b.into(), c.into())
            }
            "sub" => {
                let (a, b): (Fp12Tuple, Fp12Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::sub(a.into(), b.into())
            }
            "mul" => {
                let (a, b, c): (Fp12Tuple, Fp12Tuple, Fp12Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::mul(a.into(), b.into(), c.into())
            }
            "inversion" => {
                let a: Fp12Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::inversion(a.into())
            }
            "double" => {
                let a: Fp12Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::double(a.into())
            }
            "square" => {
                let a: Fp12Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::square(a.into())
            }
            "expansion" => {
                let (a, b, c, d): (Fp12Tuple, Fp12Tuple, Fp12Tuple, Fp12Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::expansion(a.into(), b.into(), c.into(), d.into())
            }
            "frobenius" => {
                let a: Fp12Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::frobenius(a.into())
            }
            "sqrt" => {
                let a: Fp12Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::sqrt(a.into())
            }
            "pow" => {
                let a: Fp12Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::pow(a.into())
            }
            "sum_of_products" => {
                let (a, b): (Vec<Fp12Tuple>, Vec<Fp12Tuple>) =
                    serde_json::from_value(test.input).expect("failed to get input");
                let a = a.into_iter().map(|f| f.into()).collect();
                let b = b.into_iter().map(|f| f.into()).collect();
                Self::sum_of_products(a, b)
            }
            "math_properties" => {
                let (a, b): (Fp12Tuple, Fp12Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::math_properties(a.into(), b.into())
            }
            "mul_by_014" => {
                let (c0, c1, c5, a): (Fp2Tuple, Fp2Tuple, Fp2Tuple, Fp12Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::mul_by_014(c0.into(), c1.into(), c5.into(), a.into())
            }
            "mul_by_034" => {
                let (c0, c3, c4, a): (Fp2Tuple, Fp2Tuple, Fp2Tuple, Fp12Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::mul_by_034(c0.into(), c3.into(), c4.into(), a.into())
            }
            e => panic!("unknown method for Fp12Ns: {e}"),
        }
    }
}

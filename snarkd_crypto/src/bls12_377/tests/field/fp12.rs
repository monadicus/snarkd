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

impl Namespace for Fp12Ns {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::neg(a)
            }
            "add" => {
                let (a, b, c) = serde_json::from_value(test.input).expect("failed to get input");
                Self::add(a, b, c)
            }
            "sub" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                Self::sub(a, b)
            }
            "mul" => {
                let (a, b, c) = serde_json::from_value(test.input).expect("failed to get input");
                Self::mul(a, b, c)
            }
            "inversion" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::inversion(a)
            }
            "double" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::double(a)
            }
            "square" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::square(a)
            }
            "expansion" => {
                let (a, b, c, d) = serde_json::from_value(test.input).expect("failed to get input");
                Self::expansion(a, b, c, d)
            }
            "frobenius" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::frobenius(a)
            }
            "sqrt" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::sqrt(a)
            }
            "pow" => {
                let a = serde_json::from_value(test.input).expect("failed to get input");
                Self::pow(a)
            }
            "sum_of_products" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                Self::sum_of_products(a, b)
            }
            "math_properties" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                Self::math_properties(a, b)
            }
            "mul_by_014" => {
                let (c0, c1, c5, a) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::mul_by_014(c0, c1, c5, a)
            }
            "mul_by_034" => {
                let (c0, c3, c4, a) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::mul_by_034(c0, c3, c4, a)
            }
            e => panic!("unknown method for Fp12Ns: {e}"),
        }
    }
}

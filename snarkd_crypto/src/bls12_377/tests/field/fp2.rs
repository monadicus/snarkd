use test_runner::{Namespace, Test, TestResult};

use crate::{
    bls12_377::{Fp, Fp2},
    frobenius,
};

use super::*;

pub struct Fp2Ns;

impl Fp2Ns {
    pub fn neg(a: Fp2) -> TestResult {
        neg(a)
    }

    pub fn add(a: Fp2, b: Fp2, c: Fp2) -> TestResult {
        add(a, b, c)
    }

    pub fn sub(a: Fp2, b: Fp2) -> TestResult {
        sub(a, b)
    }

    pub fn mul(a: Fp2, b: Fp2, c: Fp2) -> TestResult {
        mul(a, b, c)
    }

    pub fn inversion(a: Fp2) -> TestResult {
        inversion(a)
    }

    pub fn double(a: Fp2) -> TestResult {
        double(a)
    }

    pub fn square(a: Fp2) -> TestResult {
        square(a)
    }

    pub fn expansion(a: Fp2, b: Fp2, c: Fp2, d: Fp2) -> TestResult {
        expansion(a, b, c, d)
    }

    pub fn sqrt(a: Fp2) -> TestResult {
        sqrt(a)
    }

    pub fn frobenius(a: Fp2) -> TestResult {
        frobenius!(a, Fp2)
    }

    pub fn pow(a: Fp2) -> TestResult {
        pow(a)
    }

    pub fn sum_of_products(a: Vec<Fp2>, b: Vec<Fp2>) -> TestResult {
        sum_of_products(a, b)
    }

    pub fn math_properties(a: Fp2, b: Fp2) -> TestResult {
        math_properties(a, b)
    }
}

pub type Fp2Tuple = [Fp; 2];
impl From<Fp2Tuple> for Fp2 {
    fn from(value: Fp2Tuple) -> Self {
        Self {
            c0: value[0],
            c1: value[1],
        }
    }
}

impl Namespace for Fp2Ns {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::neg(a.into())
            }
            "add" => {
                let (a, b, c): (Fp2Tuple, Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::add(a.into(), b.into(), c.into())
            }
            "sub" => {
                let (a, b): (Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::sub(a.into(), b.into())
            }
            "mul" => {
                let (a, b, c): (Fp2Tuple, Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::mul(a.into(), b.into(), c.into())
            }
            "inversion" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::inversion(a.into())
            }
            "double" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::double(a.into())
            }
            "square" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::square(a.into())
            }
            "expansion" => {
                let (a, b, c, d): (Fp2Tuple, Fp2Tuple, Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::expansion(a.into(), b.into(), c.into(), d.into())
            }
            "frobenius" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::frobenius(a.into())
            }
            "sqrt" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::sqrt(a.into())
            }
            "pow" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::pow(a.into())
            }
            "sum_of_products" => {
                let (a, b): (Vec<Fp2Tuple>, Vec<Fp2Tuple>) =
                    serde_json::from_value(test.input).expect("failed to get input");
                let a = a.into_iter().map(|f| f.into()).collect();
                let b = b.into_iter().map(|f| f.into()).collect();
                Self::sum_of_products(a, b)
            }
            "math_properties" => {
                let (a, b): (Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::math_properties(a.into(), b.into())
            }
            e => panic!("unknown method for Fp2Ns: {e}"),
        }
    }
}

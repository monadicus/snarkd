use test_runner::{Namespace, Test, TestResult};

use crate::{bls12_377::Fp2, frobenius};

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

pub type Fp2Tuple = [String; 2];
impl TryFrom<Fp2Tuple> for Fp2 {
    type Error = String;

    fn try_from(value: Fp2Tuple) -> Result<Self, Self::Error> {
        Ok(Self {
            c0: value[0].as_str().try_into()?,
            c1: value[1].as_str().try_into()?,
        })
    }
}

impl Namespace for Fp2Ns {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::neg(a.try_into()?)
            }
            "add" => {
                let (a, b, c): (Fp2Tuple, Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::add(a.try_into()?, b.try_into()?, c.try_into()?)
            }
            "sub" => {
                let (a, b): (Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::sub(a.try_into()?, b.try_into()?)
            }
            "mul" => {
                let (a, b, c): (Fp2Tuple, Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::mul(a.try_into()?, b.try_into()?, c.try_into()?)
            }
            "inversion" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::inversion(a.try_into()?)
            }
            "double" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::double(a.try_into()?)
            }
            "square" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::square(a.try_into()?)
            }
            "expansion" => {
                let (a, b, c, d): (Fp2Tuple, Fp2Tuple, Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::expansion(a.try_into()?, b.try_into()?, c.try_into()?, d.try_into()?)
            }
            "frobenius" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::frobenius(a.try_into()?)
            }
            "sqrt" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::sqrt(a.try_into()?)
            }
            "pow" => {
                let a: Fp2Tuple = serde_json::from_value(test.input).expect("failed to get input");
                Self::pow(a.try_into()?)
            }
            "sum_of_products" => {
                let (a, b): (Vec<Fp2Tuple>, Vec<Fp2Tuple>) =
                    serde_json::from_value(test.input).expect("failed to get input");
                let a = a
                    .into_iter()
                    .map(|f| f.try_into())
                    .collect::<Result<Vec<_>, _>>()?;
                let b = b
                    .into_iter()
                    .map(|f| f.try_into())
                    .collect::<Result<Vec<_>, _>>()?;
                Self::sum_of_products(a, b)
            }
            "math_properties" => {
                let (a, b): (Fp2Tuple, Fp2Tuple) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::math_properties(a.try_into()?, b.try_into()?)
            }
            e => panic!("unknown method for Fp2Ns: {e}"),
        }
    }
}

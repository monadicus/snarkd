use std::str::FromStr;

use test_runner::{Namespace, Test, TestResult};

use crate::bls12_377::Fp;

use super::*;

pub struct FpNs;

impl FpNs {
    pub fn neg(a: Fp) -> TestResult {
        neg(a)
    }

    pub fn add(a: Fp, b: Fp, c: Fp) -> TestResult {
        add(a, b, c)
    }

    pub fn sub(a: Fp, b: Fp) -> TestResult {
        sub(a, b)
    }

    pub fn mul(a: Fp, b: Fp, c: Fp) -> TestResult {
        mul(a, b, c)
    }

    pub fn inversion(a: Fp) -> TestResult {
        inversion(a)
    }

    pub fn double(a: Fp) -> TestResult {
        double(a)
    }

    pub fn square(a: Fp) -> TestResult {
        square(a)
    }

    pub fn expansion(a: Fp, b: Fp, c: Fp, d: Fp) -> TestResult {
        expansion(a, b, c, d)
    }

    pub fn sqrt(a: Fp) -> TestResult {
        sqrt(a)
    }

    pub fn pow(a: Fp) -> TestResult {
        pow(a)
    }

    pub fn sum_of_products(a: Vec<Fp>, b: Vec<Fp>) -> TestResult {
        sum_of_products(a, b)
    }

    pub fn math_properties(a: Fp, b: Fp) -> TestResult {
        math_properties(a, b)
    }
}

impl TryFrom<String> for Fp {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.strip_prefix("0x").unwrap_or(&value);
        let hex = format!("0x{:0>96}", value);
        ruint::Uint::from_str(&hex)
            .map_err(|e| format!("Failed to deserialize input: {e}"))
            .map(Self)
    }
}

impl TryFrom<&str> for Fp {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.strip_prefix("0x").unwrap_or(value);
        let hex = format!("0x{:0>96}", value);
        ruint::Uint::from_str(&hex)
            .map_err(|e| format!("Failed to deserialize input: {e}"))
            .map(Self)
    }
}

impl Namespace for FpNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a: String = serde_json::from_value(test.input).expect("failed to get input");
                Self::neg(a.try_into()?)
            }
            "add" => {
                let (a, b, c): (String, String, String) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::add(a.try_into()?, b.try_into()?, c.try_into()?)
            }
            "sub" => {
                let (a, b): (String, String) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::sub(a.try_into()?, b.try_into()?)
            }
            "mul" => {
                let (a, b, c): (String, String, String) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::mul(a.try_into()?, b.try_into()?, c.try_into()?)
            }
            "inversion" => {
                let a: String = serde_json::from_value(test.input).expect("failed to get input");
                Self::inversion(a.try_into()?)
            }
            "double" => {
                let a: String = serde_json::from_value(test.input).expect("failed to get input");
                Self::double(a.try_into()?)
            }
            "square" => {
                let a: String = serde_json::from_value(test.input).expect("failed to get input");
                Self::square(a.try_into()?)
            }
            "expansion" => {
                let (a, b, c, d): (String, String, String, String) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::expansion(a.try_into()?, b.try_into()?, c.try_into()?, d.try_into()?)
            }
            "sqrt" => {
                let a: String = serde_json::from_value(test.input).expect("failed to get input");
                Self::sqrt(a.try_into()?)
            }
            "pow" => {
                let a: String = serde_json::from_value(test.input).expect("failed to get input");
                Self::pow(a.try_into()?)
            }
            "sum_of_products" => {
                let (a, b): (Vec<String>, Vec<String>) =
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
                let (a, b): (String, String) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::math_properties(a.try_into()?, b.try_into()?)
            }
            e => panic!("unknown method for FpNs: {e}"),
        }
    }
}

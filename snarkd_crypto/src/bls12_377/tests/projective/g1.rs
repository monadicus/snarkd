use std::ops::Mul;

use bitvec::{prelude::Lsb0, view::BitView};
use test_runner::{Namespace, Test, TestResult};

use crate::bls12_377::{G1Projective, Scalar};

use super::*;

pub struct G1ProjectiveNs;

impl G1ProjectiveNs {
    fn add(a: G1Projective, b: G1Projective, c: G1Projective) -> TestResult {
        add(a, b, c)
    }

    fn mul(a: G1Projective, b: G1Projective, s: Scalar) -> TestResult {
        mul(a, b, s)
    }

    fn double(a: G1Projective, b: G1Projective) -> TestResult {
        double(a, b)
    }

    fn neg(a: G1Projective, s: Scalar) -> TestResult {
        neg(a, s)
    }

    fn transform(g: G1Projective) -> TestResult {
        transform(g)
    }

    fn batch_normalization(batch: Vec<G1Projective>) -> TestResult {
        batch_normalization(batch)
    }

    pub fn projective_glv(point: G1Projective, scalar: Scalar) -> TestResult {
        let mut outputs = Vec::new();

        let affine = point.to_affine();
        let point_mul = point.mul(scalar);
        outputs.push(point_mul.to_string());
        let affine_mul = affine.mul(scalar);
        outputs.push(affine_mul.to_string());
        assert_eq!(point_mul, affine_mul);

        let affine_mul_bits = affine.mul_bits(
            scalar
                .0
                .as_limbs()
                .iter()
                .flat_map(|limb| limb.view_bits::<Lsb0>())
                .map(|bit| *bit)
                .rev()
                .collect::<Vec<_>>(),
        );
        outputs.push(affine_mul_bits.to_string());

        assert_eq!(affine_mul, affine_mul_bits);

        Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
    }
}

impl Namespace for G1ProjectiveNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "add" => {
                let (a, b, c) = serde_json::from_value(test.input).expect("failed to get input");
                Self::add(a, b, c)
            }
            "mul" => {
                let (a, b, s) = serde_json::from_value(test.input).expect("failed to get input");
                Self::mul(a, b, s)
            }
            "double" => {
                let (a, b) = serde_json::from_value(test.input).expect("failed to get input");
                Self::double(a, b)
            }
            "neg" => {
                let (a, s) = serde_json::from_value(test.input).expect("failed to get input");
                Self::neg(a, s)
            }
            "transform" => {
                let g = serde_json::from_value(test.input).expect("failed to get input");
                Self::transform(g)
            }
            "batch_normalization" => {
                let batch = serde_json::from_value(test.input).expect("failed to get input");
                Self::batch_normalization(batch)
            }
            "projective_glv" => {
                let (point, scalar) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::projective_glv(point, scalar)
            }
            e => panic!("unknown method for G1ProjectiveNs: {e}"),
        }
    }
}

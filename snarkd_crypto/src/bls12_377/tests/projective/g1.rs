use std::ops::Mul;

use bitvec::{prelude::Lsb0, view::BitView};

use crate::bls12_377::{G1Projective, Scalar};

use super::*;

pub struct G1ProjectiveNs;

impl G1ProjectiveNs {
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
                let (a, b, c) = Self::get(test.input);
                add::<G1Projective>(a, b, c)
            }
            "mul" => {
                let (a, b, s) = Self::get(test.input);
                mul::<G1Projective>(a, b, s)
            }
            "double" => {
                let (a, b) = Self::get(test.input);
                double::<G1Projective>(a, b)
            }
            "neg" => {
                let (a, s) = Self::get(test.input);
                neg::<G1Projective>(a, s)
            }
            "transform" => {
                let g = Self::get(test.input);
                transform::<G1Projective>(g)
            }
            "batch_normalization" => {
                let batch = Self::get(test.input);
                batch_normalization::<G1Projective>(batch)
            }
            "projective_glv" => {
                let (point, scalar) = Self::get(test.input);
                Self::projective_glv(point, scalar)
            }
            e => panic!("unknown method for G1ProjectiveNs: {e}"),
        }
    }
}

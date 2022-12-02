use std::ops::Mul;

use bitvec::{prelude::Lsb0, view::BitView};
use test_runner::{Namespace, Test, TestResult};

use crate::bls12_377::{Fp, G1Projective, Scalar};

use super::*;

type G1Tuple = ProjectiveTuple<Fp>;

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
                let (a, b, c): (G1Tuple, G1Tuple, G1Tuple) = Self::get(test.input);
                add::<G1Projective>(a.into(), b.into(), c.into())
            }
            "mul" => {
                let (a, b, s): (G1Tuple, G1Tuple, _) = Self::get(test.input);
                mul::<G1Projective>(a.into(), b.into(), s)
            }
            "double" => {
                let (a, b): (G1Tuple, G1Tuple) = Self::get(test.input);
                double::<G1Projective>(a.into(), b.into())
            }
            "neg" => {
                let (a, s): (G1Tuple, _) = Self::get(test.input);
                neg::<G1Projective>(a.into(), s)
            }
            "transform" => {
                let g: G1Tuple = Self::get(test.input);
                transform::<G1Projective>(g.into())
            }
            "batch_normalization" => {
                let batch: Vec<G1Tuple> = Self::get(test.input);
                let batch = batch.into_iter().map(|v| v.into()).collect();
                batch_normalization::<G1Projective>(batch)
            }
            "projective_glv" => {
                let (point, scalar): (G1Tuple, _) = Self::get(test.input);
                Self::projective_glv(point.into(), scalar)
            }
            e => panic!("unknown method for G1ProjectiveNs: {e}"),
        }
    }
}

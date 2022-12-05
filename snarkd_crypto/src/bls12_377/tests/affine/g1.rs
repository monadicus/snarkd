use super::*;
use crate::bls12_377::{Field, Fp, G1Affine, Projective};

use bitvec::{prelude::Lsb0, view::BitView};

pub struct G1AffineNs;

impl G1AffineNs {
    pub fn subgroup_membership(p: G1Affine, x: Fp, greatest: bool) -> TestResult {
        let mut outputs = Vec::new();

        outputs.push(p.is_in_correct_subgroup_assuming_on_curve().to_string());
        assert!(p.is_in_correct_subgroup_assuming_on_curve());

        if let Some(p) = G1Affine::from_x_coordinate(x, greatest) {
            outputs.push(p.is_in_correct_subgroup_assuming_on_curve().to_string());
            let bits = p.mul_bits(
                Scalar::characteristic()
                    .iter()
                    .flat_map(|limb| limb.view_bits::<Lsb0>())
                    .map(|b| *b)
                    .rev()
                    .collect::<Vec<_>>(),
            );
            outputs.push(bits.to_string());
            assert_eq!(p.is_in_correct_subgroup_assuming_on_curve(), bits.is_zero(),);
        }

        Ok(outputs.into())
    }
}

impl Namespace for G1AffineNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a = Self::get(test.input);
                neg::<G1Affine>(a)
            }
            "mul" => {
                let (a, s) = Self::get(test.input);
                mul::<G1Affine>(a, s)
            }
            "subgroup_membership" => {
                let (p, x, greatest) = Self::get(test.input);
                Self::subgroup_membership(p, x, greatest)
            }
            e => panic!("unknown method for G1ProjectiveNs: {e}"),
        }
    }
}

#[test]
fn test_generator() {
    let generator = G1Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

use crate::bls12_377::{test::tests::field::Fp2Tuple, G2Projective};

use super::*;

pub type G2Tuple = ProjectiveTuple<Fp2Tuple>;

impl From<G2Tuple> for G2Projective {
    fn from(value: G2Tuple) -> Self {
        Self {
            x: value[0].into(),
            y: value[1].into(),
            z: value[2].into(),
        }
    }
}

pub struct G2ProjectiveNs;

impl Namespace for G2ProjectiveNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "add" => {
                let (a, b, c): (G2Tuple, G2Tuple, G2Tuple) = Self::get(test.input);
                add::<G2Projective>(a.into(), b.into(), c.into())
            }
            "mul" => {
                let (a, b, s): (G2Tuple, G2Tuple, _) = Self::get(test.input);
                mul::<G2Projective>(a.into(), b.into(), s)
            }
            "double" => {
                let (a, b): (G2Tuple, G2Tuple) = Self::get(test.input);
                double::<G2Projective>(a.into(), b.into())
            }
            "neg" => {
                let (a, s): (G2Tuple, _) = Self::get(test.input);
                neg::<G2Projective>(a.into(), s)
            }
            "transform" => {
                let g: G2Tuple = Self::get(test.input);
                transform::<G2Projective>(g.into())
            }
            "batch_normalization" => {
                let batch: Vec<G2Tuple> = Self::get(test.input);
                let batch = batch.into_iter().map(|v| v.into()).collect();
                batch_normalization::<G2Projective>(batch)
            }
            e => panic!("unknown method for G2ProjectiveNs: {e}"),
        }
    }
}

use crate::bls12_377::{test::tests::field::Fp2Tuple, G2Projective};

use super::*;

pub type G2ProjectiveTuple = ProjectiveTuple<Fp2Tuple>;

pub struct G2ProjectiveNs;

impl Namespace for G2ProjectiveNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "add" => {
                let (a, b, c): (G2ProjectiveTuple, G2ProjectiveTuple, G2ProjectiveTuple) =
                    Self::get(test.input);
                add::<G2Projective>(a.into(), b.into(), c.into())
            }
            "mul" => {
                let (a, b, s): (G2ProjectiveTuple, G2ProjectiveTuple, _) = Self::get(test.input);
                mul::<G2Projective>(a.into(), b.into(), s)
            }
            "double" => {
                let (a, b): (G2ProjectiveTuple, G2ProjectiveTuple) = Self::get(test.input);
                double::<G2Projective>(a.into(), b.into())
            }
            "neg" => {
                let (a, s): (G2ProjectiveTuple, _) = Self::get(test.input);
                neg::<G2Projective>(a.into(), s)
            }
            "transform" => {
                let g: G2ProjectiveTuple = Self::get(test.input);
                transform::<G2Projective>(g.into())
            }
            "batch_normalization" => {
                let batch: Vec<G2ProjectiveTuple> = Self::get(test.input);
                let batch = batch.into_iter().map(|v| v.into()).collect();
                batch_normalization::<G2Projective>(batch)
            }
            e => panic!("unknown method for G2ProjectiveNs: {e}"),
        }
    }
}

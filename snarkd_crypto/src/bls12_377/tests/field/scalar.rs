use crate::bls12_377::Scalar;

use super::*;

pub struct ScalarNs;

impl Namespace for ScalarNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "neg" => {
                let a = Self::get(test.input);
                neg::<Scalar>(a)
            }
            "add" => {
                let (a, b, c) = Self::get(test.input);
                add::<Scalar>(a, b, c)
            }
            "sub" => {
                let (a, b) = Self::get(test.input);
                sub::<Scalar>(a, b)
            }
            "mul" => {
                let (a, b, c) = Self::get(test.input);
                mul::<Scalar>(a, b, c)
            }
            "inversion" => {
                let a = Self::get(test.input);
                inversion::<Scalar>(a)
            }
            "double" => {
                let a = Self::get(test.input);
                double::<Scalar>(a)
            }
            "square" => {
                let a = Self::get(test.input);
                square::<Scalar>(a)
            }
            "expansion" => {
                let (a, b, c, d) = Self::get(test.input);
                expansion::<Scalar>(a, b, c, d)
            }
            "sqrt" => {
                let a = Self::get(test.input);
                sqrt::<Scalar>(a)
            }
            "pow" => {
                let a = Self::get(test.input);
                pow::<Scalar>(a)
            }
            "sum_of_products" => {
                let (a, b) = Self::get(test.input);
                sum_of_products::<Scalar>(a, b)
            }
            "math_properties" => {
                let (a, b) = Self::get(test.input);
                math_properties::<Scalar>(a, b)
            }
            e => panic!("unknown method for ScalarNs: {e}"),
        }
    }
}

#[test]
fn test_powers_of_g() {
    use crate::bls12_377::scalar::{GENERATOR, POWERS_OF_G, T, TWO_ADICITY};
    use ruint::uint;
    let two = Scalar(uint!(2_U256));

    // Compute the expected powers of G.
    let g = Scalar(GENERATOR).pow(T.as_limbs());
    let powers = (0..TWO_ADICITY - 1)
        .map(|i| g.pow(two.pow(&[i as u64]).0.as_limbs()))
        .collect::<Vec<_>>();

    // Ensure the correct number of powers of G are present.
    assert_eq!(POWERS_OF_G.len() as u64, (TWO_ADICITY - 1) as u64);
    assert_eq!(POWERS_OF_G.len(), powers.len());

    // Ensure the expected and candidate powers match.
    for (expected, candidate) in powers.iter().zip(POWERS_OF_G.iter()) {
        println!("{:?} =?= {:?}", expected, candidate);
        assert_eq!(*expected, Scalar(*candidate));
    }
}

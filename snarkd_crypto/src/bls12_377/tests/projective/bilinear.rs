use crate::bls12_377::{pairing, Fp12, G1Projective, G2Projective};

use super::*;

pub struct BilinearNs;

impl BilinearNs {
    fn bilinearity(a: G1Projective, b: G2Projective, s: Scalar) -> TestResult {
        let mut outputs = Vec::new();

        let sa = a * s;
        outputs.push(sa.to_string());
        let sb = b * s;
        outputs.push(sb.to_string());

        let ans1 = pairing(sa, b);
        outputs.push(ans1.to_string());
        let ans2 = pairing(a, sb);
        outputs.push(ans2.to_string());
        let ans3 = pairing(a, b).pow(s.0.as_limbs());
        outputs.push(ans3.to_string());

        assert_eq!(ans1, ans2);
        assert_eq!(ans2, ans3);

        assert_ne!(ans1, Fp12::ONE);
        assert_ne!(ans2, Fp12::ONE);
        assert_ne!(ans3, Fp12::ONE);

        assert_eq!(ans1.pow(&Scalar::characteristic()), Fp12::ONE);
        assert_eq!(ans2.pow(&Scalar::characteristic()), Fp12::ONE);
        assert_eq!(ans3.pow(&Scalar::characteristic()), Fp12::ONE);

        Ok(serde_json::to_value(outputs).expect("failed to serialize results"))
    }
}

impl Namespace for BilinearNs {
    fn run_test(&self, test: Test) -> TestResult {
        match test.method.as_str() {
            "bilinearity" => {
                let (a, b, s): (_, G2Tuple, _) =
                    serde_json::from_value(test.input).expect("failed to get input");
                Self::bilinearity(a, b.into(), s)
            }
            e => panic!("unknown method for BilinearNs: `{e}`"),
        }
    }
}

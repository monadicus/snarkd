mod affine;
mod field;
mod projective;

use test_runner::{run_tests, Namespace, Runner};

use self::{
    field::{Fp12Ns, Fp2Ns, Fp6Ns, FpNs, ScalarNs},
    projective::{G1ProjectiveNs, G2ProjectiveNs},
};

struct TestRunner;

impl Runner for TestRunner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>> {
        Some(match name {
            "FpNs" => Box::new(FpNs),
            "Fp2Ns" => Box::new(Fp2Ns),
            "Fp6Ns" => Box::new(Fp6Ns),
            "Fp12Ns" => Box::new(Fp12Ns),
            "G1ProjectiveNs" => Box::new(G1ProjectiveNs),
            "G2ProjectiveNs" => Box::new(G2ProjectiveNs),
            "ScalarNs" => Box::new(ScalarNs),
            _ => return None,
        })
    }
}

#[test]
pub fn curve_tests() {
    run_tests(&TestRunner, "crypto");
}

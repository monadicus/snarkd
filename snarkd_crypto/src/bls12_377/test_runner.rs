use super::G1ProjectiveNs;
use test_runner::{run_tests, Namespace, Runner};

struct TestRunner;

impl Runner for TestRunner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>> {
        Some(match name {
            "G1ProjectiveNs" => Box::new(G1ProjectiveNs),
            _ => return None,
        })
    }
}

#[test]
pub fn curve_tests() {
    run_tests(&TestRunner, "crypto");
}

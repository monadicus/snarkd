use test_runner::{Namespace, Runner};

struct TestRunner;

impl Runner for TestRunner {
    fn resolve_namespace(&self, _name: &str) -> Option<Box<dyn Namespace>> {
        None
    }
}

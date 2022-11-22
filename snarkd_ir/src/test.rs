use serde_json::Value;
use test_runner::*;

struct IRNamespace;

impl IRNamespace {
    fn u8_deserialize_serialize(input: Value) -> Result<String, String> {
        let value_to_convert: u8 = serde_json::from_value(input).expect("foo");
        Ok(value_to_convert.to_string())
    }
}

impl Namespace for IRNamespace {
    fn run_test(&self, test: Test) -> Result<String, String> {
        match test.method.as_str() {
            "u8_deserialize_serialize" => Self::u8_deserialize_serialize(test.input),
            e => panic!("unknown method for IRNamespace: {e}"),
        }
    }
}

struct TestRunner;

impl Runner for TestRunner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>> {
        Some(match name {
            "IR" => Box::new(IRNamespace),
            _ => return None,
        })
    }
}

#[test]
pub fn ir_tests() {
    run_tests(&TestRunner, "ir");
}
use serde_json::Value;
use test_runner::*;

struct IRNamespace;

impl IRNamespace {
    fn u8_deserialize_serialize(input: Value) -> Result<Value, String> {
        let value_to_convert: Result<u8, _> = serde_json::from_value(input);
        match value_to_convert {
            Ok(value) => Ok(Value::from(value.to_string())),
            Err(_) => Err("Failed".to_string()),
        }
    }
}

impl Namespace for IRNamespace {
    fn run_test(&self, test: Test) -> Result<Value, String> {
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

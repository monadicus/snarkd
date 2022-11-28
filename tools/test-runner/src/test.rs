use std::{collections::BTreeMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum TestExpectationMode {
    Pass,
    Fail,
    Skip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Case {
    pub expectation: TestExpectationMode,
    pub input: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    #[serde(skip)]
    pub path: PathBuf,
    pub namespace: String,
    pub method: String,
    pub tests: BTreeMap<String, Case>,
}

/*
{
    "namespace": "Foo",
    "method": "add",
    "cases": {
        "simple": {
            "input": [1, 2],
            "expectation": "Pass",
        },
        "overflow": {
            "input: [11231231231231, 2],
            "expectation": "Fail",
        }
    }
}
*/

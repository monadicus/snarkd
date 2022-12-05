use std::collections::BTreeMap;

use serde_json::Value;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Default)]
pub struct TestExpectation(pub BTreeMap<String, Value>);

/*
{
    "simple": "3",
    "overflow": "error overflow",
}
*/

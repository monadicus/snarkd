use std::collections::BTreeMap;

use serde_json::Value;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct TestExpectation(pub BTreeMap<String, Value>);

/*
{
    "simple": "3",
    "overflow": "error overflow",
}
*/

use std::collections::BTreeMap;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct TestExpectation(pub BTreeMap<String, String>);

/*
{
    "simple": "3",
    "overflow": "error overflow",
}
*/

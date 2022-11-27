use std::{fs::read_to_string, path::Path};

use walkdir::WalkDir;

use crate::test::TestConfig;

pub fn find_tests(path: &Path) -> impl Iterator<Item = TestConfig> {
    WalkDir::new(path)
        .into_iter()
        .flatten()
        .filter_map(move |f| {
            let path = f.path();
            if path.extension().filter(|p| *p == "json").is_some() {
                let content = read_to_string(path)
                    .unwrap_or_else(|_| panic!("failed to read test file {}", path.display()));
                let mut test_config: TestConfig =
                    serde_json::from_str(&content).unwrap_or_else(|e| {
                        panic!("could not deserialize test file {}: `{e}`", path.display())
                    });
                test_config.path = path.to_path_buf();
                Some(test_config)
            } else {
                None
            }
        })
}

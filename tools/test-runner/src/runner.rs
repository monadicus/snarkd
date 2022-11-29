use colored::Colorize;
use serde_json::Value;
use std::{
    any::Any,
    backtrace::Backtrace,
    collections::BTreeMap,
    panic::{self, RefUnwindSafe, UnwindSafe},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
};

use crate::{error::*, fetch::find_tests, output::TestExpectation, test::*};

#[derive(Debug)]
pub struct Test {
    pub method: String,
    pub input: Value,
}

pub type TestResult = Result<Value, String>;

pub trait Namespace: UnwindSafe + RefUnwindSafe {
    fn run_test(&self, test: Test) -> TestResult;
}

pub trait Runner {
    fn resolve_namespace(&self, name: &str) -> Option<Box<dyn Namespace>>;
}

fn is_env_var_set(var: &str) -> bool {
    std::env::var(var)
        .unwrap_or_else(|_| "".to_string())
        .trim()
        .is_empty()
}

fn set_hook() -> Arc<Mutex<Option<String>>> {
    let panic_buf = Arc::new(Mutex::new(None));
    let thread_id = thread::current().id();
    panic::set_hook({
        let panic_buf = panic_buf.clone();
        Box::new(move |e| {
            if thread::current().id() == thread_id {
                if !is_env_var_set("RUST_BACKTRACE") {
                    *panic_buf.lock().unwrap() = Some(format!("{e}\n{}", Backtrace::capture()));
                } else {
                    *panic_buf.lock().unwrap() = Some(e.to_string());
                }
            } else {
                println!("{e}")
            }
        })
    });
    panic_buf
}

fn take_hook(
    output: Result<Result<Value, String>, Box<dyn Any + Send>>,
    panic_buf: Arc<Mutex<Option<String>>>,
) -> Result<Result<Value, String>, String> {
    let _ = panic::take_hook();
    output.map_err(|_| {
        panic_buf
            .lock()
            .unwrap()
            .take()
            .expect("failed to get panic message")
    })
}

pub struct TestCases {
    path_prefix: PathBuf,
    fail_categories: Vec<TestFailure>,
}

impl TestCases {
    fn new(expectation_category: &str) -> Self {
        let mut path_prefix = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_prefix.push("../../tests/");
        path_prefix.push(expectation_category);
        if let Ok(p) = std::env::var("TEST_FILTER") {
            path_prefix.push(p);
        }

        let mut expectation_dir = path_prefix.clone();
        expectation_dir.push("expectations");

        Self {
            path_prefix,
            fail_categories: Vec::new(),
        }
    }

    fn load_expectations(&self, path: &Path) -> (PathBuf, Option<TestExpectation>) {
        let test_dir = [env!("CARGO_MANIFEST_DIR"), "../../tests/"]
            .iter()
            .collect::<PathBuf>();
        let relative_path = path.strip_prefix(&test_dir).expect("path error for test");
        let expectation_path = test_dir
            .join("expectations")
            .join(relative_path.parent().expect("no parent dir for test"))
            .join(relative_path.file_name().expect("no file name for test"))
            .with_extension("json");

        if expectation_path.exists() {
            if !is_env_var_set("CLEAR_TEST_EXPECTATIONS") {
                (expectation_path, None)
            } else {
                let raw = std::fs::read_to_string(&expectation_path)
                    .expect("failed to read expectations file");
                (
                    expectation_path,
                    Some(serde_json::from_str(&raw).expect("invalid yaml in expectations file")),
                )
            }
        } else {
            (expectation_path, None)
        }
    }
}

#[cfg(target_os = "windows")]
const TESTS: &str = "tests\\";
#[cfg(not(target_os = "windows"))]
const TESTS: &str = "tests/";

pub fn run_tests<T: Runner>(runner: &T, expectation_category: &str) {
    let mut cases = TestCases::new(expectation_category);
    let configs = find_tests(&cases.path_prefix.clone()).collect::<Vec<_>>();

    let mut pass_tests = 0;
    let mut fail_tests = 0;
    let mut skipped_tests = 0;

    let mut outputs = vec![];
    for config in configs {
        let namespace = match runner.resolve_namespace(&config.namespace) {
            Some(ns) => ns,
            None => return,
        };

        let (expectation_path, expectations) = cases.load_expectations(&config.path);

        let mut errors = vec![];
        if let Some(expectations) = expectations.as_ref() {
            let found = config.tests.len()
                - config
                    .tests
                    .iter()
                    .filter(|(_, case)| case.expectation == TestExpectationMode::Skip)
                    .count();
            let expected = expectations.0.len();
            // TODO should we have it just add a new expectation to the json instead of panicking
            assert_eq!(found, expected, "invalid number of test expectations");
        }

        let mut new_outputs = BTreeMap::new();
        let expected_output = expectations.clone().unwrap_or_default().0;

        for (i, (test_name, test_case)) in config.tests.into_iter().enumerate() {
            if matches!(test_case.expectation, TestExpectationMode::Skip) {
                println!(
                    "{}",
                    format!(
                        "skipping test '{test_name}' @ '{TESTS}{}'",
                        config
                            .path
                            .display()
                            .to_string()
                            .rsplit('/')
                            .next()
                            .unwrap(),
                    )
                    .yellow()
                );
                skipped_tests += 1;
                continue;
            }

            let expected_output = expected_output.get(&test_name);
            if expectations.is_some() && expected_output.is_none() {
                // TODO should we have it just add a new expectation to the json instead of panicking
                panic!("no test expectation for `{test_name}`");
            }
            println!(
                "{}",
                format!(
                    "running test '{test_name}' @ '{TESTS}{}'",
                    config
                        .path
                        .display()
                        .to_string()
                        .rsplit('/')
                        .next()
                        .unwrap(),
                )
                .cyan()
            );

            let panic_buf = set_hook();
            let snarkd_output = panic::catch_unwind(|| {
                namespace.run_test(Test {
                    method: config.method.clone(),
                    input: test_case.input.clone(),
                })
            });
            let output = take_hook(snarkd_output, panic_buf);

            if let Some(error) = emit_errors(
                &test_name,
                &output,
                test_case.expectation,
                expected_output,
                i,
            ) {
                fail_tests += 1;
                errors.push(error);
            } else {
                pass_tests += 1;
                new_outputs.insert(test_name, output.unwrap().unwrap_or_else(|e| e.into()));
            }
        }

        if errors.is_empty() {
            if expectations.is_none() {
                outputs.push((expectation_path, TestExpectation(new_outputs)));
            }
        } else {
            cases.fail_categories.push(TestFailure {
                path: config.path.display().to_string(),
                errors,
            })
        }
    }

    println!(
        "{}",
        format!("skipped {skipped_tests}/{skipped_tests} tests").yellow()
    );

    if !cases.fail_categories.is_empty() {
        for (i, fail) in cases.fail_categories.iter().enumerate() {
            println!(
                "{}",
                format!(
                    "\n\n-----------------TEST #{} FAILED (and shouldn't have)-----------------",
                    i + 1
                )
                .red()
            );
            println!("File: {}", fail.path);
            for error in &fail.errors {
                println!("{error}");
            }
        }
        panic!("failed {pass_tests}/{} tests", fail_tests + pass_tests,);
    } else {
        for (path, new_expectation) in outputs {
            std::fs::create_dir_all(path.parent().unwrap())
                .expect("failed to make test expectation parent directory");
            std::fs::write(
                &path,
                serde_json::to_string_pretty(&new_expectation)
                    .expect("failed to serialize expectation json"),
            )
            .expect("failed to write expectation file");
        }
        println!(
            "{}",
            format!("passed {pass_tests}/{} tests", fail_tests + pass_tests,).green()
        );
    }
}

/// returns all test configs for a given test folder.
pub fn get_benches(test_folder: &str) -> Vec<TestConfig> {
    let cases = TestCases::new(test_folder);
    find_tests(&cases.path_prefix)
        .map(|mut config| {
            config
                .tests
                .retain(|_, case| case.expectation == TestExpectationMode::Pass);
            config
        })
        .collect::<Vec<_>>()
}

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

pub trait Namespace: UnwindSafe + RefUnwindSafe {
    fn run_test(&self, test: Test) -> Result<String, String>;
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
    output: Result<Result<String, String>, Box<dyn Any + Send>>,
    panic_buf: Arc<Mutex<Option<String>>>,
) -> Result<Result<String, String>, String> {
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
    tests: Vec<TestConfig>,
    path_prefix: PathBuf,
    fail_categories: Vec<TestFailure>,
}

impl TestCases {
    fn new(
        expectation_category: &str,
        additional_check: impl Fn(&TestConfig) -> bool,
    ) -> (Self, Vec<TestConfig>) {
        let mut path_prefix = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path_prefix.push("../../tests/");
        path_prefix.push(expectation_category);
        if let Ok(p) = std::env::var("TEST_FILTER") {
            path_prefix.push(p);
        }

        let mut expectation_dir = path_prefix.clone();
        expectation_dir.push("expectations");

        let mut new = Self {
            tests: Vec::new(),
            path_prefix,
            fail_categories: Vec::new(),
        };
        let tests = new.load_tests(additional_check);
        (new, tests)
    }

    fn load_tests(&mut self, additional_check: impl Fn(&TestConfig) -> bool) -> Vec<TestConfig> {
        let mut configs = Vec::new();

        self.tests = find_tests(&self.path_prefix.clone())
            .filter(|cfg| {
                let res = additional_check(cfg);
                configs.push(cfg.clone());
                res
            })
            .collect();

        configs
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
            .with_extension("out");

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

pub fn run_tests<T: Runner>(runner: &T, expectation_category: &str) {
    let (mut cases, configs) = TestCases::new(expectation_category, |_| true);

    let mut pass_categories = 0;
    let mut pass_tests = 0;
    let mut fail_tests = 0;

    let mut outputs = vec![];
    for config in cases.tests.clone() {
        let namespace = match runner.resolve_namespace(&config.namespace) {
            Some(ns) => ns,
            None => return,
        };

        let (expectation_path, expectations) = cases.load_expectations(&config.path);

        let mut errors = vec![];
        if let Some(expectations) = expectations.as_ref() {
            if config.tests.len() != expectations.0.len() {
                errors.push(TestError::MismatchedTestExpectationLength);
            }
        }

        let mut new_outputs = BTreeMap::new();
        let mut expected_output = expectations.as_ref().map(|x| x.0.iter());

        for (i, (test_name, test_case)) in config.tests.into_iter().enumerate() {
            let expected_output = expected_output.as_mut().and_then(|x| x.next());
            println!(
                "running test {test_name} @ '{}'",
                config.path.to_str().unwrap(),
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
                new_outputs.insert(test_name, output.unwrap().unwrap_or_else(|e| e));
            }
        }

        if errors.is_empty() {
            if expectations.is_none() {
                outputs.push((expectation_path, TestExpectation(new_outputs)));
            }
            pass_categories += 1;
        } else {
            cases.fail_categories.push(TestFailure {
                path: config.path.display().to_string(),
                errors,
            })
        }
    }

    if !cases.fail_categories.is_empty() {
        for (i, fail) in cases.fail_categories.iter().enumerate() {
            println!(
                "\n\n-----------------TEST #{} FAILED (and shouldn't have)-----------------",
                i + 1
            );
            println!("File: {}", fail.path);
            for error in &fail.errors {
                println!("{error}");
            }
        }
        panic!(
            "failed {}/{} tests in {}/{} categories",
            pass_tests,
            fail_tests + pass_tests,
            cases.fail_categories.len(),
            cases.fail_categories.len() + pass_categories
        );
    } else {
        for (path, new_expectation) in outputs {
            std::fs::create_dir_all(path.parent().unwrap())
                .expect("failed to make test expectation parent directory");
            std::fs::write(
                &path,
                serde_json::to_string_pretty(&new_expectation)
                    .expect("failed to serialize expectation yaml"),
            )
            .expect("failed to write expectation file");
        }
        println!(
            "passed {}/{} tests in {}/{} categories",
            pass_tests,
            fail_tests + pass_tests,
            pass_categories,
            pass_categories
        );
    }
}

// /// returns (name, content) for all benchmark samples
// pub fn get_benches() -> Vec<(String, String)> {
//     let (mut cases, configs) = TestCases::new("compiler", |config| {
//         (&config.namespace == "Bench" && config.expectation == TestExpectationMode::Pass)
//             || (&config.namespace == "Compile"
//                 && !matches!(
//                     config.expectation,
//                     TestExpectationMode::Fail | TestExpectationMode::Skip
//                 ))
//     });

//     // cases.process_tests(configs, |_, (_, content, test_name, _)| {
//     //     (test_name.to_string(), content.to_string())
//     // })
//     todo!()
// }

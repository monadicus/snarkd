use colored::Colorize;

use std::fmt;

use crate::test::TestExpectationMode;

pub struct TestFailure {
    pub path: String,
    pub errors: Vec<TestError>,
}

#[derive(Debug)]
pub enum TestError {
    Panicked {
        test: String,
        index: usize,
        error: String,
    },
    UnexpectedOutput {
        test: String,
        index: usize,
        expected: String,
        output: String,
    },
    PassedAndShouldntHave {
        test: String,
        index: usize,
    },
    FailedAndShouldntHave {
        test: String,
        index: usize,
        error: String,
    },
    UnexpectedError {
        test: String,
        index: usize,
        expected: String,
        output: String,
    },
    MismatchedTestExpectationLength,
    MissingTestConfig,
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestError::Panicked { test, index, error } => {
                write!(
                    f,
                    "{}",
                    format!(
                        "test #{}: case `{}` encountered a rust panic:\n{}",
                        index + 1,
                        test.purple(),
                        error
                    )
                    .red()
                )
            }
            TestError::UnexpectedOutput {
                test,
                index,
                expected,
                output,
            } => {
                write!(
                    f,
                    "{}",
                    format!(
                        "test #{}: case '{}' expected '{}' got '{}'",
                        index + 1,
                        test.purple(),
                        serde_json::to_string(&expected)
                            .expect("serialization failed")
                            .cyan(),
                        serde_json::to_string(&output)
                            .expect("serialization failed")
                            .blue()
                    )
                    .red()
                )
            }
            TestError::PassedAndShouldntHave { test, index } => {
                write!(
                    f,
                    "{}",
                    format!(
                        "test #{}: case '{}' passed and shouldn't have",
                        index + 1,
                        test.purple()
                    )
                    .red()
                )
            }
            TestError::FailedAndShouldntHave { test, index, error } => {
                write!(
                    f,
                    "{}",
                    format!(
                        "test #{}: case '{}' failed and shouldn't have:\n{}",
                        index + 1,
                        test.purple(),
                        error
                    )
                    .red()
                )
            }
            TestError::UnexpectedError {
                test,
                expected,
                output,
                index,
            } => {
                write!(
                    f,
                    "{}",
                    format!(
                        "test #{}: case `{}` expected error `{}` got `{}`",
                        index + 1,
                        test.purple(),
                        expected.cyan(),
                        output.blue(),
                    )
                    .red(),
                )
            }
            TestError::MismatchedTestExpectationLength => {
                write!(f, "{}", "invalid number of test expectations".red())
            }
            TestError::MissingTestConfig => write!(f, "{}", "missing test config".red()),
        }
    }
}

pub fn emit_errors(
    test: &str,
    output: &Result<Result<String, String>, String>,
    mode: TestExpectationMode,
    expected_output: Option<(&String, &String)>,
    test_index: usize,
) -> Option<TestError> {
    match (output, mode) {
        (Err(e), _) => Some(TestError::Panicked {
            test: test.to_string(),
            index: test_index,
            error: e.to_string(),
        }),
        (Ok(Ok(output)), TestExpectationMode::Pass) => {
            // passed and should have
            if let Some(expected_output) = expected_output.as_ref() {
                if output != expected_output.1 {
                    // invalid output
                    return Some(TestError::UnexpectedOutput {
                        test: test.to_string(),
                        index: test_index,
                        expected: expected_output.1.clone(),
                        output: output.clone(),
                    });
                }
            }
            None
        }
        (Ok(Ok(_tokens)), TestExpectationMode::Fail) => Some(TestError::PassedAndShouldntHave {
            test: test.to_string(),
            index: test_index,
        }),
        (Ok(Err(err)), TestExpectationMode::Pass) => Some(TestError::FailedAndShouldntHave {
            test: test.to_string(),
            error: err.to_string(),
            index: test_index,
        }),
        (Ok(Err(err)), TestExpectationMode::Fail) => {
            if let Some((_, expected_output)) = expected_output {
                if err != expected_output {
                    // invalid output
                    return Some(TestError::UnexpectedError {
                        test: test.to_string(),
                        expected: expected_output.to_string(),
                        output: err.to_string(),
                        index: test_index,
                    });
                }
            }
            None
        }
        (Ok(_), TestExpectationMode::Skip) => None,
    }
}

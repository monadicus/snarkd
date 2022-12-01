use clap::Parser;
use rand::{thread_rng, Rng};
use serde_json::Value;
use snarkd_crypto::bls12_377::{Field, Fp, Fp2};
use std::{collections::BTreeMap, fs::write, path::PathBuf};
use test_runner::{Case, TestConfig, TestExpectationMode};

#[derive(Parser, Debug)]
struct Args {
    /// test namespace
    namespace: String,
    /// method to test in namespace
    method: String,
    /// number of cases to generate
    n_tests: usize,
    /// case input generation mode
    #[arg(value_enum)]
    input: Input,
    /// path to the output file. prints to terminal if unset
    #[arg(short)]
    output: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Input {
    OneFp,
    TwoFp,
    ThreeFp,
    FourFp,
    TwoFpLists,
    OneFp2,
    TwoFp2,
    ThreeFp2,
    FourFp2,
    TwoFp2Lists,
}

impl Input {
    fn gen(self, n: usize) -> Tests {
        match self {
            Input::OneFp => Self::gen_tests(n, Self::gen_fp),
            Input::TwoFp => Self::gen_tests(n, Self::gen_fps::<2>),
            Input::ThreeFp => Self::gen_tests(n, Self::gen_fps::<3>),
            Input::FourFp => Self::gen_tests(n, Self::gen_fps::<4>),
            Input::TwoFpLists => Self::gen_tests(n, Self::gen_fp_lists::<2>),
            Input::OneFp2 => Self::gen_tests(n, Self::gen_fp2),
            Input::TwoFp2 => Self::gen_tests(n, Self::gen_fp2s::<2>),
            Input::ThreeFp2 => Self::gen_tests(n, Self::gen_fp2s::<3>),
            Input::FourFp2 => Self::gen_tests(n, Self::gen_fp2s::<4>),
            Input::TwoFp2Lists => Self::gen_tests(n, Self::gen_fp2_lists::<2>),
        }
    }

    fn gen_tests(n_tests: usize, f: fn() -> Value) -> Tests {
        let pad_len = n_tests.to_string().len() - 1;
        Tests(
            (0..n_tests)
                .map(|i| {
                    (
                        format!("random_{i:0pad_len$}"),
                        Case {
                            expectation: TestExpectationMode::Pass,
                            input: f(),
                        },
                    )
                })
                .collect(),
        )
    }

    fn gen_lists<const N_ARGS: usize>(f: fn() -> Value) -> Value {
        Value::Array(
            (0..N_ARGS)
                .map(|_| (0..thread_rng().gen_range(0..10)).map(|_| f()).collect())
                .collect(),
        )
    }

    fn gen_fp() -> Value {
        Fp::rand().to_string().into()
    }

    fn gen_fps<const N_ARGS: usize>() -> Value {
        Value::Array((0..N_ARGS).map(|_| Self::gen_fp()).collect())
    }

    fn gen_fp_lists<const N_ARGS: usize>() -> Value {
        Self::gen_lists::<N_ARGS>(Self::gen_fp)
    }

    fn gen_fp2() -> Value {
        let v = Fp2::rand();
        Value::Array(vec![v.c0.to_string().into(), v.c1.to_string().into()])
    }

    fn gen_fp2s<const N_ARGS: usize>() -> Value {
        Value::Array((0..N_ARGS).map(|_| Self::gen_fp2()).collect())
    }

    fn gen_fp2_lists<const N_ARGS: usize>() -> Value {
        Self::gen_lists::<N_ARGS>(Self::gen_fp2)
    }
}

struct Tests(BTreeMap<String, Case>);

impl Tests {
    fn into_config(self, namespace: String, method: String) -> String {
        serde_json::to_string_pretty(&TestConfig {
            path: Default::default(),
            namespace,
            method,
            tests: self.0,
        })
        .unwrap()
    }
}

fn main() {
    let args = Args::parse();
    let tests = args.input.gen(args.n_tests);
    let json = tests.into_config(args.namespace, args.method.clone());
    if let Some(mut out) = args.output {
        if out.is_dir() {
            out.push(format!("generated_{}.json", args.method))
        }
        write(&out, json).unwrap();
        println!("wrote output to {}", out.display());
    } else {
        println!("{json}");
    };
}

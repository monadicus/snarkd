use clap::Parser;
use rand::{thread_rng, Rng};
use serde_json::Value;
use snarkd_crypto::bls12_377::{Field, Fp, Fp12, Fp2, Fp6};
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

/// The different generators for inputs
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
    OneFp6,
    TwoFp6,
    ThreeFp6,
    FourFp6,
    TwoFp6Lists,
    OneFp12,
    TwoFp12,
    ThreeFp12,
    FourFp12,
    TwoFp12Lists,
}

impl Input {
    /// lookup table for generator methods
    const FN_TABLE: &'static [fn() -> Value] = &[
        Self::gen_fp,
        Self::gen_fps::<2>,
        Self::gen_fps::<3>,
        Self::gen_fps::<4>,
        Self::gen_fp_lists::<2>,
        Self::gen_fp2,
        Self::gen_fp2s::<2>,
        Self::gen_fp2s::<3>,
        Self::gen_fp2s::<4>,
        Self::gen_fp2_lists::<2>,
        Self::gen_fp6,
        Self::gen_fp6s::<2>,
        Self::gen_fp6s::<3>,
        Self::gen_fp6s::<4>,
        Self::gen_fp6_lists::<2>,
        Self::gen_fp12,
        Self::gen_fp12s::<2>,
        Self::gen_fp12s::<3>,
        Self::gen_fp12s::<4>,
        Self::gen_fp12_lists::<2>,
    ];

    /// used to generate `n` input cases for tests. this is the method you want
    fn gen(self, n: usize) -> Tests {
        Self::gen_inner(n, Self::FN_TABLE[self as usize])
    }

    /// inner function to generate `n` input cases using the given generator `f`
    fn gen_inner(n_tests: usize, f: fn() -> Value) -> Tests {
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

    /// helper for generating multiple of the input type
    fn gen_multi<const N_ARGS: usize>(f: fn() -> Value) -> Value {
        (0..N_ARGS).map(|_| f()).collect::<Vec<_>>().into()
    }

    /// helper for generating multiple arrays of the input type
    fn gen_lists<const N_ARGS: usize>(f: fn() -> Value) -> Value {
        (0..N_ARGS)
            .map(|_| {
                (0..thread_rng().gen_range(0..10))
                    .map(|_| f())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .into()
    }

    /// generates a single Fp value
    fn gen_fp() -> Value {
        Fp::rand().to_string().into()
    }

    /// generates a single Fp2 value
    fn gen_fp2() -> Value {
        let v = Fp2::rand();
        vec![v.c0.to_string(), v.c1.to_string()].into()
    }

    /// generates a single Fp6 value
    fn gen_fp6() -> Value {
        let v = Fp6::rand();
        vec![
            vec![v.c0.c0.to_string(), v.c0.c1.to_string()],
            vec![v.c1.c0.to_string(), v.c1.c1.to_string()],
            vec![v.c2.c0.to_string(), v.c2.c1.to_string()],
        ]
        .into()
    }

    /// generates a single Fp12 value
    fn gen_fp12() -> Value {
        let v = Fp12::rand();
        vec![
            vec![
                vec![v.c0.c0.c0.to_string(), v.c0.c0.c1.to_string()],
                vec![v.c0.c1.c0.to_string(), v.c0.c1.c1.to_string()],
                vec![v.c0.c2.c0.to_string(), v.c0.c2.c1.to_string()],
            ],
            vec![
                vec![v.c1.c0.c0.to_string(), v.c1.c0.c1.to_string()],
                vec![v.c1.c1.c0.to_string(), v.c1.c1.c1.to_string()],
                vec![v.c1.c2.c0.to_string(), v.c1.c2.c1.to_string()],
            ],
        ]
        .into()
    }

    /// generates multiple Fp values
    fn gen_fps<const N_ARGS: usize>() -> Value {
        Self::gen_multi::<N_ARGS>(Self::gen_fp)
    }

    /// generates multiple Fp2 values
    fn gen_fp2s<const N_ARGS: usize>() -> Value {
        Self::gen_multi::<N_ARGS>(Self::gen_fp2)
    }

    /// generates multiple Fp6 values
    fn gen_fp6s<const N_ARGS: usize>() -> Value {
        Self::gen_multi::<N_ARGS>(Self::gen_fp6)
    }

    /// generates multiple Fp12 values
    fn gen_fp12s<const N_ARGS: usize>() -> Value {
        Self::gen_multi::<N_ARGS>(Self::gen_fp)
    }

    /// generates multiple vectors of Fp values
    fn gen_fp_lists<const N_ARGS: usize>() -> Value {
        Self::gen_lists::<N_ARGS>(Self::gen_fp)
    }

    /// generates multiple vectors of Fp2 values
    fn gen_fp2_lists<const N_ARGS: usize>() -> Value {
        Self::gen_lists::<N_ARGS>(Self::gen_fp2)
    }

    /// generates multiple vectors of Fp6 values
    fn gen_fp6_lists<const N_ARGS: usize>() -> Value {
        Self::gen_lists::<N_ARGS>(Self::gen_fp6)
    }

    /// generates multiple vectors of Fp12 values
    fn gen_fp12_lists<const N_ARGS: usize>() -> Value {
        Self::gen_lists::<N_ARGS>(Self::gen_fp12)
    }
}

/// input cases to test on a method
struct Tests(BTreeMap<String, Case>);

impl Tests {
    /// converts the given tests into a json format for the test-runner
    fn into_json(self, namespace: String, method: String) -> String {
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
    let json = tests.into_json(args.namespace, args.method.clone());
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

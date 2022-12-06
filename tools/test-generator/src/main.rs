use clap::Parser;
use rand::{thread_rng, Rng};
use serde_json::Value;
use snarkd_crypto::bls12_377::{
    test::tests::{
        affine::{G1AffineTuple, G2AffineTuple},
        field::{Fp12Tuple, Fp2Tuple, Fp6Tuple},
        projective::{G1ProjectiveTuple, G2ProjectiveTuple},
    },
    Affine, Field, Fp, Fp12, Fp2, Fp6, G1Parameters, G2Parameters, Projective, SWAffine,
    SWProjective, Scalar,
};
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
    /// the input types to generate for each case
    #[arg(value_enum)]
    input: Vec<Input>,
    /// path to the output file. prints to terminal if unset
    #[arg(short)]
    output: Option<PathBuf>,
}

impl Args {
    /// used to generate json with `n_tests` input cases for a target
    fn gen(&self, n_tests: usize) -> String {
        serde_json::to_string_pretty(&TestConfig {
            path: Default::default(),
            namespace: self.namespace.clone(),
            method: self.method.clone(),
            tests: self.gen_tests(n_tests),
        })
        .unwrap()
    }

    /// generates `n_tests` input cases for a target
    fn gen_tests(&self, n_tests: usize) -> BTreeMap<String, Case> {
        let pad_len = n_tests.to_string().len() - 1;
        (0..n_tests)
            .map(|i| {
                (
                    format!("random_{i:0pad_len$}"),
                    Case {
                        expectation: TestExpectationMode::Pass,
                        input: self.gen_input(),
                    },
                )
            })
            .collect()
    }

    /// generates input case for a target
    fn gen_input(&self) -> Value {
        if self.input.len() == 1 {
            self.input[0].gen()
        } else {
            self.input
                .iter()
                .map(|v| v.gen())
                .collect::<Vec<_>>()
                .into()
        }
    }
}

/// The different types for inputs
#[derive(Debug, Clone, Copy, clap::ValueEnum)]
enum Input {
    Fp,
    VecFp,
    Fp2,
    VecFp2,
    Fp6,
    VecFp6,
    Fp12,
    VecFp12,
    Scalar,
    VecScalar,
    G1P,
    VecG1P,
    G2P,
    VecG2P,
    G1A,
    VecG1A,
    G2A,
    VecG2A,
    Bool,
}

impl Input {
    /// generates random json value for the input type
    fn gen(&self) -> Value {
        let gen_vec = |f: fn() -> Value| -> Value {
            (0..thread_rng().gen_range(0..10))
                .map(|_| f())
                .collect::<Vec<_>>()
                .into()
        };

        match self {
            Input::Fp => Self::gen_fp(),
            Input::VecFp => gen_vec(Self::gen_fp),
            Input::Fp2 => Self::gen_fp2(),
            Input::VecFp2 => gen_vec(Self::gen_fp2),
            Input::Fp6 => Self::gen_fp6(),
            Input::VecFp6 => gen_vec(Self::gen_fp6),
            Input::Fp12 => Self::gen_fp12(),
            Input::VecFp12 => gen_vec(Self::gen_fp12),
            Input::Scalar => Self::gen_scalar(),
            Input::VecScalar => gen_vec(Self::gen_scalar),
            Input::G1P => Self::gen_g1a(),
            Input::VecG1P => gen_vec(Self::gen_g1a),
            Input::G2P => Self::gen_g2a(),
            Input::VecG2P => gen_vec(Self::gen_g2a),
            Input::G1A => Self::gen_g1p(),
            Input::VecG1A => gen_vec(Self::gen_g1p),
            Input::G2A => Self::gen_g2p(),
            Input::VecG2A => gen_vec(Self::gen_g2p),
            Input::Bool => Self::gen_bool(),
        }
    }

    fn gen_fp() -> Value {
        serde_json::to_value(Fp::rand()).unwrap()
    }

    fn gen_fp2() -> Value {
        serde_json::to_value(Fp2Tuple::from(Fp2::rand())).unwrap()
    }

    fn gen_fp6() -> Value {
        serde_json::to_value(Fp6Tuple::from(Fp6::rand())).unwrap()
    }

    fn gen_fp12() -> Value {
        serde_json::to_value(Fp12Tuple::from(Fp12::rand())).unwrap()
    }

    fn gen_scalar() -> Value {
        serde_json::to_value(Scalar::rand()).unwrap()
    }

    fn gen_g1a() -> Value {
        serde_json::to_value(G1ProjectiveTuple::from(SWProjective::<G1Parameters>::rand())).unwrap()
    }

    fn gen_g2a() -> Value {
        serde_json::to_value(G2ProjectiveTuple::from(SWProjective::<G2Parameters>::rand())).unwrap()
    }

    fn gen_g1p() -> Value {
        serde_json::to_value(G1AffineTuple::from(SWAffine::<G1Parameters>::rand())).unwrap()
    }

    fn gen_g2p() -> Value {
        serde_json::to_value(G2AffineTuple::from(SWAffine::<G2Parameters>::rand())).unwrap()
    }

    fn gen_bool() -> Value {
        serde_json::to_value(thread_rng().gen::<bool>()).unwrap()
    }
}

fn main() {
    let args = Args::parse();
    let tests = args.gen(args.n_tests);

    if let Some(mut out) = args.output {
        if out.is_dir() {
            out.push(format!("generated_{}.json", args.method))
        }
        write(&out, tests).unwrap();
        println!("wrote output to {}", out.display());
    } else {
        println!("{tests}");
    };
}

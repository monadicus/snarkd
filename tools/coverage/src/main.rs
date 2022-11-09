use std::{
    env,
    path::PathBuf,
    process::{Command, Stdio},
};

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Options {
    module: String,
    namespace: Option<String>,
}

fn cargo(cmd: &str, args: &[String]) {
    dbg!(cmd);
    let (key, value) = match cmd {
        "build" => ("RUSTFLAGS", "-Cinstrument-coverage"),
        "test" => (
            "LLVM_PROFILE_FILE",
            "./target/out/snarkd_coverage-%p-%m.profraw",
        ),
        _ => ("", ""),
    };
    let mut tmp = Command::new("cargo");
    tmp.env(key, value)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .arg(cmd)
        .args(args);
    dbg!(&tmp);
    let stat = tmp
        .spawn()
        .expect("Failed to run cargo command.")
        .wait_with_output()
        .unwrap()
        .status
        .success();
    assert!(stat);
}

fn grcov() {
    let stat = Command::new("grcov")
        .env(
            "LLVM_PROFILE_FILE",
            "../target/out/snarkd_coverage-%p-%m.profraw",
        )
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .args([
            "../target/out",
            "-s",
            ".",
            "--binary-path",
            "../target/debug/",
            "-t",
            "html",
            "--branch",
            "--ignore-not-existing",
            "-o",
            "../target/debug/coverage/",
        ])
        .spawn()
        .expect("Failed to run grcov command.")
        .wait_with_output()
        .unwrap()
        .status
        .success();
    assert!(stat)
}

fn main() {
    let options = Options::parse();
    let namespace = options.namespace.unwrap_or_default();
    let cargo_args = &[namespace];

    // cargo("clean", &[]);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push(options.module);
    dbg!(&path);
    env::set_current_dir(&path).expect("Failed to set cwd.");

    cargo("build", &[]);

    cargo("test", cargo_args);
    grcov();

    path.pop();
    env::set_current_dir(path).expect("Failed to set cwd.");

    let stat = Command::new("rm")
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .arg("default_*.profraw")
        .spawn()
        .expect("Failed to run rm command.")
        .wait_with_output()
        .unwrap()
        .status
        .success();
    assert!(stat);
}

/*
cargo install grcov && rustup component add llvm-tools-preview

cargo clean
cd snarkd_crypto
export RUSTFLAGS=-Cinstrument-coverage
cargo build
export LLVM_PROFILE_FILE=../target/out/snarkd_coverage-%p-%m.profraw
cargo test -- --exact
grcov ../target/out -s . --binary-path ../target/debug/ -t html --branch --ignore-not-existing -o ../target/debug/coverage/
cd ..
rm default_*.profraw
export RUSTFLAGS=
export LLVM_PROFILE_FILE=
*/

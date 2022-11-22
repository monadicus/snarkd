use std::{
    env::{args, current_dir, set_current_dir},
    fs::remove_dir_all,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

const WORKSPACE_NAME: &str = "snarkd";
const TMP_OUTPUT: &str = "coverage_tmp";
const COVERAGE_OUTPUT: &str = "coverage";

/// a hack to point to the absolute path to the snarkd workspace root.
/// this is to avoid situations where people are `cd`'d into the coverage crate
fn set_current_dir_to_snarkd() {
    let mut dir = current_dir().unwrap();
    if !dir.ends_with(WORKSPACE_NAME) {
        dir = dir
            .iter()
            .take_while(|p| *p != WORKSPACE_NAME)
            .collect::<PathBuf>();
        dir.push(WORKSPACE_NAME);
    }
    set_current_dir(dir).unwrap();
}

/// runs a command with the given args and environment variables and returns its exit status
fn run_command(cmd: &str, args: &[String], env: &[(String, String)]) -> ExitStatus {
    let mut command = Command::new(cmd);
    for (key, val) in env {
        command.env(key, val);
    }
    command.args(args);
    command
        .spawn()
        .expect("Failed to run command")
        .wait_with_output()
        .unwrap()
        .status
}

/// cargo install grcov && rustup component add llvm-tools-preview
fn install_required() {
    assert!(run_command("cargo", &["install".into(), "grcov".into()], &[]).success());
    assert!(run_command(
        "rustup",
        &[
            "component".into(),
            "add".into(),
            "llvm-tools-preview".into(),
        ],
        &[],
    )
    .success());
}

/// runs `cargo clean`. ignores failures
fn cargo_clean() {
    run_command("cargo", &["clean".into()], &[]);
}

/// runs `cargo test` with coverage and appends the `additional_args` to the command
fn cargo_test(additional_args: &[String]) {
    let mut args = vec!["test".to_string()];
    args.extend_from_slice(additional_args);

    assert!(run_command(
        "cargo",
        &args,
        &[
            ("RUSTFLAGS".into(), "-Cinstrument-coverage".into()),
            (
                "LLVM_PROFILE_FILE".into(),
                current_dir()
                    .unwrap()
                    .join("target")
                    .join(TMP_OUTPUT)
                    .join("snarkd_coverage-%p-%m.profraw")
                    .display()
                    .to_string()
            ),
        ],
    )
    .success())
}

/// compiles coverage generated by `cargo_test`
fn grcov() {
    let tmp_output = Path::new("target").join(TMP_OUTPUT);
    assert!(run_command(
        "grcov",
        &[
            tmp_output.display().to_string(),
            "-s".into(),
            ".".into(),
            "--binary-path".into(),
            Path::new("target").join("debug").display().to_string(),
            "-t".into(),
            "html".into(),
            "--branch".into(),
            "--ignore-not-existing".into(),
            "-o".into(),
            Path::new("target")
                .join(COVERAGE_OUTPUT)
                .display()
                .to_string(),
        ],
        &[],
    )
    .success());
    remove_dir_all(tmp_output).unwrap();
}

fn main() {
    let args = args().skip(1).collect::<Vec<_>>();
    println!("COVERAGE: installing deps");
    install_required();
    println!("COVERAGE: switching dir");
    set_current_dir_to_snarkd();
    println!("COVERAGE: cleaning");
    cargo_clean();
    println!("COVERAGE: testing");
    cargo_test(&args);
    println!("COVERAGE: compiling coverage");
    grcov();
    println!("COVERAGE: opening coverage report in browser");
    open::that(Path::new("target").join(COVERAGE_OUTPUT).join("index.html")).unwrap();
}

/*
cargo install grcov && rustup component add llvm-tools-preview
cargo clean
export RUSTFLAGS=-Cinstrument-coverage
export LLVM_PROFILE_FILE=$PWD/target/coverage_tmp/snarkd_coverage-%p-%m.profraw
cargo test -p snarkd_ir
export RUSTFLAGS=
export LLVM_PROFILE_FILE=
grcov target/coverage_tmp -s . --binary-path target/debug/ -t html --branch --ignore-not-existing -o target/coverage/
rmdir target/coverage_tmp
*/
fn main() {
    prost_build::compile_protos(&["proto/snarkd.proto"], &["proto/"])
        .expect("failed to build proto");
}

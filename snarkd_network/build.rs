fn main() {
    prost_build::Config::default()
        .extern_path(".snarkd_common.Digest", "::snarkd_common::Digest")
        .compile_protos(&["proto/snarkd.proto"], &["proto/"])
        .expect("failed to build proto");
}

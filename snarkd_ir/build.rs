fn main() {
    prost_build::Config::new()
        .include_file("_includes.rs")
        .compile_protos(&["proto/ir.proto"], &["proto/"])
        .unwrap_or_else(|e| panic!("failed to build IR protobuf: {e}"));
}

fn main() {
    prost_build::compile_protos(&["proto/ir.proto"], &["proto/"])
        .expect("failed to build IR protobuf");
}

fn main() {
    tonic_build::compile_protos("api/proto/peer.proto")
        .unwrap_or_else(|e| panic!("Failed to compile protos {:?}", e));
}

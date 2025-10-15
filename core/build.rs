fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .extern_path(".mtid", "::mtid::prost")
        .compile_protos(
            &[
                "proto/caretta_sync/authorization_request/authorization_request.proto",
                "proto/caretta_sync/authorized_node/authorized_node.proto",
                "proto/caretta_sync/remote_node/remote_node.proto",
            ],
            &["proto", "../mtid/proto"],
        )?;
    Ok(())
}

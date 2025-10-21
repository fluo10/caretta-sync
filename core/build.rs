fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .extern_path(".mtid", "::mtid::proto")
        .compile_protos(
            &[
                "caretta-sync-proto/caretta_sync/authorization_request/authorization_request.proto",
                "caretta-sync-proto/caretta_sync/authorized_node/authorized_node.proto",
                "caretta-sync-proto/caretta_sync/remote_node/remote_node.proto",
            ],
            &["caretta-sync-proto", "mtid-proto"],
        )?;
    Ok(())
}

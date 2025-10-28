fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .extern_path(".mtid", "::mtid::proto")
        .compile_protos(
            &[
                "caretta-sync-proto/caretta_sync/api/device/device_service.proto",
                "caretta-sync-proto/caretta_sync/doc/authorized_node.proto",
                "caretta-sync-proto/caretta_sync/types/uuid/uuid.proto",
            ],
            &["caretta-sync-proto", "mtid-proto"],
        )?;
    Ok(())
}

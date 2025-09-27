fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure()
        .extern_path(".tripod_id", "::tripod_id::prost::generated")
        .compile_protos(
            &["proto/caretta_sync.proto", "proto/caretta_sync.common.proto"],
            &["proto"]
        )?;
    Ok(())
}
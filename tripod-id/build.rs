fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature="prost")]
    prost_build::compile_protos(
        &[
            "proto/tripod_id/single.proto",
            "proto/tripod_id/double.proto",
            "proto/tripod_id/triple.proto"
        ],
        &["proto/"]
    )?;
    Ok(())
}
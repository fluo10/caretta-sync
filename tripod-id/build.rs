fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature="prost")]
    prost_build::compile_protos(
        &["proto/tripod_id.proto"],
        &["proto/"]
    )?;
    Ok(())
}
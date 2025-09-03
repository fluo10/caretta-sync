fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/caretta_sync.proto")?;
    tonic_prost_build::compile_protos("proto/iroh.proto")?;

    Ok(())
}
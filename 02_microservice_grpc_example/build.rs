// build script for cargo and configures tonic build
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/payments.proto")?;
    Ok(())
}
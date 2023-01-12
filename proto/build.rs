fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("./proto/issues/issues.proto")?;
    tonic_build::compile_protos("./proto/eventbus/*")?;
    Ok(())
}
const PROTO: &str = "../proto/kvstore/kvstore.proto";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed={PROTO}");
    mrpc_build::compile_protos(PROTO)?;
    Ok(())
}

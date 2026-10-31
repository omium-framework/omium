use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), standard::runtime::RuntimeError> {
    let config_dir = std::env::var("RUSTBRIDGE_CONFIG_DIR").map_or_else(
        |_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config"),
        PathBuf::from,
    );
    standard::runtime::run(config_dir).await
}

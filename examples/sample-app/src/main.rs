//! Sample application binary entry point.

use std::path::PathBuf;

use sample_app::runtime;

#[tokio::main]
async fn main() -> Result<(), runtime::RuntimeError> {
    let config_dir = std::env::var("RUSTBRIDGE_CONFIG_DIR").map_or_else(
        |_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config"),
        PathBuf::from,
    );
    runtime::run(config_dir).await
}

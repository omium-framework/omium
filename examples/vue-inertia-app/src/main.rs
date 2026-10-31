use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), vue_inertia_app::runtime::RuntimeError> {
    let config_dir = std::env::var("RUSTBRIDGE_CONFIG_DIR").map_or_else(
        |_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config"),
        PathBuf::from,
    );
    vue_inertia_app::runtime::run(config_dir).await
}

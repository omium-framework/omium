use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), inertia_vue_app::runtime::RuntimeError> {
    let config_dir = std::env::var("RUSTBRIDGE_CONFIG_DIR").map_or_else(
        |_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config"),
        PathBuf::from,
    );
    inertia_vue_app::runtime::run(config_dir).await
}

//! `omnisan` binary entry (D-017).

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    omium_cli::run_as("omnisan").await
}

//! `rustbridge` CLI entry point (placeholder public name — O-001).

mod commands;
mod generator;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "rustbridge",
    about = "RustBridge developer CLI (internal placeholder name)",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the HTTP server using ./config (or --config-dir)
    Serve(commands::ServeArgs),
    /// Validate local environment and configuration
    Doctor(commands::DoctorArgs),
    /// List registered framework routes
    RouteList,
    /// Generate a new application skeleton
    New(generator::NewArgs),
    /// Generate a controller module stub
    #[command(name = "make:controller")]
    MakeController(generator::MakeArgs),
    /// Generate a service module stub
    #[command(name = "make:service")]
    MakeService(generator::MakeArgs),
    /// Generate a model module stub
    #[command(name = "make:model")]
    MakeModel(generator::MakeArgs),
    /// Generate a migration stub in migration/src/
    #[command(name = "make:migration")]
    MakeMigration(generator::MakeMigrationArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Serve(args)) => commands::serve(args).await,
        Some(Commands::Doctor(args)) => commands::doctor(&args),
        Some(Commands::RouteList) => {
            commands::route_list();
            Ok(())
        }
        Some(Commands::New(args)) => generator::new_app(&args),
        Some(Commands::MakeController(args)) => generator::make_controller(&args),
        Some(Commands::MakeService(args)) => generator::make_service(&args),
        Some(Commands::MakeModel(args)) => generator::make_model(&args),
        Some(Commands::MakeMigration(args)) => generator::make_migration(&args),
        None => {
            use clap::CommandFactory;
            Cli::command().print_help()?;
            println!();
            Ok(())
        }
    }
}

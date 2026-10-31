//! Omium developer CLI (`omnisan` binary).

pub mod commands;
pub mod generator;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    about = "Omium developer CLI (omnisan — Laravel Artisan-style workflow)",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Install dependencies (`npm install` for Vue/Inertia apps)
    Install(commands::InstallArgs),
    /// Local development with Vite hot reload (Vue/Inertia)
    Dev(commands::DevArgs),
    /// Build frontend assets and optional release binary
    Build(commands::BuildArgs),
    /// Apply `SeaORM` migrations (`migration/` crate)
    Migrate(commands::MigrateArgs),
    /// HTTP server (debug Vue apps start Vite dev + HMR automatically)
    Serve(commands::ServeArgs),
    /// Run the application without Vite (same as `serve --production` for Vue)
    Run(commands::RunArgs),
    /// Validate local environment and configuration
    Doctor(commands::DoctorArgs),
    /// List registered HTTP routes
    #[command(name = "route:list", visible_alias = "route-list")]
    RouteList(commands::RouteListArgs),
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
    /// Install `omnisan` into ~/.cargo/bin (run once per machine)
    #[command(name = "self-install", visible_alias = "self:install")]
    SelfInstall,
}

/// Parses CLI arguments and runs the selected command.
///
/// # Errors
///
/// Returns when argument parsing or the selected command fails.
pub async fn run_as(program_name: &'static str) -> Result<()> {
    let mut args: Vec<String> = std::env::args().collect();
    if args.is_empty() {
        args.push(program_name.to_string());
    } else {
        args[0] = program_name.to_string();
    }
    let cli = Cli::parse_from(args);
    dispatch(cli).await
}

async fn dispatch(cli: Cli) -> Result<()> {
    match cli.command {
        Some(Commands::Install(args)) => commands::install(args).await,
        Some(Commands::Dev(args)) => commands::dev(args).await,
        Some(Commands::Build(args)) => commands::build(args).await,
        Some(Commands::Migrate(args)) => commands::migrate(args).await,
        Some(Commands::Serve(args)) => commands::serve(args).await,
        Some(Commands::Run(args)) => commands::run_app(args).await,
        Some(Commands::Doctor(args)) => commands::doctor(&args),
        Some(Commands::RouteList(args)) => commands::route_list(args).await,
        Some(Commands::New(args)) => generator::new_app(&args),
        Some(Commands::MakeController(args)) => generator::make_controller(&args),
        Some(Commands::MakeService(args)) => generator::make_service(&args),
        Some(Commands::MakeModel(args)) => generator::make_model(&args),
        Some(Commands::MakeMigration(args)) => generator::make_migration(&args),
        Some(Commands::SelfInstall) => commands::self_install(),
        None => {
            use clap::CommandFactory;
            Cli::command().print_help()?;
            println!();
            Ok(())
        }
    }
}

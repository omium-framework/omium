//! CLI command handlers.

pub mod build;
pub mod dev;
pub mod doctor;
pub mod install;
pub mod migrate;
pub mod project;
pub mod route_list;
pub mod run;
pub mod self_install;
pub mod serve;

pub use build::{BuildArgs, build};
pub use dev::{DevArgs, dev};
pub use doctor::{DoctorArgs, doctor};
pub use install::{InstallArgs, install};
pub use migrate::{MigrateArgs, migrate};
pub use route_list::{RouteListArgs, route_list};
pub use run::{RunArgs, run_app};
pub use self_install::self_install;
pub use serve::{ServeArgs, serve};

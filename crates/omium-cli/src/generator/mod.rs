//! Project and code generators (template-based).

mod api;
mod crate_deps;
pub(crate) mod framework_paths;
mod make;
mod new;
mod new_prompts;
mod omnisan_script;
mod react;
mod standalone_manifest;
mod standard;
mod template_copy;
mod template_design;
mod template_manifest;
mod vue;

pub use make::{
    MakeArgs, MakeMigrationArgs, make_controller, make_migration, make_model, make_service,
};
pub use new::{NewArgs, new_app};

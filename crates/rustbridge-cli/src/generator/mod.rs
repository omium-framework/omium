//! Project and code generators (template-based).

mod make;
mod new;

pub use make::{
    MakeArgs, MakeMigrationArgs, make_controller, make_migration, make_model, make_service,
};
pub use new::{NewArgs, new_app};

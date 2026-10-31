//! [`inertia_rs`] Axum integration helpers (Phase 7 bridge).

mod form;
mod presentation;
mod render;
mod validation;
mod vite;

pub use form::InertiaForm;
pub use presentation::inertia_presentation_props;
pub use render::{HtmlShell, render_page};
pub use validation::validation_errors_prop;
pub use vite::{ViteAssetError, vite_dev_tags, vite_production_tags, vite_react_dev_tags};

pub use inertia_rs::axum::{InertiaError, InertiaRequest, SharedProps, VersionLayer};
pub use inertia_rs::{Inertia, InertiaProps};

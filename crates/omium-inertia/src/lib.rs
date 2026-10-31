//! Inertia integration for Vue/React modes (adapter behind feature flags).
//!
//! Phase 7 uses the approved **`inertia_rs`** bridge. See `docs/INERTIA_SPIKE_REPORT.md`.

mod adapter;

#[cfg(feature = "inertia-rs")]
pub mod inertia_rs;

pub use adapter::{InertiaAdapter, InertiaAdapterError};

pub use omium_core::FRAMEWORK_CODENAME;

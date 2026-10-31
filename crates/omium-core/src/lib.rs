//! Core primitives for `Omium`. This crate must not depend on HTTP, database, or frontend crates.

pub mod bootstrap;
pub mod config;
pub mod error;
pub mod presentation;
pub mod shutdown;
pub mod state;
pub mod telemetry;

/// Working product codename. Public naming requires owner approval (O-001).
pub const FRAMEWORK_CODENAME: &str = "Omium";

/// Returns the provisional framework codename.
#[must_use]
pub fn framework_codename() -> &'static str {
    FRAMEWORK_CODENAME
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codename_is_non_empty() {
        assert!(!framework_codename().is_empty());
    }
}

//! Merge [`RequestPresentation`] into Inertia page props.

use omium_core::presentation::RequestPresentation;
use inertia_rs::InertiaProps;

/// Adds `nav`, `flash`, and `currentPath` props for shared layouts.
#[must_use]
pub fn inertia_presentation_props(presentation: &RequestPresentation) -> InertiaProps {
    InertiaProps::new()
        .value("currentPath", presentation.current_path.clone())
        .value(
            "nav",
            serde_json::to_value(&presentation.nav).unwrap_or(serde_json::Value::Null),
        )
        .value(
            "flash",
            serde_json::to_value(&presentation.flash).unwrap_or(serde_json::Value::Null),
        )
}

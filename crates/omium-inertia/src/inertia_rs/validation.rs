//! Map server-side validation errors to Inertia page props.

use omium_view::ValidationErrors;
use serde_json::Value;

/// Builds the Inertia `errors` prop object from framework validation errors.
#[must_use]
pub fn validation_errors_prop(errors: &ValidationErrors) -> Value {
    errors.to_inertia_value()
}

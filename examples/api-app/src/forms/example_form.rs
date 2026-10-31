//! Example form validation (shared rules with sample-app).

use omium_view::ValidationErrors;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct ExampleFormInput {
    pub name: String,
    pub description: String,
}

/// Validates form input and returns field errors (does not hit the database).
#[must_use]
pub fn validate_example_form(input: &ExampleFormInput) -> ValidationErrors {
    let mut errors = ValidationErrors::new();
    let name = input.name.trim();
    if name.is_empty() {
        errors.add("name", "Name is required.");
    } else if name.contains('<') || name.contains('>') {
        errors.add("name", "Name must not contain angle brackets.");
    }
    let description = input.description.trim();
    if description.contains('<') || description.contains('>') {
        errors.add(
            "description",
            "Description must not contain angle brackets.",
        );
    }
    errors
}

pub fn normalized_description(input: &ExampleFormInput) -> Option<String> {
    let trimmed = input.description.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

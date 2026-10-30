//! Create-example form with validation error rendering.

use askama::Template;
use rustbridge_view::ValidationErrors;

#[derive(Template)]
#[template(path = "examples/new.html")]
pub struct ExampleNewTemplate {
    pub app_name: String,
    pub page_title: String,
    pub name: String,
    pub description: String,
    pub errors: ValidationErrors,
}

#[derive(serde::Deserialize)]
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

impl ExampleNewTemplate {
    #[must_use]
    pub fn blank(app_name: String) -> Self {
        Self {
            app_name,
            page_title: String::from("New example"),
            name: String::new(),
            description: String::new(),
            errors: ValidationErrors::new(),
        }
    }

    #[must_use]
    pub fn from_input(
        app_name: String,
        input: &ExampleFormInput,
        errors: ValidationErrors,
    ) -> Self {
        Self {
            app_name,
            page_title: String::from("New example"),
            name: input.name.clone(),
            description: input.description.clone(),
            errors,
        }
    }
}

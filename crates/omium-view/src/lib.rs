//! Server-rendered HTML via Askama behind a small framework-facing API.
//!
//! Reference layout: [`templates/layout.html`](../../templates/layout.html). Application
//! crates copy or symlink that file into their own `templates/` directory (Askama resolves
//! `{% extends "layout.html" %}` from the app manifest dir).

mod context;
mod error;
mod render;

pub use context::{PageContext, ValidationErrors, WebPageShell};
pub use error::ViewError;
pub use render::into_html;

pub use omium_core::FRAMEWORK_CODENAME;
pub use omium_core::presentation::{FlashMessages, NavLink, RequestPresentation};

#[cfg(test)]
mod tests {
    use askama::Template;

    use super::ValidationErrors;

    #[derive(Template)]
    #[template(path = "escape_check.html", ext = "html")]
    struct EscapeCheck {
        user_input: String,
    }

    #[test]
    fn validation_errors_expose_field_messages() {
        let mut errors = ValidationErrors::new();
        errors.add("name", "Name is required.");
        assert!(errors.has_field("name"));
        assert_eq!(errors.messages("name"), ["Name is required."]);
        assert!(!errors.has_field("email"));
    }

    #[test]
    fn inertia_value_uses_string_or_array_per_field() {
        let mut errors = ValidationErrors::new();
        errors.add("name", "Name is required.");
        errors.add("description", "Too short.");
        errors.add("description", "Invalid characters.");
        let value = errors.to_inertia_value();
        assert_eq!(value["name"], "Name is required.");
        assert!(value["description"].is_array());
    }

    #[test]
    fn askama_escapes_interpolated_html() {
        let rendered = EscapeCheck {
            user_input: String::from("<script>alert(1)</script>"),
        }
        .render()
        .expect("render");
        assert!(!rendered.contains("<script>"));
        assert!(rendered.contains("alert(1)"));
        assert!(
            rendered.contains("&lt;") || rendered.contains("&#60;") || rendered.contains("&#x3c;"),
            "expected escaped angle brackets, got: {rendered}"
        );
    }
}

//! Server-rendered HTML via Askama behind a small framework-facing API.
//!
//! Reference layout: [`templates/layout.html`](../../templates/layout.html). Application
//! crates copy or symlink that file into their own `templates/` directory (Askama resolves
//! `{% extends "layout.html" %}` from the app manifest dir).

mod context;
mod error;
mod render;

pub use context::{PageContext, ValidationErrors};
pub use error::ViewError;
pub use render::into_html;

pub use rustbridge_core::FRAMEWORK_CODENAME;

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

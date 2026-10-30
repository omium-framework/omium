//! Askama templates for sample-app pages.

mod example_form;
mod home;

pub use example_form::{ExampleFormInput, ExampleNewTemplate, validate_example_form};
pub use home::{ExampleRow, HomeTemplate};

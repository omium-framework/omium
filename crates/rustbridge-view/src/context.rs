//! Typed page and validation context for templates.

use std::collections::BTreeMap;

/// Shared metadata for standard HTML pages (layout title, branding).
#[derive(Clone, Debug)]
pub struct PageContext {
    pub app_name: String,
    pub page_title: String,
}

impl PageContext {
    #[must_use]
    pub fn new(app_name: impl Into<String>, page_title: impl Into<String>) -> Self {
        Self {
            app_name: app_name.into(),
            page_title: page_title.into(),
        }
    }
}

/// Field-level validation messages for forms (Laravel `MessageBag`-style, simplified).
#[derive(Clone, Debug, Default)]
pub struct ValidationErrors {
    fields: BTreeMap<String, Vec<String>>,
}

impl ValidationErrors {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.fields
            .entry(field.into())
            .or_default()
            .push(message.into());
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty()
    }

    /// Whether the given field has at least one error message.
    #[must_use]
    pub fn has_field(&self, field: &str) -> bool {
        self.fields
            .get(field)
            .is_some_and(|messages| !messages.is_empty())
    }

    /// Messages for a field (empty slice when none).
    #[must_use]
    pub fn messages(&self, field: &str) -> &[String] {
        self.fields.get(field).map_or(&[], Vec::as_slice)
    }
}

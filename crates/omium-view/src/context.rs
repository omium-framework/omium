//! Shared metadata for standard HTML pages (layout title, branding, nav, flash).

use std::collections::BTreeMap;

use omium_core::presentation::{FlashMessages, NavLink, RequestPresentation};

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

/// Layout fields every standard page receives from [`RequestPresentation`].
#[derive(Clone, Debug)]
pub struct WebPageShell {
    pub app_name: String,
    pub page_title: String,
    pub nav: Vec<NavLink>,
    pub flash: FlashMessages,
}

impl WebPageShell {
    #[must_use]
    pub fn from_presentation(
        app_name: impl Into<String>,
        page_title: impl Into<String>,
        presentation: &RequestPresentation,
    ) -> Self {
        Self {
            app_name: app_name.into(),
            page_title: page_title.into(),
            nav: presentation.nav.clone(),
            flash: presentation.flash.clone(),
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

    /// Serializes field errors for Inertia's `errors` page prop (Laravel-compatible shape).
    #[must_use]
    pub fn to_inertia_value(&self) -> serde_json::Value {
        let mut object = serde_json::Map::new();
        for (field, messages) in &self.fields {
            let value = if messages.len() == 1 {
                serde_json::Value::String(messages[0].clone())
            } else {
                serde_json::Value::Array(
                    messages
                        .iter()
                        .cloned()
                        .map(serde_json::Value::String)
                        .collect(),
                )
            };
            object.insert(field.clone(), value);
        }
        serde_json::Value::Object(object)
    }
}

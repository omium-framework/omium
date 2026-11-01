//! HTML shell and Inertia page rendering for Axum handlers.

use axum::response::{Html, IntoResponse, Response};
use inertia_rs::axum::{InertiaError, InertiaRequest};
use inertia_rs::{Inertia, InertiaProps};

/// Root HTML document pieces around the Inertia page payload.
pub struct HtmlShell<'a> {
    pub title: &'a str,
    pub vite_tags: &'a str,
    pub body_class: Option<&'a str>,
    /// Session CSRF token for `<meta name="csrf-token">` (Inertia / fetch clients).
    pub csrf_token: Option<&'a str>,
}

/// Renders an Inertia page as HTML (first visit) or JSON (Inertia XHR).
///
/// # Errors
///
/// Returns [`InertiaError`] when prop serialization or protocol handling fails.
pub fn render_page(
    request: &InertiaRequest,
    component: &str,
    props: InertiaProps,
    shell: &HtmlShell<'_>,
) -> Result<Response, InertiaError> {
    let body_class = shell.body_class.unwrap_or("");
    let title = shell.title;
    let vite_tags = shell.vite_tags;
    let csrf_meta = shell
        .csrf_token
        .map(|token| format!(r#"<meta name="csrf-token" content="{token}" />"#))
        .unwrap_or_default();
    request.render(Inertia::response(component, props), |context| {
        Html(format!(
            r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{title}</title>
  {csrf_meta}
  {vite_tags}
</head>
<body class="{body_class}">
  <script type="application/json" data-page="app">{}</script>
  <div id="app"></div>
</body>
</html>"#,
            context.data_page()
        ))
        .into_response()
    })
}

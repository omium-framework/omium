//! Inertia page rendering helpers.

use std::borrow::Cow;

use axum::response::Response;
use omium_core::presentation::RequestPresentation;
use omium_inertia::inertia_rs::{
    HtmlShell, InertiaProps, InertiaRequest, inertia_presentation_props, render_page,
    validation_errors_prop,
};
use omium_view::ValidationErrors;

use crate::app::{RuntimeError, RuntimeState, resolve_vite_tags};

/// Builds Inertia props for the example form page.
#[must_use]
pub fn example_form_props(
    base: InertiaProps,
    heading: impl Into<String>,
    lead: impl Into<String>,
    form_action: impl Into<String>,
    name: String,
    description: String,
    errors: ValidationErrors,
) -> InertiaProps {
    base.value("heading", heading.into())
        .value("lead", lead.into())
        .value("formAction", form_action.into())
        .value("name", name)
        .value("description", description)
        .value("errors", validation_errors_prop(&errors))
}

/// Renders an Inertia page (HTML first visit or JSON visit).
pub fn render_inertia(
    state: &RuntimeState,
    request: &InertiaRequest,
    presentation: &RequestPresentation,
    authenticated: bool,
    component: &str,
    title_suffix: &str,
    build_props: impl FnOnce(InertiaProps) -> InertiaProps,
) -> Result<Response, RuntimeError> {
    let app_name = state.app.config().app.name.clone();
    let vite_tags = vite_tags_for_shell(state)?;
    let props = build_props(
        inertia_presentation_props(presentation).value("authenticated", authenticated),
    );
    render_page(
        request,
        component,
        props,
        &HtmlShell {
            title: &format!("{app_name} — {title_suffix}"),
            vite_tags: vite_tags.as_ref(),
            body_class: Some("antialiased"),
            csrf_token: presentation.csrf_token.as_deref(),
        },
    )
    .map_err(RuntimeError::Inertia)
}

fn vite_tags_for_shell(state: &RuntimeState) -> Result<Cow<'_, str>, RuntimeError> {
    if state.app.config().app.debug {
        Ok(Cow::Owned(resolve_vite_tags(state.app.config())?))
    } else {
        Ok(Cow::Borrowed(state.vite_tags.as_str()))
    }
}

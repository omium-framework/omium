//! Web middleware: navigation + flash on every HTML/Inertia request.

use axum::body::Body;
use axum::extract::State;
use axum::http::header::{COOKIE, SET_COOKIE};
use axum::http::{HeaderValue, Request, Response};
use axum::middleware::Next;
use omium_core::presentation::{
    FLASH_ERROR_COOKIE, FLASH_SUCCESS_COOKIE, FlashMessages, RequestPresentation, build_nav,
    read_flash_from_cookie_header,
};

/// Application navigation entries (`href`, `label`), shared by middleware.
#[derive(Clone, Copy)]
pub struct PresentationNav {
    pub entries: &'static [(&'static str, &'static str)],
}

/// Inserts [`RequestPresentation`] into request extensions and clears consumed flash cookies.
pub async fn inject_request_presentation(
    State(nav): State<PresentationNav>,
    mut request: Request<Body>,
    next: Next,
) -> Response<Body> {
    let current_path = request.uri().path().to_string();
    let cookie_header = request
        .headers()
        .get(COOKIE)
        .and_then(|value| value.to_str().ok());
    let flash = read_flash_from_cookie_header(cookie_header);
    let had_flash = flash.success.is_some() || flash.error.is_some();

    let presentation = RequestPresentation {
        current_path,
        nav: build_nav(nav.entries, request.uri().path()),
        flash,
    };
    request.extensions_mut().insert(presentation);

    let mut response = next.run(request).await;
    if had_flash {
        append_clear_cookie(&mut response, FLASH_SUCCESS_COOKIE);
        append_clear_cookie(&mut response, FLASH_ERROR_COOKIE);
    }
    response
}

fn append_clear_cookie(response: &mut Response<Body>, name: &str) {
    let Ok(value) = HeaderValue::from_str(&format!(
        "{name}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0"
    )) else {
        return;
    };
    response.headers_mut().append(SET_COOKIE, value);
}

fn cookie_encode(input: &str) -> String {
    input
        .chars()
        .map(|ch| match ch {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' => ch.to_string(),
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", ch as u32),
        })
        .collect()
}

/// Queues a one-shot success flash (read on the next web request).
#[must_use]
pub fn flash_success_cookie(message: &str) -> Option<HeaderValue> {
    flash_cookie(FLASH_SUCCESS_COOKIE, message)
}

/// Queues a one-shot error flash (read on the next web request).
#[must_use]
pub fn flash_error_cookie(message: &str) -> Option<HeaderValue> {
    flash_cookie(FLASH_ERROR_COOKIE, message)
}

fn flash_cookie(name: &str, message: &str) -> Option<HeaderValue> {
    if message.is_empty() {
        return None;
    }
    HeaderValue::from_str(&format!(
        "{name}={}; Path=/; HttpOnly; SameSite=Lax; Max-Age=120",
        cookie_encode(message)
    ))
    .ok()
}

/// Attaches flash Set-Cookie headers to a response (redirect or HTML).
pub fn apply_flash_to_response(response: &mut Response<Body>, flash: &FlashMessages) {
    if let Some(message) = flash.success.as_deref()
        && let Some(value) = flash_success_cookie(message)
    {
        response.headers_mut().append(SET_COOKIE, value);
    }
    if let Some(message) = flash.error.as_deref()
        && let Some(value) = flash_error_cookie(message)
    {
        response.headers_mut().append(SET_COOKIE, value);
    }
}

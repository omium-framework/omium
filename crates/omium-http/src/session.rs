//! Session cookie middleware (loads/saves via [`SessionStore`]).

use std::sync::Arc;

use axum::body::Body;
use axum::extract::State;
use axum::http::header::{COOKIE, SET_COOKIE};
use axum::http::{HeaderValue, Request, Response};
use axum::middleware::Next;
use omium_auth::{Session, SessionCookieConfig, SessionHandle, SessionId, SessionStore};
use tokio::sync::RwLock;

/// Shared state for [`start_session`].
#[derive(Clone)]
pub struct SessionMiddlewareState {
    pub store: Arc<dyn SessionStore>,
    pub cookie: SessionCookieConfig,
}

impl SessionMiddlewareState {
    pub fn new(store: Arc<dyn SessionStore>, cookie: SessionCookieConfig) -> Self {
        Self { store, cookie }
    }
}

/// Loads or creates a session, attaches [`SessionHandle`] to the request, persists after the handler.
pub async fn start_session(
    State(state): State<SessionMiddlewareState>,
    mut request: Request<Body>,
    next: Next,
) -> Response<Body> {
    let ip = client_ip(&request);
    let user_agent = user_agent_header(&request);

    let session = match cookie_session_id(&request, &state.cookie.cookie_name) {
        Some(raw) => match SessionId::parse(&raw) {
            Ok(id) => match state.store.load(&id).await {
                Ok(Some(mut existing)) => {
                    existing.set_client_hints(ip.clone(), user_agent.clone());
                    existing
                }
                Ok(None) => {
                    let mut fresh = Session::new();
                    fresh.set_client_hints(ip, user_agent);
                    fresh
                }
                Err(err) => {
                    tracing::warn!(error = %err, "session load failed; starting new session");
                    let mut fresh = Session::new();
                    fresh.set_client_hints(ip, user_agent);
                    fresh
                }
            },
            Err(_) => {
                let mut fresh = Session::new();
                fresh.set_client_hints(ip, user_agent);
                fresh
            }
        },
        None => {
            let mut fresh = Session::new();
            fresh.set_client_hints(ip, user_agent);
            fresh
        }
    };

    let handle: SessionHandle = Arc::new(RwLock::new(session));
    request.extensions_mut().insert(handle.clone());

    let mut response = next.run(request).await;

    let session = handle.read().await;
    if session.is_dirty() {
        if let Err(err) = state.store.save(&session).await {
            tracing::error!(error = %err, "failed to persist session");
        } else if session.is_destroyed() {
            if let Ok(value) = HeaderValue::from_str(&state.cookie.clear_cookie_value()) {
                response.headers_mut().append(SET_COOKIE, value);
            }
        } else if let Ok(value) =
            HeaderValue::from_str(&state.cookie.set_cookie_value(session.id().as_str()))
        {
            response.headers_mut().append(SET_COOKIE, value);
        }
    }

    response
}

fn cookie_session_id(request: &Request<Body>, name: &str) -> Option<String> {
    let header = request.headers().get(COOKIE)?.to_str().ok()?;
    for part in header.split(';') {
        let part = part.trim();
        let (key, value) = part.split_once('=')?;
        if key == name {
            return Some(value.to_string());
        }
    }
    None
}

fn client_ip(request: &Request<Body>) -> Option<String> {
    request
        .extensions()
        .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
        .map(|info| info.ip().to_string())
}

fn user_agent_header(request: &Request<Body>) -> Option<String> {
    request
        .headers()
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Router;
    use axum::routing::get;
    use omium_auth::FileSessionStore;
    use tower::ServiceExt;

    #[tokio::test]
    async fn session_cookie_round_trip() {
        let store = Arc::new(FileSessionStore::new(tempfile::tempdir().unwrap().path()));
        let state = SessionMiddlewareState::new(store, SessionCookieConfig::default());

        let app = Router::new()
            .route(
                "/",
                get(|session: axum::Extension<SessionHandle>| async move {
                    let mut guard = session.write().await;
                    guard.put("count", 1_i32);
                }),
            )
            .layer(axum::middleware::from_fn_with_state(
                state,
                start_session,
            ));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert!(response
            .headers()
            .get(SET_COOKIE)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|c| c.contains("omium_session=")));
    }
}

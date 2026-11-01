//! Asset version layer (Inertia `X-Inertia-Version` handling).
//!
//! Upstream [`inertia_rs::axum::VersionLayer`] treats a missing client version as stale
//! and responds with `409` + `X-Inertia-Location`, which makes `@inertiajs/core` call
//! `window.location.reload()` on every visit. We only force a refresh when the client
//! sends an explicit version that no longer matches the server.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use axum::extract::OriginalUri;
use axum::http::header::{HeaderValue, VARY};
use axum::http::{HeaderMap, Method, Request, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use inertia_rs::axum::InertiaVersion;
use inertia_rs::{X_INERTIA, X_INERTIA_LOCATION, X_INERTIA_VERSION};
use tower::{Layer, Service};

type VersionProvider = Arc<dyn Fn() -> String + Send + Sync>;
type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

fn header<'headers>(headers: &'headers HeaderMap, name: &str) -> Option<&'headers str> {
    headers.get(name).and_then(|value| value.to_str().ok())
}

fn local_uri(uri: &Uri) -> String {
    uri.path_and_query()
        .map(|path_and_query| path_and_query.as_str().to_owned())
        .unwrap_or_else(|| "/".to_owned())
}

fn original_uri_from_extensions<B>(request: &Request<B>) -> String {
    request
        .extensions()
        .get::<OriginalUri>()
        .map(|original_uri| local_uri(&original_uri.0))
        .unwrap_or_else(|| local_uri(request.uri()))
}

fn conflict_response(url: &str) -> Response {
    let mut response = StatusCode::CONFLICT.into_response();
    if let Ok(location) = HeaderValue::from_str(url) {
        response.headers_mut().insert(X_INERTIA_LOCATION, location);
    }
    response
        .headers_mut()
        .append(VARY, HeaderValue::from_static(X_INERTIA));
    response
}

/// Installs Inertia asset version handling for Axum apps.
///
/// Unlike the upstream layer, a missing `X-Inertia-Version` header does not trigger a
/// forced full-page reload (SPA navigation keeps working). A mismatch only occurs when
/// the client sends a version that differs from the server value.
#[derive(Clone)]
pub struct VersionLayer {
    version_provider: VersionProvider,
}

impl VersionLayer {
    /// Creates a layer with a static asset `version`.
    pub fn new<V: Into<String>>(version: V) -> Self {
        let version = version.into();
        Self::dynamic(move || version.clone())
    }

    /// Creates a layer with a dynamic asset-version provider.
    pub fn dynamic<F, V>(version_provider: F) -> Self
    where
        F: Fn() -> V + Send + Sync + 'static,
        V: Into<String>,
    {
        Self {
            version_provider: Arc::new(move || version_provider().into()),
        }
    }
}

impl<S> Layer<S> for VersionLayer {
    type Service = VersionService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        VersionService {
            inner,
            version_provider: self.version_provider.clone(),
        }
    }
}

/// Service produced by [`VersionLayer`].
#[derive(Clone)]
pub struct VersionService<S> {
    inner: S,
    version_provider: VersionProvider,
}

impl<S, B> Service<Request<B>> for VersionService<S>
where
    S: Service<Request<B>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    B: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request<B>) -> Self::Future {
        let version = (self.version_provider)();
        let request_version = header(request.headers(), X_INERTIA_VERSION);
        let stale = request_version.is_some_and(|client| client != version.as_str());

        if request.method() == Method::GET
            && header(request.headers(), X_INERTIA).is_some()
            && stale
        {
            let response =
                conflict_response(&original_uri_from_extensions(&request));
            return Box::pin(async move { Ok(response) });
        }

        request
            .extensions_mut()
            .insert(InertiaVersion::new(version));

        let mut inner = self.inner.clone();
        let future = inner.call(request);
        Box::pin(future)
    }
}

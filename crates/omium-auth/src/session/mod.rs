mod cookie;
mod database;
mod file;
mod id;
mod model;
mod store;

pub use cookie::SessionCookieConfig;
pub use database::DatabaseSessionStore;
pub use file::FileSessionStore;
pub use id::SessionId;
pub use model::{Session, now_unix};
pub use store::SessionStore;

/// Shared mutable session attached to an HTTP request (see `omium_http::start_session`).
pub type SessionHandle = std::sync::Arc<tokio::sync::RwLock<Session>>;

//! Application bootstrap, shared state, and HTTP errors.

mod auth;
mod bootstrap;
mod error;
mod nav;
mod state;

pub use auth::{load_auth_settings, build_api_token_middleware, AuthSettings};
pub use bootstrap::{connect_if_enabled, resolve_vite_tags, ASSET_VERSION, VITE_ENTRY};
pub use error::RuntimeError;
pub use nav::MAIN_NAV;
pub use state::RuntimeState;

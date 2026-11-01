//! Application bootstrap, shared state, and HTTP errors.

mod auth;
mod bootstrap;
mod credentials;
mod demo_users;
mod error;
mod mail;
mod nav;
mod state;

pub use auth::{
    apply_demo_password_override, build_api_token_middleware, load_auth_settings,
    load_auth_settings_with_overrides, AuthSettings,
};
pub use demo_users::{DemoUserError, DemoUserStore};
pub use credentials::{effective_auth, email_is_registered, verify_credentials};
pub use bootstrap::{connect_if_enabled, resolve_vite_tags, ASSET_VERSION, VITE_ENTRY};
pub use error::RuntimeError;
pub use mail::build_app_mailer;
pub use nav::MAIN_NAV;
pub use state::RuntimeState;

//! Askama view structs (templates live under `resources/views/`).

mod login;
mod welcome;

pub use login::LoginTemplate;
pub use welcome::WelcomeTemplate;

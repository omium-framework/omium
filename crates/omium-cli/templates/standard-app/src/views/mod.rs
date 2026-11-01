//! Askama view structs (templates live under `resources/views/`).

mod login;
mod password;
mod register;
mod welcome;

pub use login::LoginTemplate;
pub use password::{ForgotPasswordTemplate, ResetPasswordTemplate};
pub use register::RegisterTemplate;
pub use welcome::WelcomeTemplate;

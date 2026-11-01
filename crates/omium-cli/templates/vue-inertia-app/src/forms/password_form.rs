//! Forgot / reset password forms.

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordForm {
    pub email: String,
    pub _token: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordForm {
    pub password: String,
    pub password_confirmation: String,
    pub _token: String,
}

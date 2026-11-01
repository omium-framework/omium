//! Login form (`POST /login`).

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
    pub _token: String,
}

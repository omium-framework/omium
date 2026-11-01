//! CSRF-only form body (`POST /logout`).

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CsrfForm {
    pub _token: String,
}

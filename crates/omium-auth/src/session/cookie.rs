//! HTTP session cookie settings.

/// Cookie attributes for the session id.
#[derive(Clone, Debug)]
pub struct SessionCookieConfig {
    pub cookie_name: String,
    pub lifetime_seconds: i64,
    pub secure: bool,
}

impl Default for SessionCookieConfig {
    fn default() -> Self {
        Self {
            cookie_name: String::from("omium_session"),
            lifetime_seconds: 7200,
            secure: false,
        }
    }
}

impl SessionCookieConfig {
    /// Builds a `Set-Cookie` header value for the session id.
    #[must_use]
    pub fn set_cookie_value(&self, session_id: &str) -> String {
        let mut value = format!(
            "{}={session_id}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
            self.cookie_name, self.lifetime_seconds
        );
        if self.secure {
            value.push_str("; Secure");
        }
        value
    }

    /// Clears the session cookie (logout).
    #[must_use]
    pub fn clear_cookie_value(&self) -> String {
        format!(
            "{}=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0",
            self.cookie_name
        )
    }
}

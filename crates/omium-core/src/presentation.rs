//! Per-request navigation highlights and one-shot flash messages (cookie-backed).

use serde::{Deserialize, Serialize};

pub const FLASH_SUCCESS_COOKIE: &str = "omium_flash_success";
pub const FLASH_ERROR_COOKIE: &str = "omium_flash_error";

/// Main navigation entry with server-computed active state.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NavLink {
    pub href: String,
    pub label: String,
    pub active: bool,
}

/// Success/error banners consumed once per request (after redirect or form post).
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct FlashMessages {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Built on every web request by middleware; handlers and Inertia props read this.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestPresentation {
    pub current_path: String,
    pub nav: Vec<NavLink>,
    pub flash: FlashMessages,
    /// CSRF synchronizer token (web forms / `X-XSRF-TOKEN`).
    pub csrf_token: Option<String>,
}

/// Marks whether a nav entry is active for `current_path`.
#[must_use]
pub fn nav_link_active(current_path: &str, href: &str) -> bool {
    if href == "/" {
        return current_path == "/";
    }
    current_path == href || current_path.starts_with(&format!("{href}/"))
}

/// Builds navigation with `active` flags from the request path.
#[must_use]
pub fn build_nav(entries: &[(&str, &str)], current_path: &str) -> Vec<NavLink> {
    build_nav_for_user(entries, current_path, false)
}

/// Like [`build_nav`], but omits guest-only entries (e.g. `/login`) when `authenticated`.
#[must_use]
pub fn build_nav_for_user(
    entries: &[(&str, &str)],
    current_path: &str,
    authenticated: bool,
) -> Vec<NavLink> {
    entries
        .iter()
        .filter(|(href, _)| !(authenticated && is_guest_only_nav_href(href)))
        .map(|(href, label)| NavLink {
            href: (*href).to_string(),
            label: (*label).to_string(),
            active: nav_link_active(current_path, href),
        })
        .collect()
}

fn is_guest_only_nav_href(href: &str) -> bool {
    matches!(href, "/login" | "/register")
}

/// Parses flash cookies from the raw `Cookie` header value.
#[must_use]
pub fn read_flash_from_cookie_header(cookie_header: Option<&str>) -> FlashMessages {
    let Some(header) = cookie_header else {
        return FlashMessages::default();
    };
    let mut flash = FlashMessages::default();
    for part in header.split(';') {
        let part = part.trim();
        let Some((name, value)) = part.split_once('=') else {
            continue;
        };
        let decoded = cookie_decode(value.trim());
        if name.trim() == FLASH_SUCCESS_COOKIE {
            flash.success = Some(decoded);
        } else if name.trim() == FLASH_ERROR_COOKIE {
            flash.error = Some(decoded);
        }
    }
    flash
}

fn cookie_decode(input: &str) -> String {
    let mut out = String::new();
    let mut chars = input.chars();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            let h1 = chars.next().unwrap_or('0');
            let h2 = chars.next().unwrap_or('0');
            if let Ok(byte) = u8::from_str_radix(&format!("{h1}{h2}"), 16) {
                out.push(char::from(byte));
            }
            continue;
        }
        if ch == '+' {
            out.push(' ');
            continue;
        }
        out.push(ch);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_active_only_on_exact_match() {
        assert!(nav_link_active("/", "/"));
        assert!(!nav_link_active("/settings", "/"));
    }

    #[test]
    fn nested_path_activates_prefix_routes() {
        assert!(nav_link_active("/settings", "/settings"));
        assert!(nav_link_active("/examples/1/edit", "/examples"));
    }

    #[test]
    fn reads_flash_cookies() {
        let header = "omium_flash_success=Saved%21; omium_flash_error=Bad";
        let flash = read_flash_from_cookie_header(Some(header));
        assert_eq!(flash.success.as_deref(), Some("Saved!"));
        assert_eq!(flash.error.as_deref(), Some("Bad"));
    }

    #[test]
    fn authenticated_nav_hides_sign_in() {
        let entries = [("/", "Home"), ("/login", "Sign in")];
        let nav = build_nav_for_user(&entries, "/", true);
        assert_eq!(nav.len(), 1);
        assert_eq!(nav[0].href, "/");
        let guest = build_nav_for_user(&entries, "/", false);
        assert_eq!(guest.len(), 2);
    }
}

//! Primary navigation entries (used by web presentation middleware).

/// `href`, then label — active state is computed per request in middleware.
pub const MAIN_NAV: &[(&str, &str)] = &[
    ("/", "Home"),
    ("/health", "Health"),
];

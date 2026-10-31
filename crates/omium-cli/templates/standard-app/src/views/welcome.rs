use askama::Template;
use omium_core::presentation::{FlashMessages, NavLink};

#[derive(Template)]
#[template(path = "welcome.html")]
pub struct WelcomeTemplate {
    pub app_name: String,
    pub nav: Vec<NavLink>,
    pub flash: FlashMessages,
    pub greeting: String,
    pub hint: String,
    pub mode_label: String,
}

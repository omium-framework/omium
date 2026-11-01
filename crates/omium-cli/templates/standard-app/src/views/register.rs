use askama::Template;
use omium_core::presentation::{FlashMessages, NavLink};

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {
    pub app_name: String,
    pub nav: Vec<NavLink>,
    pub flash: FlashMessages,
    pub csrf_token: Option<String>,
    pub error: Option<String>,
}

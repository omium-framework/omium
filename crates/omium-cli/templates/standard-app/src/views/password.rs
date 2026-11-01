use askama::Template;
use omium_core::presentation::{FlashMessages, NavLink};

#[derive(Template)]
#[template(path = "forgot_password.html")]
pub struct ForgotPasswordTemplate {
    pub app_name: String,
    pub nav: Vec<NavLink>,
    pub flash: FlashMessages,
    pub csrf_token: Option<String>,
    pub status: Option<String>,
}

#[derive(Template)]
#[template(path = "reset_password.html")]
pub struct ResetPasswordTemplate {
    pub app_name: String,
    pub nav: Vec<NavLink>,
    pub flash: FlashMessages,
    pub csrf_token: Option<String>,
    pub token: String,
    pub error: Option<String>,
}

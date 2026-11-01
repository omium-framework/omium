use askama::Template;
use omium_core::presentation::{FlashMessages, NavLink};

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    pub app_name: String,
    pub nav: Vec<NavLink>,
    pub flash: FlashMessages,
    pub csrf_token: Option<String>,
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn login_form_includes_csrf_hidden_field() {
        let html = LoginTemplate {
            app_name: String::from("Test"),
            nav: Vec::new(),
            flash: FlashMessages::default(),
            csrf_token: Some(String::from("test-csrf-token")),
            error: None,
        }
        .render()
        .expect("render login");

        assert!(html.contains(r#"name="_token""#));
        assert!(html.contains("test-csrf-token"));
    }
}

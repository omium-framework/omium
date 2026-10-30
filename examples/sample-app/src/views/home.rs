//! Home page listing examples.

use askama::Template;

#[derive(Clone)]
pub struct ExampleRow {
    pub id: u32,
    pub name: String,
    pub description: String,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub app_name: String,
    pub page_title: String,
    pub rows: Vec<ExampleRow>,
    pub status_message: Option<String>,
}

impl HomeTemplate {
    #[must_use]
    pub fn empty_db_message(app_name: impl Into<String>) -> Self {
        let app_name = app_name.into();
        Self {
            app_name: app_name.clone(),
            page_title: String::from("Home"),
            rows: vec![ExampleRow {
                id: 0,
                name: String::from("—"),
                description: String::from(
                    "Database disabled. Set APP_DATABASE__ENABLED=true and run migrations.",
                ),
            }],
            status_message: None,
        }
    }

    #[must_use]
    pub fn load_error(app_name: impl Into<String>, message: String) -> Self {
        let app_name = app_name.into();
        Self {
            app_name: app_name.clone(),
            page_title: String::from("Home"),
            rows: Vec::new(),
            status_message: Some(message),
        }
    }
}

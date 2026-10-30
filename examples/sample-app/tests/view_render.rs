//! HTML rendering integration test for sample-app views.

use askama::Template;
use sample_app::views::{ExampleRow, HomeTemplate};

#[test]
fn home_template_renders_table_and_escapes_cell_content() {
    let template = HomeTemplate {
        app_name: String::from("Omium"),
        page_title: String::from("Home"),
        rows: vec![ExampleRow {
            id: 1,
            name: String::from("<unsafe>"),
            description: String::from("Safe & sound"),
        }],
        status_message: None,
    };

    let html = template.render().expect("render");
    assert!(html.contains("Omium"));
    assert!(!html.contains("<unsafe>"));
    assert!(html.contains("unsafe"));
    assert!(html.contains("Safe"));
    assert!(html.contains("sound"));
    assert!(html.contains("<table>"));
}

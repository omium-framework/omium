//! Sample application HTTP runtime and home page.

use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::get;
use rustbridge_core::bootstrap::{BootstrapOptions, bootstrap};
use rustbridge_core::config::AppConfig;
use rustbridge_core::error::BootstrapError;
use rustbridge_core::state::AppState;
use rustbridge_db::{DatabaseError, connect, ping};
use rustbridge_http::{ServeError, apply_default_middleware, health_response, serve_bound_router};
use rustbridge_view::{ViewError, into_html};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::models::Example;
use crate::services::ExampleService;
use crate::views::{
    ExampleFormInput, ExampleNewTemplate, ExampleRow, HomeTemplate, validate_example_form,
};

/// Combined state for sample routes (core config + optional database).
#[derive(Clone)]
pub struct RuntimeState {
    pub app: AppState,
    pub db: Option<DatabaseConnection>,
}

/// Bootstraps config, optional `MySQL`, and starts the HTTP server.
///
/// # Errors
///
/// Returns [`RuntimeError`] when bootstrap, database, or HTTP serving fails.
pub async fn run(config_dir: std::path::PathBuf) -> Result<(), RuntimeError> {
    let state = bootstrap(&BootstrapOptions::new(config_dir))?;
    let db = connect_if_enabled(state.config()).await?;
    let runtime = RuntimeState { app: state, db };
    let settings = runtime.app.config().server.clone();
    let router = build_router(runtime);
    serve_bound_router(router, &settings).await?;
    Ok(())
}

fn build_router(runtime: RuntimeState) -> Router {
    let settings = runtime.app.config().server.clone();
    let router = Router::new()
        .route("/", get(home))
        .route(
            "/examples/new",
            get(example_new_form).post(example_new_submit),
        )
        .route("/health", get(health))
        .with_state(runtime);
    apply_default_middleware(router, &settings)
}

async fn home(State(state): State<RuntimeState>) -> Result<Html<String>, RuntimeError> {
    let app_name = state.app.config().app.name.clone();

    let template = match &state.db {
        Some(connection) => match Example::find().all(connection).await {
            Ok(examples) if examples.is_empty() => HomeTemplate {
                app_name: app_name.clone(),
                page_title: String::from("Home"),
                rows: vec![ExampleRow {
                    id: 0,
                    name: String::from("—"),
                    description: String::from(
                        "No rows yet. Use New example or run the CRUD integration test.",
                    ),
                }],
                status_message: None,
            },
            Ok(examples) => HomeTemplate {
                app_name: app_name.clone(),
                page_title: String::from("Home"),
                rows: examples
                    .into_iter()
                    .map(|example| ExampleRow {
                        id: example.id,
                        name: example.name,
                        description: example.description.unwrap_or_else(|| String::from("-")),
                    })
                    .collect(),
                status_message: None,
            },
            Err(error) => {
                HomeTemplate::load_error(app_name, format!("Failed to load examples: {error}"))
            }
        },
        None => HomeTemplate::empty_db_message(app_name),
    };

    Ok(into_html(template)?)
}

async fn example_new_form(
    State(state): State<RuntimeState>,
) -> Result<Html<String>, RuntimeError> {
    let app_name = state.app.config().app.name.clone();
    Ok(into_html(ExampleNewTemplate::blank(app_name))?)
}

async fn example_new_submit(
    State(state): State<RuntimeState>,
    axum::Form(input): axum::Form<ExampleFormInput>,
) -> Result<Response, RuntimeError> {
    let app_name = state.app.config().app.name.clone();
    let errors = validate_example_form(&input);
    if !errors.is_empty() {
        return Ok(into_html(ExampleNewTemplate::from_input(
            app_name, &input, errors,
        ))?
        .into_response());
    }

    let Some(connection) = state.db.as_ref() else {
        let mut errors = rustbridge_view::ValidationErrors::new();
        errors.add(
            "database",
            "Database is disabled. Enable APP_DATABASE__ENABLED and run migrations.",
        );
        return Ok(into_html(ExampleNewTemplate::from_input(
            app_name, &input, errors,
        ))?
        .into_response());
    };

    let description = {
        let trimmed = input.description.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    };

    ExampleService::create(connection, input.name.trim(), description)
        .await
        .map_err(RuntimeError::ExampleService)?;

    Ok(Redirect::to("/").into_response())
}

async fn health(State(state): State<RuntimeState>) -> impl IntoResponse {
    health_response(&state.app).into_response()
}

async fn connect_if_enabled(
    config: &AppConfig,
) -> Result<Option<DatabaseConnection>, RuntimeError> {
    if !config.database.enabled {
        tracing::info!("database disabled; home page will show setup instructions");
        return Ok(None);
    }

    let connection = connect(&config.database).await?;
    ping(&connection).await?;
    tracing::info!(
        database.host = %config.database.host,
        database.name = %config.database.name,
        "database connectivity verified"
    );
    Ok(Some(connection))
}

#[derive(Debug, thiserror::Error)]
pub enum RuntimeError {
    #[error(transparent)]
    Bootstrap(#[from] BootstrapError),

    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error(transparent)]
    Http(#[from] ServeError),

    #[error(transparent)]
    View(#[from] ViewError),

    #[error(transparent)]
    ExampleService(#[from] crate::services::ExampleServiceError),
}

impl IntoResponse for RuntimeError {
    fn into_response(self) -> Response {
        tracing::error!(error = %self, "request handling failed");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "An error occurred while processing your request.",
        )
            .into_response()
    }
}

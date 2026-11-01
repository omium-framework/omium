//! Examples resource (Inertia CRUD demo).

use axum::extract::{Path, State};
use axum::response::Response;
use axum::Extension;
use omium_core::presentation::RequestPresentation;
use omium_inertia::inertia_rs::{Inertia, InertiaForm, InertiaRequest};
use omium_view::ValidationErrors;

use crate::app::{RuntimeError, RuntimeState};
use crate::forms::{ExampleFormInput, normalized_description, validate_example_form};
use crate::http::{example_form_props, render_inertia};
use crate::middleware::Authenticated;
use crate::services::ExampleService;

/// `GET /examples`
pub async fn index(
    State(state): State<RuntimeState>,
    request: InertiaRequest,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(Authenticated(authenticated)): Extension<Authenticated>,
) -> Result<Response, RuntimeError> {
    let (rows, db_enabled, hint) = match &state.db {
        Some(connection) => match ExampleService::list(connection).await {
            Ok(models) if models.is_empty() => (
                serde_json::json!([{
                    "id": 0,
                    "name": "No records yet",
                    "description": "Create one with the button above.",
                }]),
                true,
                Option::<String>::None,
            ),
            Ok(models) => {
                let rows = models
                    .into_iter()
                    .map(|row| {
                        serde_json::json!({
                            "id": row.id,
                            "name": row.name,
                            "description": row.description.unwrap_or_else(|| String::from("—")),
                        })
                    })
                    .collect::<Vec<_>>();
                (serde_json::Value::Array(rows), true, None)
            }
            Err(error) => (
                serde_json::json!([]),
                true,
                Some(format!("Failed to load examples: {error}")),
            ),
        },
        None => (
            serde_json::json!([
                { "id": 0, "name": "Database disabled", "description": "Run ./omnisan install and ensure MySQL is up, then ./omnisan migrate" },
            ]),
            false,
            Some(String::from(
                "Enable MySQL: ./omnisan install (creates .env), docker compose up -d, ./omnisan migrate.",
            )),
        ),
    };

    render_inertia(
        &state,
        &request,
        &presentation,
        authenticated,
        "Examples",
        "Examples",
        move |base| {
            let mut page = base.value("rows", rows).value("dbEnabled", db_enabled);
            if let Some(message) = hint {
                page = page.value("statusMessage", message);
            }
            page
        },
    )
}

/// `GET /examples/new`
pub async fn create_form(
    State(state): State<RuntimeState>,
    request: InertiaRequest,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(Authenticated(authenticated)): Extension<Authenticated>,
) -> Result<Response, RuntimeError> {
    render_inertia(
        &state,
        &request,
        &presentation,
        authenticated,
        "ExampleForm",
        "New example",
        |base| {
            example_form_props(
                base,
                "Create example",
                "Add a row to the examples table.",
                "/examples/new",
                String::new(),
                String::new(),
                ValidationErrors::new(),
            )
        },
    )
}

/// `POST /examples/new`
pub async fn create(
    State(state): State<RuntimeState>,
    request: InertiaRequest,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(Authenticated(authenticated)): Extension<Authenticated>,
    InertiaForm(input): InertiaForm<ExampleFormInput>,
) -> Result<Response, RuntimeError> {
    let errors = validate_example_form(&input);
    if !errors.is_empty() {
        return render_inertia(
            &state,
            &request,
            &presentation,
        authenticated,
            "ExampleForm",
            "New example",
            |base| {
                example_form_props(
                    base,
                    "Create example",
                    "Add a row to the examples table.",
                    "/examples/new",
                    input.name.clone(),
                    input.description.clone(),
                    errors,
                )
            },
        );
    }

    let Some(connection) = state.db.as_ref() else {
        return render_inertia(
            &state,
            &request,
            &presentation,
        authenticated,
            "ExampleForm",
            "New example",
            |base| {
                example_form_props(
                    base,
                    "Create example",
                    "Add a row to the examples table.",
                    "/examples/new",
                    input.name.clone(),
                    input.description.clone(),
                    ValidationErrors::new(),
                )
                .value(
                    "statusMessage",
                    "Database is not available. Run ./omnisan install && ./omnisan migrate.",
                )
            },
        );
    };

    ExampleService::create(
        connection,
        input.name.trim(),
        normalized_description(&input),
    )
    .await?;

    request
        .redirect(Inertia::redirect("/examples"))
        .map_err(RuntimeError::Inertia)
}

/// `GET /examples/{id}/edit`
pub async fn edit_form(
    State(state): State<RuntimeState>,
    Path(id): Path<u32>,
    request: InertiaRequest,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(Authenticated(authenticated)): Extension<Authenticated>,
) -> Result<Response, RuntimeError> {
    let Some(connection) = state.db.as_ref() else {
        return render_inertia(
            &state,
            &request,
            &presentation,
        authenticated,
            "ExampleForm",
            "Edit example",
            |base| {
                base.value(
                    "statusMessage",
                    "Database is not available. Run ./omnisan install && ./omnisan migrate.",
                )
            },
        );
    };

    let Some(example) = ExampleService::find_by_id(connection, id).await? else {
        return render_inertia(
            &state,
            &request,
            &presentation,
        authenticated,
            "Examples",
            "Examples",
            |base| base.value("statusMessage", format!("Example #{id} was not found.")),
        );
    };

    let form_action = format!("/examples/{id}/edit");
    render_inertia(
        &state,
        &request,
        &presentation,
        authenticated,
        "ExampleForm",
        "Edit example",
        |base| {
            example_form_props(
                base,
                format!("Edit example #{id}"),
                "Update name and description.",
                form_action,
                example.name,
                example.description.unwrap_or_default(),
                ValidationErrors::new(),
            )
        },
    )
}

/// `POST /examples/{id}/edit`
pub async fn update(
    State(state): State<RuntimeState>,
    Path(id): Path<u32>,
    request: InertiaRequest,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(Authenticated(authenticated)): Extension<Authenticated>,
    InertiaForm(input): InertiaForm<ExampleFormInput>,
) -> Result<Response, RuntimeError> {
    let form_action = format!("/examples/{id}/edit");
    let errors = validate_example_form(&input);
    if !errors.is_empty() {
        return render_inertia(
            &state,
            &request,
            &presentation,
        authenticated,
            "ExampleForm",
            "Edit example",
            |base| {
                example_form_props(
                    base,
                    format!("Edit example #{id}"),
                    "Update name and description.",
                    form_action,
                    input.name.clone(),
                    input.description.clone(),
                    errors,
                )
            },
        );
    }

    let Some(connection) = state.db.as_ref() else {
        return render_inertia(
            &state,
            &request,
            &presentation,
        authenticated,
            "ExampleForm",
            "Edit example",
            |base| {
                example_form_props(
                    base,
                    format!("Edit example #{id}"),
                    "Update name and description.",
                    form_action,
                    input.name.clone(),
                    input.description.clone(),
                    ValidationErrors::new(),
                )
                .value(
                    "statusMessage",
                    "Database is not available. Run ./omnisan install && ./omnisan migrate.",
                )
            },
        );
    };

    match ExampleService::update(
        connection,
        id,
        input.name.trim(),
        normalized_description(&input),
    )
    .await
    {
        Ok(_) => request
            .redirect(Inertia::redirect("/examples"))
            .map_err(RuntimeError::Inertia),
        Err(crate::services::ExampleServiceError::Database(
            sea_orm::DbErr::RecordNotFound(_),
        )) => render_inertia(
            &state,
            &request,
            &presentation,
        authenticated,
            "Examples",
            "Examples",
            |base| base.value("statusMessage", format!("Example #{id} was not found.")),
        ),
        Err(error) => Err(RuntimeError::ExampleService(error)),
    }
}

/// `DELETE /examples/{id}`
pub async fn destroy(
    State(state): State<RuntimeState>,
    Path(id): Path<u32>,
    request: InertiaRequest,
    Extension(presentation): Extension<RequestPresentation>,
    Extension(Authenticated(authenticated)): Extension<Authenticated>,
) -> Result<Response, RuntimeError> {
    let Some(connection) = state.db.as_ref() else {
        return render_inertia(
            &state,
            &request,
            &presentation,
        authenticated,
            "Examples",
            "Examples",
            |base| {
                base.value(
                    "statusMessage",
                    "Database is not available. Run ./omnisan install && ./omnisan migrate.",
                )
            },
        );
    };

    match ExampleService::delete(connection, id).await {
        Ok(()) => request
            .redirect(Inertia::redirect("/examples"))
            .map_err(RuntimeError::Inertia),
        Err(crate::services::ExampleServiceError::Database(
            sea_orm::DbErr::RecordNotFound(_),
        )) => render_inertia(
            &state,
            &request,
            &presentation,
        authenticated,
            "Examples",
            "Examples",
            |base| base.value("statusMessage", format!("Example #{id} was not found.")),
        ),
        Err(error) => Err(RuntimeError::ExampleService(error)),
    }
}

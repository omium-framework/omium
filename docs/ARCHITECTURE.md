# Architecture

## 1. High-level architecture

The framework is a modular Rust workspace. Application business logic is independent of frontend mode and HTTP response format.

```text
Generated Application
├── routes
├── controllers / handlers
├── services / use-cases
├── models / DTOs
├── config
├── database migrations
├── tests
└── frontend (optional)
      ├── Standard: Askama templates
      ├── Vue: Vue 3 + Inertia + Vite
      ├── React: React + Inertia + Vite
      └── API: no bundled frontend

Framework Workspace
├── core
├── http (Axum + Tower)
├── db (SeaORM)
├── view (Askama)
├── inertia (selected Inertia v3 adapter)
├── config
├── cli (Clap)
├── testing
└── auth (only after approved design)
```

## 2. Request lifecycle

1. Process starts.
2. Load typed configuration.
3. Validate required settings and secrets.
4. Initialize tracing.
5. Create Tokio runtime through the application entry point.
6. Connect database pool and validate connectivity.
7. Build shared application state.
8. Compose Axum router and Tower middleware.
9. Accept HTTP requests.
10. Middleware applies request IDs, tracing, timeouts, body limits, and other approved policies.
11. Route selects a handler.
12. Typed extractors parse path/query/body/state.
13. Handler calls application service.
14. Service executes business rules and repository operations.
15. Handler maps result to one transport:
    - Standard: Askama HTML response.
    - Vue/React: Inertia page response.
    - API: JSON response.
16. Error boundary maps typed errors to safe HTTP responses.
17. Graceful shutdown drains requests and closes resources.

## 3. Dependency direction

- Core has no dependency on Axum, SeaORM, Inertia, Askama, Vue, or React.
- HTTP adapter depends on core and Axum/Tower.
- DB adapter depends on core and SeaORM.
- View adapter depends on core and Askama.
- Inertia adapter depends on core and the approved Inertia server crate.
- CLI depends on framework crates; framework crates do not depend on CLI.
- Application services do not depend on frontend-specific response types.
- Avoid circular crate dependencies.

## 4. Application folder convention

The following is the target generated application layout. It is a framework convention, not a requirement to mirror PHP syntax.

```text
my-app/
├── Cargo.toml
├── Cargo.lock
├── rustbridge.toml              # provisional filename; owner approval before public API freeze
├── .env.example
├── config/
│   ├── app.toml
│   ├── database.toml
│   └── logging.toml
├── src/
│   ├── main.rs
│   ├── app.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   ├── web.rs
│   │   └── api.rs
│   ├── controllers/
│   ├── services/
│   ├── models/
│   ├── dto/
│   ├── middleware/
│   └── errors/
├── migration/
├── resources/
│   ├── views/                   # Standard only
│   └── js/                      # Vue/React only
│       ├── app.ts
│       ├── Pages/
│       ├── Components/
│       └── Layouts/
├── public/
├── tests/
└── rules/                       # optional project-level agent rules copied by generator
```

The owner must approve public folder naming before the first stable template release. Until then, use this layout for prototypes and keep generators easy to change.

## 5. Configuration model

- Files: TOML under `config/`.
- Environment overrides: explicit documented prefix, provisionally `APP_`.
- Secrets: environment or secret manager only.
- Precedence: built-in defaults < config files < environment overrides.
- Command-line overrides may be added for one-off CLI operations, not silently to application runtime.
- Parse and validate configuration once at startup.
- Invalid configuration must stop startup with a clear error.
- Never print secret values in config debug output.
- Environment key naming and nested mapping must be tested and documented before public release.

## 6. Dependency injection

- Use typed application state and Axum extractors.
- Avoid global mutable service locator patterns.
- Prefer constructing services at bootstrap and injecting them through application state.
- Do not build a large reflection-based container.
- If a container abstraction is later proposed, request owner approval and compare explicit state vs container ergonomics.

## 7. Response boundary

The same service can serve multiple transports:

```text
ProductService::list()
       |
       +--> Standard handler -> Askama HTML
       |
       +--> Inertia handler -> Page component + props
       |
       +--> API handler -> JSON DTO
```

Do not put response-specific behavior in the service layer. Do not serialize persistence entities by default.

## 8. Error model

- Domain errors are typed.
- HTTP adapter maps known errors to status codes and safe response bodies.
- Internal diagnostics go to structured logs with correlation IDs.
- Unknown errors return generic 500 responses.
- Validation errors have a consistent DTO shape and can be mapped to Inertia error bags.
- Error mapping must be covered by tests.

## 9. Graceful shutdown

- Handle termination signals.
- Stop accepting new requests.
- Allow in-flight requests a bounded drain period.
- Close database pools and background workers.
- Do not introduce queue workers in initial core milestone.

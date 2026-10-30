# Architecture rules

## Layer boundaries

1. `rustbridge-core`: framework-independent primitives, errors, config abstractions, application state contracts.
2. `rustbridge-http`: Axum integration, routing helpers, extractors, response abstractions, middleware integration.
3. `rustbridge-db`: SeaORM integration, connection configuration, repository conventions, transactions.
4. `rustbridge-inertia`: Inertia protocol integration and frontend asset manifest integration.
5. `rustbridge-view`: Standard server-rendered view integration using Askama.
6. `rustbridge-cli`: project creation, commands, generators, migrations, dev server orchestration.
7. `rustbridge-auth`: authentication/session/CSRF only after the security design is approved.
8. `rustbridge-testing`: test helpers and assertions.
9. Application code lives in the generated app, not inside framework crates.

Names are provisional package identifiers. Before publishing crates or selecting a public namespace, obtain owner approval. Internal workspace crate names may use a temporary consistent prefix.

## Dependency direction

- Core must not depend on Axum, frontend libraries, or a concrete database.
- HTTP may depend on core and Axum/Tower.
- DB adapter may depend on core and SeaORM.
- Inertia adapter may depend on core and the selected Inertia server adapter.
- View adapter may depend on core and Askama.
- CLI may orchestrate framework crates but framework crates must not depend on CLI.
- Application business logic must not depend on frontend mode.
- Avoid circular dependencies.

## Application architecture

- Use explicit application state and dependency injection through typed state/extractors.
- Prefer constructor-based dependencies and typed services.
- Do not implement a global mutable service locator.
- Routes are composition roots that connect transport handlers to application services.
- Keep business rules in services/domain modules, not in HTTP handlers.
- Handlers validate transport input, call services, and map results to responses.
- Use DTOs for request/response boundaries; do not serialize database entities blindly.

## Configuration

- Support `config/` files and environment variable overrides.
- Configuration must be typed, validated at startup, and fail fast with actionable errors.
- Never silently fall back to insecure values.
- Keep secrets out of committed config files.
- Environment precedence and config file formats must follow `docs/ARCHITECTURE.md`.

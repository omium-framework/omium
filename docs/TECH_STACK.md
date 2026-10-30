# Technology stack and integration contracts

This document defines the selected baseline and how each component is integrated. Versions must be verified against current crate releases during implementation and pinned through Cargo.lock. Do not invent version numbers.

## Core runtime

### Rust + Cargo
- Stable Rust, Edition 2024.
- Use a Cargo workspace for framework crates.
- Centralize shared dependency declarations in workspace `Cargo.toml`.
- Commit `Cargo.lock` for applications and reproducible examples. Library crates follow standard Cargo publishing conventions.
- Set workspace lints, including `unsafe_code = "forbid"`.

### Tokio
- One Tokio multi-thread runtime per application process, created by the application bootstrap.
- Framework libraries must not create nested runtimes.
- Use Tokio-compatible async I/O and database operations.
- Runtime configuration (worker count, blocking pool limits, shutdown behavior) must have safe defaults and documented overrides.

### Axum + Tower
- Axum is the HTTP engine.
- Tower/tower-http provide middleware composition, request tracing, timeout, compression, and request limits.
- Framework routing helpers wrap Axum rather than forking it.
- Keep Axum types behind framework abstractions where this does not create excessive complexity.
- Handler functions remain ordinary Rust async functions and use typed extractors.

## Database

### SeaORM
- SeaORM is the default ORM.
- Use SeaORM entities, ActiveModel, relations, transactions, connection pools, and query APIs.
- Framework model conventions must be documented and generated code must remain valid SeaORM code.
- Do not build a second custom ORM.
- Do not hide important database behavior behind magic.
- Use DTOs for HTTP/Inertia serialization; do not expose ActiveModel or persistence entities as public API responses by default.

### SeaORM Migration
- Use the official SeaORM migration ecosystem for schema changes.
- Generate migration files through the framework CLI.
- Migrations must be deterministic, ordered, reversible where feasible, and tested against the supported MySQL version.
- Production migration execution must be an explicit deployment command initially; never run schema migrations automatically on every server boot.
- Destructive migrations require clear warnings and owner confirmation in generated workflows.

### SQLx
- SQLx is permitted as a specialized escape hatch for raw, parameterized SQL or query patterns not well expressed in SeaORM.
- SQLx is not the default repository API.
- Use bind parameters. Never concatenate user-controlled values into SQL.
- Keep raw SQL in named repository/query modules and test it.

### MySQL
- Initial first-class database target.
- Test against a supported MySQL server version selected during implementation and document it.
- Use connection pooling and startup connectivity checks.
- Do not silently add SQLite/PostgreSQL support without approval.

## Serialization and errors

- Serde for typed serialization/deserialization.
- serde_json for JSON APIs and Inertia page props.
- thiserror for typed errors in framework libraries.
- anyhow may be used at top-level application startup/CLI boundaries with context.
- Public error responses must be stable, sanitized, and must not reveal internal details.

## Views and frontend

### Standard
- Askama for server-side HTML templates.
- Keep templates typed and compile-time checked where supported.
- Provide a root layout and error-page conventions.
- Do not bundle a custom JavaScript framework.

### Vue mode
- Vue 3, TypeScript, official Inertia Vue client, Vite.
- Generate a working frontend package and typed page-prop examples.
- Pages live under `resources/js/Pages`.
- Components and layouts live under `resources/js/Components` and `resources/js/Layouts`.

### React mode
- React, TypeScript, official Inertia React client, Vite.
- Pages live under `resources/js/Pages`.
- Components and layouts live under `resources/js/Components` and `resources/js/Layouts`.

### Inertia
- Implement official Inertia v3 server protocol through a selected Rust adapter, only after D-013 approval.
- Keep a framework-owned adapter interface so the selected third-party crate is not leaked everywhere.
- Do not implement a custom incompatible protocol.
- Test protocol headers, initial HTML response, JSON page object, asset versioning, redirects, validation errors, shared props, partial reloads, and each advanced feature before documenting it as supported.
- SSR is not part of the initial release.

### Vite
- Development: generated app may use Vite dev server and hot-file detection.
- Production: read Vite manifest and emit fingerprinted asset URLs.
- Fail with a helpful startup/build error when the manifest is missing in production.
- Do not require Node.js for API-only projects.

## CLI and configuration

- Clap derive for CLI parsing.
- TOML configuration files and environment variable overrides.
- Configuration structs must be typed and validated at startup.
- Do not implement a custom configuration language.

## Observability

- tracing and tracing-subscriber.
- Structured logs with request correlation IDs.
- Redact secrets and personal data.
- No telemetry or external reporting by default.

## Dependency integration checklist

Before adding or upgrading any crate:
1. Confirm current stable release and MSRV.
2. Review license and transitive licenses.
3. Review maintenance activity and issue status.
4. Review security advisories.
5. Confirm Tokio/Axum compatibility.
6. Review default features and disable unnecessary features.
7. Add a minimal integration test.
8. Update dependency inventory and decision docs.
9. Ask owner approval if it is a core dependency or changes an approved decision.

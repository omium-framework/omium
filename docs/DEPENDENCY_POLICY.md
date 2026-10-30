# Dependency and supply-chain policy

## General

- Prefer mature, maintained, well-documented open-source crates.
- Do not reinvent existing infrastructure without a written technical reason and owner approval.
- Keep dependencies minimal and feature sets explicit.
- Use workspace dependency declarations to keep versions consistent.
- Review crate source/docs, license, maintenance, MSRV, open issues, security advisories, and transitive dependencies.
- Avoid abandoned or experimental dependencies in production-critical paths unless owner explicitly approves the risk.
- Do not pin guessed versions in blueprint examples. During implementation verify current releases and commit Cargo.lock for applications/examples.

## Core baseline inventory

| Area | Selected technology | Notes |
|---|---|---|
| Language/build | Rust stable + Cargo | Edition 2024 |
| Async runtime | Tokio | One runtime per process |
| HTTP | Axum | 0.8 line baseline |
| Middleware | Tower / tower-http | Layer/service composition |
| ORM | SeaORM | Default ORM |
| Migrations | SeaORM Migration | Explicit deployment step |
| Raw SQL escape hatch | SQLx | Parameterized SQL only |
| Serialization | Serde / serde_json | Typed boundaries |
| Errors | thiserror | Library/domain errors |
| CLI | Clap | Binary name awaits approval |
| Standard templates | Askama | Server-rendered HTML |
| Frontend build | Vite | Vue/React only |
| Frontend | Vue 3 / React | TypeScript |
| Inertia | Rust Inertia v3 adapter | Selection awaits spike + owner approval |
| Observability | tracing | Structured logs |

## Prohibited without approval

- Custom cryptography.
- Unsafe Rust or FFI.
- Unreviewed Git dependencies in production.
- Unmaintained crates for auth/session/security.
- Telemetry SDKs or analytics.
- Runtime code downloading/executing dependencies.
- Multiple competing ORMs in the standard application API.

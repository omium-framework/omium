# Architecture Decision Register

Status meanings:
- **APPROVED**: owner has explicitly chosen this direction.
- **PROPOSED**: recommended, but not approved by owner.
- **OPEN**: must be decided with owner approval.
- **DEFERRED**: intentionally postponed; do not implement early.

## Approved by owner

### D-001 — Real Rust only
**Status: APPROVED**

All framework application code is real Rust. No PHP-like DSL, transpiler, or interpreter. Rust compiler and Cargo remain authoritative.

### D-002 — Convention over configuration
**Status: APPROVED**

The framework follows Laravel's convention-over-configuration philosophy with sensible defaults and explicit configuration.

### D-003 — Prefer mature open-source infrastructure
**Status: APPROVED**

Do not reinvent ORM, HTTP runtime, serialization, CLI parsing, or template engines when a suitable maintained crate exists. Integrate through framework-owned boundaries where practical.

### D-004 — Frontend choice at project creation
**Status: APPROVED**

The project generator must let the developer choose:
- `standard`
- `vue`
- `react`
- `api`

Vue and React use Inertia as the preferred full-stack interaction model. API mode is frontend-independent.

### D-005 — Inertia as a design reference
**Status: APPROVED**

Use Inertia's server-driven SPA model as the reference for Vue/React integration. Do not build a separate client-side router or duplicate API layer for an Inertia application. Implement the official protocol, not an incompatible imitation.

### D-016 — Phase 0 implementation start with proposed baseline
**Status: APPROVED**

Owner approved starting Phase 0 using the PROPOSED baseline stack (D-006–D-012, D-014–D-015) and provisional internal crate/binary prefix `rustbridge-*` until public naming (O-001) is decided.

### D-017 — Development CLI binary `omnisan`
**Status: APPROVED**

Owner approved **`omnisan`** as the Laravel Artisan-style developer CLI binary name alongside the provisional `rustbridge` binary (same entry point, two `[[bin]]` targets). Public packaging may still reconcile names under O-001 before crates.io release.

### D-018 — crates.io CLI crate name `omnisan`
**Status: APPROVED**

Owner registered and published the placeholder crate **[`omnisan`](https://crates.io/crates/omnisan)** on crates.io (MIT). Target end-user install command:

```bash
cargo install omnisan
```

The monorepo crate directory remains **`crates/omium-cli`**; the published library crate is **`omium-cli`** on crates.io. The **`omnisan`** crate ships the `omnisan` binary and depends on **`omium-cli`**. Placeholder **0.1.0** must be superseded by **0.1.1+** with a working binary once **`omium-*`** libraries are published.

### D-019 — crates.io library namespace `omium-*`
**Status: APPROVED**

Owner approved published library crate names:

| crates.io package | Monorepo path | Rust library name |
| --- | --- | --- |
| `omium-core` | `crates/omium-core` | `omium_core` |
| `omium-http` | `crates/omium-http` | `omium_http` |
| `omium-db` | `crates/omium-db` | `omium_db` |
| `omium-view` | `crates/omium-view` | `omium_view` |
| `omium-inertia` | `crates/omium-inertia` | `omium_inertia` |
| `omium-cli` | `crates/omium-cli` | `omium_cli` |

Generated apps use `omium-* = {{ version = "…" }}` when no Framework clone is found on disk. Monorepo directories and Rust crate names use the same **`omium-*` / `omium_*`** naming (owner rename, 2025-10).

These are the implementation baseline recommended by this blueprint. A change to a core item requires owner approval.

### D-006 — Rust toolchain
**Status: PROPOSED BASELINE**

- Stable Rust.
- Rust 2024 edition.
- Cargo workspaces.
- MSRV: set only after dependency compatibility verification; initial target should be the lowest stable version supported by all selected dependencies. Do not hardcode an unverified number.

### D-007 — HTTP and async
**Status: PROPOSED BASELINE**

- Tokio for async runtime.
- Axum 0.8 line for HTTP routing and extractors.
- Tower and tower-http for middleware/services.
- Hyper remains an implementation detail through Axum unless a specific need is approved.

### D-008 — ORM and database
**Status: PROPOSED BASELINE**

- SeaORM as the default ORM.
- SeaORM Migration for schema migrations.
- SQLx may be used for specialized, performance-critical SQL where SeaORM cannot express the query cleanly; it is not the default application data API.
- Initial database: MySQL, reflecting the owner's preference and familiarity.
- Do not add PostgreSQL or SQLite as first-class supported production databases without approval.

### D-009 — Serialization and errors
**Status: PROPOSED BASELINE**

- Serde + serde_json for serialization.
- thiserror for library/domain error types.
- anyhow only at application/binary boundaries, not in public library APIs.

### D-010 — CLI
**Status: PROPOSED BASELINE**

- Clap derive for command parsing.
- CLI binary/public command name is OPEN and requires owner approval. Use an internal placeholder in prototypes only.

### D-011 — Standard view rendering
**Status: PROPOSED BASELINE**

- Askama for server-rendered templates.
- Standard mode is server-rendered HTML, not a custom frontend framework.
- Template selection and error handling must be integrated behind a framework-owned view abstraction.

### D-012 — Frontend build
**Status: PROPOSED BASELINE**

- Vite for Vue/React development and production assets.
- Vue 3 + TypeScript + official Inertia Vue client.
- React + TypeScript + official Inertia React client.
- Tailwind CSS is the default styling option in generated templates, but the framework must not require it at runtime.
- Node.js is needed only for Vue/React frontend asset workflows, not API-only Rust builds.

### D-013 — Inertia Rust adapter
**Status: APPROVED (Phase 7 bridge — `inertia_rs` 0.4)**

Owner direction (Phase 7): integrate the Inertia bridge into the application base (`examples/inertia-vue-app`) using `omium-inertia` + `inertia_rs`. Re-evaluate official **`inertia-omega`** when MSRV policy allows Rust **1.99+** and a pinned release strategy exists.

Production Vue/React apps must use the framework crate boundary (`omium-inertia`), not direct adapter types in application handlers where avoidable.

Spike archive: `docs/INERTIA_SPIKE_REPORT.md`, `examples/inertia-spike/`.

### D-014 — Observability
**Status: PROPOSED BASELINE**

- tracing and tracing-subscriber for structured logs.
- tower-http trace middleware where appropriate.
- No telemetry or outbound reporting by default.

### D-015 — Configuration
**Status: PROPOSED BASELINE**

- TOML configuration files in `config/`.
- Environment variable overrides.
- Typed deserialization and startup validation.
- Secrets are environment/secret-manager values, never committed config values.
- Exact precedence rules are specified in `ARCHITECTURE.md`.

### D-020 — Authentication and sessions (O-003)
**Status: APPROVED (2026-11-01)**

Owner approved **`docs/AUTH_DESIGN_PROPOSAL.md`** with:

- **C1** `omium-auth`, Argon2id, signed session cookie.
- **C2** Session drivers v1: **database + file**.
- **C3** CSRF synchronizer token (web/Inertia).
- **C4** API: **personal_access_tokens** (not JWT).
- **C5** Email verification + password reset; local dev via **Mailpit** (SMTP).
- **C6** Sequential phases **A1–A7**.

Phase **A1–A2** implemented in `crates/omium-auth` and `omium-http::start_session`.

## Deferred decisions

- Authentication/session/CSRF: see **D-020** (PROPOSED) and `docs/AUTH_DESIGN_PROPOSAL.md`.
- Queue backend and queue semantics: DEFERRED.
- Cache backend: DEFERRED.
- Scheduler implementation: DEFERRED.
- Email/notification integrations: DEFERRED.
- SSR architecture: DEFERRED.
- Plugin/package registry: DEFERRED.
- Public name, crate namespace, CLI name, license: OPEN.
- Automatic Laravel project conversion: later product milestone; do not promise lossless conversion.

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

## Technical baseline selected for the blueprint

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
**Status: OPEN — OWNER APPROVAL REQUIRED AFTER SPIKE**

Evaluate `inertia-omega` and `inertia_rs` against the official Inertia v3 protocol. Do not select solely by popularity or feature claims.
Required spike report:
- protocol coverage matrix;
- initial HTML and JSON responses;
- asset versioning;
- redirects and external redirects;
- validation errors and error bags;
- shared props;
- partial reloads;
- deferred/merge/once props;
- history behavior;
- Vite manifest integration;
- SSR status;
- license, maintenance, test quality, MSRV;
- adapter API and ability to wrap it behind our own trait.
Recommendation must be presented to owner. No production integration before approval.

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

## Deferred decisions

- Authentication/session/CSRF crate and architecture: OPEN; approval required.
- Queue backend and queue semantics: DEFERRED.
- Cache backend: DEFERRED.
- Scheduler implementation: DEFERRED.
- Email/notification integrations: DEFERRED.
- SSR architecture: DEFERRED.
- Plugin/package registry: DEFERRED.
- Public name, crate namespace, CLI name, license: OPEN.
- Automatic Laravel project conversion: later product milestone; do not promise lossless conversion.

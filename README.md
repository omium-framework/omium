# RustBridge — Engineering Blueprint

> **Working codename only.** The public product name, CLI binary name, crate namespace, logo, and domain are NOT approved. Do not publish or rename packages without the owner's explicit approval.

This repository is the authoritative product, architecture, implementation, and AI-agent instruction package for a Rust-native web application framework aimed first at PHP/Laravel developers.

## Mission

Make modern Rust backend and full-stack development approachable to experienced PHP developers without hiding or replacing the Rust language. Applications written with this framework are real Rust applications, compiled by Cargo, using Rust's type system, ownership model, borrowing rules, and compiler checks.

The framework must provide a Laravel-inspired developer experience:
- Convention over configuration.
- Sensible defaults and explicit configuration files.
- A predictable application structure.
- A cohesive CLI and code generators.
- Integrated database, migrations, validation, authentication, queues, cache, and testing.
- Full-stack options: Standard server-rendered pages, Vue + Inertia, React + Inertia, or API-only.

## Non-negotiable operating rules

1. Read `AGENTS.md` and every applicable file under `rules/` before making changes.
2. Treat `docs/DECISIONS.md` as the approved decision register.
3. Do not make product, architecture, dependency, security, public API, licensing, or scope decisions that are not already approved.
4. If a decision is not explicitly settled, stop at that decision, explain options and trade-offs in Turkish, and request owner approval. Do not silently choose a default.
5. Never claim a feature is complete unless implementation, tests, documentation, and acceptance criteria are complete.
6. After each completed task, teach the owner in Turkish using a PHP/Laravel comparison, then explain the Rust concepts used.
7. The owner is an experienced PHP/Laravel developer but a Rust beginner. Do not explain basic web development unnecessarily; explain Rust-specific concepts clearly.
8. Do not introduce unsafe Rust. `unsafe_code = "forbid"` is the default.
9. Never weaken security or tests to make a build pass.
10. Keep the framework frontend-independent at its core.

## Start here

1. `AGENTS.md`
2. `rules/00-global.md`
3. `docs/PRODUCT_VISION.md`
4. `docs/DECISIONS.md`
5. `docs/ARCHITECTURE.md`
6. `docs/IMPLEMENTATION_ROADMAP.md`
7. `docs/ACCEPTANCE_CRITERIA.md`

## Document map

- `docs/PRODUCT_VISION.md` — audience, positioning, product principles.
- `docs/DECISIONS.md` — approved choices and approval-required open decisions.
- `docs/TECH_STACK.md` — technology selections and integration contracts.
- `docs/ARCHITECTURE.md` — system boundaries, request lifecycle, application structure.
- `docs/FRONTEND_MODES.md` — Standard, Vue + Inertia, React + Inertia, API-only.
- `docs/DATABASE_AND_ORM.md` — SeaORM conventions, migrations, SQLx escape hatch.
- `docs/SECURITY.md` — security baseline and threat model.
- `docs/CLI_AND_GENERATORS.md` — CLI commands and generator contracts.
- `docs/TESTING_AND_QUALITY.md` — test strategy and quality gates.
- `docs/IMPLEMENTATION_ROADMAP.md` — staged delivery plan.
- `docs/ACCEPTANCE_CRITERIA.md` — definition of done and release gates.
- `docs/OWNER_LEARNING_PROTOCOL.md` — required PHP-to-Rust teaching format.
- `docs/DEPENDENCY_POLICY.md` — dependency selection, updates, licenses.
- `docs/DEPENDENCY_INVENTORY.md` — workspace dependency inventory by phase.
- `docs/OPEN_DECISIONS.md` — decisions that must be escalated.
- `rules/` — canonical AI coding rules.
- `.cursor/rules/` and `.github/copilot-instructions.md` — entry points for common coding agents.

## Important scope boundary

Implementation is incremental per `docs/IMPLEMENTATION_ROADMAP.md`. Phase 0 provides a Cargo workspace skeleton; HTTP, database, and frontend integrations arrive in later phases.

## Development (Phase 0)

Workspace layout:

```text
crates/
├── rustbridge-core
├── rustbridge-http
├── rustbridge-db
├── rustbridge-view
├── rustbridge-inertia
├── rustbridge-cli      # provisional binary name: rustbridge
└── rustbridge-testing
```

Commands:

```bash
cargo build --workspace
cargo test --workspace
cargo run -p rustbridge-cli
cargo run -p sample-app
```

Sample app config lives in `examples/sample-app/config/`. Environment overrides use the `APP_` prefix (for example `APP_NAME`, `APP_DEBUG`, `APP_LOGGING__LEVEL`, `APP_SERVER__PORT`). Database passwords must be set with `APP_DATABASE__PASSWORD`, not TOML files.

Run the sample HTTP server:

```bash
cargo run -p sample-app
curl -s http://127.0.0.1:8080/          # HTML home (Askama + layout)
curl -s http://127.0.0.1:8080/examples/new  # form + validation demo
curl -s http://127.0.0.1:8080/health
```

From `examples/sample-app`, the CLI can bootstrap the same config and serve the framework default router (health only):

```bash
cd examples/sample-app
cargo run -p rustbridge-cli -- serve --config-dir config
```

For the demo home page with database listing, always use `cargo run -p sample-app`.

Default application name is **Omium** (override with `APP_NAME` or `config/app.toml`).

MySQL (Phase 3): see `docs/LOCAL_MYSQL.md` and `docker compose up -d` (MySQL on `3306`, phpMyAdmin on `http://127.0.0.1:8081`), then:

```bash
export APP_DATABASE__ENABLED=true APP_DATABASE__PASSWORD=omium
cargo run -p sample-app-migration
```

See `CONTRIBUTING.md` and `docs/DEPENDENCY_INVENTORY.md`.

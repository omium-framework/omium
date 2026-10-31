# Implementation roadmap

Work in small vertical slices. Each phase has an exit gate. Do not start the next phase automatically.

## Phase 0 — Repository and decision foundation

Deliver:
- Cargo workspace skeleton.
- Workspace lint policy.
- `README`, contribution notes, architecture docs.
- CI for fmt, clippy, tests.
- Dependency inventory.
- Minimal runnable binary.

Exit criteria:
- clean workspace build;
- CI green;
- no unapproved dependencies;
- owner reviews repository structure.

## Phase 1 — Core application bootstrap and configuration

Deliver:
- typed TOML config;
- environment overrides;
- startup validation;
- tracing initialization;
- application state;
- graceful shutdown skeleton.

Exit criteria:
- config precedence tests;
- invalid config fails with useful error;
- secrets are redacted;
- sample app starts.

## Phase 2 — HTTP foundation

Deliver:
- Axum integration;
- route registration convention;
- typed state/extractors;
- Tower middleware baseline;
- error boundary;
- health endpoint.

Exit criteria:
- route integration tests;
- request ID and structured logs;
- body/time limits;
- no blocking work on request path.

## Phase 3 — Database and ORM

Deliver:
- SeaORM MySQL connection;
- entity conventions;
- transaction helpers;
- SeaORM Migration integration;
- CLI migration commands.

Exit criteria:
- real MySQL integration test;
- fresh database migration test;
- upgrade migration test;
- CRUD example and PHP/Laravel learning note.

## Phase 4 — Application conventions and CLI

Deliver:
- project generator;
- controller/service/model/migration generators;
- route:list;
- serve;
- doctor;
- generated app structure.

Exit criteria:
- generated sample app compiles and tests;
- generated code is readable and documented;
- no file overwrite without confirmation.

## Phase 5 — Standard frontend

Deliver:
- Askama integration;
- root layout;
- typed page context;
- form and error rendering example.

Exit criteria:
- HTML rendering tests;
- escaping tests;
- no Node requirement.

## Phase 6 — Inertia adapter spike and approval

Deliver:
- compare `inertia-omega` and `inertia_rs`;
- protocol support matrix;
- minimal Axum proof of concept;
- Vue and React client compatibility check;
- license, maintenance, MSRV and test review;
- owner decision.

Exit criteria:
- explicit owner approval recorded in `DECISIONS.md`.
- If neither adapter is acceptable, stop and ask whether to implement an adapter from official protocol specification. Do not independently choose.

## Phase 7 — Vue + Inertia and React + Inertia

Deliver:
- Vue template;
- React template;
- Vite dev/production integration;
- Inertia response integration;
- shared props and validation mapping;
- asset manifest handling.

Exit criteria:
- both generated projects build;
- browser navigation and form flow work;
- protocol integration tests pass;
- API/domain logic remains shared.

## Phase 7.1 — Omnisan on PATH and crates.io CLI (planned)

Does **not** block Phase 7 app/template work. Tracks distribution only.

Deliver:

| Item | Status |
| --- | --- |
| `omnisan` / `rustbridge` binaries in `omium-cli` | Done |
| `omnisan self-install` (`cargo install --path … --bin omnisan`) | Done |
| `./omnisan` → delegate to `~/.cargo/bin/omnisan` when installed | Done |
| Owner decision **O-001** (library crate names, license) | Partial — CLI crate **`omnisan`** reserved (**D-018**) |
| Publish `rustbridge-*` libraries to crates.io | Planned |
| Publish full **`omnisan`** CLI from monorepo (replace placeholder) | Planned |
| Generator emits `{ version = "…" }` instead of path deps | Planned |
| End-user docs: `cargo install omnisan` and empty-dir `omnisan new .` | Planned |

Exit criteria:

- `cargo install omnisan` (or approved equivalent) installs a working CLI on a clean machine with Rust only;
- `omnisan --version` and `omnisan new` work **without** a Framework git clone once Steps 3–5 are complete;
- monorepo developers can keep using `self-install` or path install until publish.

**Execution rule:** do not publish to crates.io or rename the CLI crate publicly until **O-001** is approved.

## Phase 8 — API-only experience

Deliver:
- API-only generator;
- JSON DTO/error conventions;
- API versioning convention;
- OpenAPI support after crate selection approval;
- CORS configuration.

Exit criteria:
- no Node dependency;
- API tests;
- documented API example.

## Phase 9 — Security and production readiness

Deliver only after approval for auth/session architecture:
- authentication;
- authorization;
- session/CSRF;
- rate limiting;
- deployment checklist;
- dependency audit;
- production config checks.

Exit criteria:
- security acceptance checklist passes;
- threat model reviewed;
- owner approves release candidate.

## Phase 10 — Post-MVP ecosystem

Deferred:
- queue;
- cache;
- scheduler;
- mail/notifications;
- broadcasting/WebSockets;
- SSR;
- plugin system;
- Laravel migration analyzer/converter;
- official starter kits.

Each item requires roadmap prioritization and owner approval before work begins.

## Execution rule

The agent must not implement multiple phases in one uninterrupted run. Complete one phase, report results, teach the owner, and wait for the owner's direction before crossing the next phase gate.

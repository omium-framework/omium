# Dependency inventory

This file tracks baseline dependencies aligned with `docs/DECISIONS.md` (D-006–D-015). Versions are declared in the workspace root `Cargo.toml` under `[workspace.dependencies]` when reserved for upcoming phases. **Crates do not enable these dependencies until the relevant roadmap phase.**

| Area | Crate(s) | First use (roadmap) | Status in workspace |
| --- | --- | --- | --- |
| Async runtime | `tokio` | Phase 1–2 | Reserved in workspace |
| HTTP | `axum`, `tower`, `tower-http` | Phase 2 | Reserved |
| ORM | `sea-orm`, `sea-orm-migration` | Phase 3 | Reserved |
| Raw SQL escape hatch | `sqlx` | Phase 3+ | Reserved |
| Serialization | `serde`, `serde_json` | Phase 1+ | Reserved |
| Errors | `thiserror`, `anyhow` | Phase 1+ | Reserved |
| CLI | `clap` | Phase 4 | Reserved |
| Views | `askama` | Phase 5 | `omium-view`, `sample-app` |
| Observability | `tracing`, `tracing-subscriber` | Phase 1 | Reserved |
| Inertia adapter | `inertia_rs` 0.4 (bridge) | Phase 7 | `omium-inertia`, `inertia-vue-app`, spike example |

Last updated: Phase 5 Askama views and sample-app templates.

## Version policy

- Verify crate versions on crates.io before adoption in a crate manifest.
- Pin through `Cargo.lock` once dependencies are used by application or example binaries.
- Run `cargo audit` (or equivalent) before release candidates (Phase 9).

## Phase 0

Only path dependencies between workspace crates are active. No third-party crates are linked yet.

## Phase 1

| Crate | Version (lockfile) | Used in |
| --- | --- | --- |
| `serde` | workspace / lock | `omium-core` config types |
| `toml` | 0.8.x | config file parsing |
| `thiserror` | 2.x | `ConfigError`, `BootstrapError` |
| `tokio` | 1.x | shutdown signals, CLI/sample `main` |
| `tracing`, `tracing-subscriber` | 0.1 / 0.3 | structured logging at bootstrap |
| `tempfile` | 3.x | dev-only config tests |

Last updated: Phase 1 bootstrap and configuration.

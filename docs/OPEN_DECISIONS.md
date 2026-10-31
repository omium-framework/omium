# Open decisions requiring owner approval

These questions are intentionally not decided by the AI agent.

## O-001 — Public product identity
- Framework name.
- ~~CLI crates.io crate name~~ — **decided:** [`omnisan`](https://crates.io/crates/omnisan) (see **D-018**); command **`omnisan`** (**D-017**).
- **Product placeholder on crates.io:** [`omium`](https://crates.io/crates/omium).
- ~~Crate namespace for published libraries~~ — **decided:** **`omium-*`** on crates.io and in monorepo (**D-019**).
- ~~License (CLI)~~ — MIT on published crates (matches workspace).
- Domain and visual identity.
**Remaining:** framework marketing name, domain, and visual identity before claiming a stable public **1.0** release.

## O-002 — Inertia Rust adapter

Candidates evaluated in Phase 6: see **`docs/INERTIA_SPIKE_REPORT.md`**.

- `inertia-omega` — official ALPHA, git/path, MSRV 1.99
- `inertia_rs` — crates.io 0.4, Axum spike in repo

**Required:** owner records choice in D-013 (approve one path, defer MSRV, or reject both).

## O-003 — Authentication and sessions

**Status:** APPROVED — **D-020**; implementation in progress (phases A1–A7). See **`docs/AUTH_DESIGN_PROPOSAL.md`**.

## O-004 — OpenAPI
Select crate and generation strategy for API-only mode. Compare code-first and schema-first options. Ask owner before adding dependency.

## O-005 — Frontend folder and config naming
The proposed layout and `rustbridge.toml` filename are provisional. Confirm before first stable generator release.

## O-006 — Supported MySQL versions
Verify SeaORM driver compatibility and choose a documented server support matrix.

## O-007 — Release and compatibility policy
Choose MSRV policy, semantic versioning, deprecation window, and release cadence before public release.

## O-008 — Queue/cache/scheduler
Choose backend and semantics only when the relevant roadmap phase is prioritized.

## O-009 — SSR
Decide whether SSR is required, which runtime hosts it, timeout/fallback behavior, and operational requirements.

## O-010 — Automatic Laravel migration tooling
Decide scope after core framework is usable. A lossless PHP-to-Rust business-logic conversion must not be promised.

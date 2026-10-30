# Open decisions requiring owner approval

These questions are intentionally not decided by the AI agent.

## O-001 — Public product identity
- Framework name.
- CLI command name.
- Crate namespace.
- License.
- Domain and visual identity.
**Required:** owner decision before public package publishing.

## O-002 — Inertia Rust adapter
Candidates to evaluate:
- `inertia-omega`
- `inertia_rs`
- official-protocol implementation behind our own adapter, only if existing options fail review.

**Required:** technical spike and owner approval before production integration.

## O-003 — Authentication and sessions
Choose architecture and maintained crates for:
- password hashing;
- session store;
- cookies;
- CSRF;
- token authentication;
- email verification and password reset.

**Required:** threat model + options + owner approval before implementation.

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

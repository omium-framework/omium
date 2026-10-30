# Global rules — highest priority

These rules apply to every AI agent, every task, and every generated file.

## Product identity

- Build a real Rust framework. All application code is valid Rust compiled by Cargo.
- Do not create a PHP-like language, transpiler, runtime interpreter, or fake Rust syntax.
- The framework may provide Laravel-inspired conventions and ergonomic APIs, but it must preserve Rust's actual type system, ownership, borrowing, and compile-time guarantees.
- The first audience is experienced PHP/Laravel/CodeIgniter/Symfony developers moving to Rust.
- The framework is frontend-independent at its core.

## Approved design principles

1. Convention over configuration, with explicit escape hatches.
2. Batteries included, but modular crates and optional features.
3. Mature ecosystem integrations before custom implementations.
4. Secure defaults; insecure behavior must require explicit opt-in and must be documented.
5. Predictable generated code that developers can inspect and edit.
6. No hidden runtime magic that undermines compile-time guarantees.
7. Avoid unnecessary abstraction and premature optimization.
8. Public APIs must be coherent, documented, tested, and stable before release.

## No autonomous decisions

Ask the owner before:
- Selecting or replacing a core dependency not approved in `docs/DECISIONS.md`.
- Adding a database engine or changing the supported database matrix.
- Changing ORM conventions or model lifecycle behavior.
- Designing authentication, authorization, cookies, sessions, CSRF, CORS, or password policy.
- Changing Inertia protocol support, frontend build strategy, or SSR behavior.
- Adding a new first-class frontend framework.
- Introducing a plugin system, code generation macro, reflection-like system, or custom DSL.
- Changing package names, framework name, CLI name, license, pricing, telemetry, or public roadmap.
- Introducing unsafe Rust, FFI, native extensions, or custom cryptography.
- Changing minimum supported Rust version (MSRV) or edition.
- Making a breaking public API change.
- Expanding scope beyond the current roadmap milestone.

For each approval request, explain:
1. The decision to make.
2. Why it matters.
3. At least two viable options when applicable.
4. Your recommendation and trade-offs.
5. The exact consequence of choosing each option.

Do not ask for approval for routine implementation details already covered by approved decisions.

## Rust quality

- Use stable Rust only unless approved otherwise.
- Use Rust 2024 edition.
- `unsafe_code = "forbid"` across the workspace.
- Prefer explicit error types and `Result`.
- Avoid production `unwrap()` and `expect()`.
- Use `Option` for genuinely optional values; do not encode errors as missing values.
- Avoid needless cloning; explain unavoidable clones in hot paths.
- Use `Arc` for shared ownership only where needed.
- Keep async request paths non-blocking.
- Use traits for stable extension boundaries, not abstraction for its own sake.
- Keep public types and functions documented.
- Use `cargo fmt`, `cargo clippy --workspace --all-targets --all-features -- -D warnings`, and `cargo test --workspace` as baseline checks, adjusting only when workspace features require it.

## Change discipline

- One milestone or vertical slice at a time.
- No giant unreviewable generated code dumps.
- No placeholder `todo!()` or `unimplemented!()` in code claimed complete.
- No commented-out old implementation.
- No fake tests that only assert constants or mock the behavior under test.
- Update examples and docs when public behavior changes.
- Never claim tests pass unless they were actually run.

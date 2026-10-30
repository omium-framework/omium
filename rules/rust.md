# Rust implementation rules

- Use idiomatic stable Rust, Rust 2024 edition.
- Keep `unsafe_code = "forbid"` at workspace level.
- Use `thiserror` for library/domain error enums. Use `anyhow` only in binaries, build scripts, or top-level application startup where contextual propagation is appropriate.
- Return `Result<T, E>` from fallible operations. Do not panic for user input, configuration errors, database errors, or network errors.
- Use `?` for error propagation; convert errors at clear boundaries.
- Use `Option<T>` only for absence, not failure.
- Avoid `clone()` as a default fix for borrow-checker errors. First explain ownership and borrowing, then choose references, moves, or shared ownership intentionally.
- Prefer borrowed inputs for read-only APIs when ergonomic and useful.
- Use `Arc<T>` for shared immutable state across tasks; use synchronization primitives only when mutation is required.
- Do not hold a synchronous mutex guard across `.await`.
- Do not block Tokio worker threads with synchronous filesystem, network, or database calls.
- Avoid unbounded channels and unbounded task spawning.
- Use structured tracing fields; never interpolate secrets into log messages.
- Public APIs must have rustdoc examples where practical.
- Use feature flags to keep optional integrations optional.
- Avoid exposing third-party crate types in public framework APIs unless the framework explicitly intends to re-export them as stable API.
- Keep adapter boundaries so Axum, SeaORM, Inertia, Askama, and Tower can be upgraded or replaced without rewriting application business logic.
- Do not add macros to imitate PHP syntax. Derive/procedural macros may be considered only with an approved ADR.

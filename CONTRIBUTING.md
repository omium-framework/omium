# Contributing

Thank you for contributing to RustBridge. This repository combines framework code, architecture decisions, and AI agent rules.

## Before you change code

1. Read `AGENTS.md` and `rules/00-global.md`.
2. Check `docs/DECISIONS.md` for approved choices and `docs/OPEN_DECISIONS.md` for items that need owner approval.
3. Work in one roadmap phase at a time (`docs/IMPLEMENTATION_ROADMAP.md`).

## Local checks

From the repository root:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo run -p rustbridge-cli
```

CLI subcommands (run from an application root such as `examples/sample-app`):

```bash
cargo run -p rustbridge-cli -- serve --config-dir config
cargo run -p rustbridge-cli -- doctor --config-dir config
cargo run -p rustbridge-cli -- route:list
cargo run -p rustbridge-cli -- new my-api --frontend api --yes
cargo run -p rustbridge-cli -- make:service Product --yes
```

Generated `new` apps require a nearby `crates/` directory (monorepo layout). Overwriting existing generator output requires `--yes`.

## Pull requests

- Keep changes small and reviewable.
- Include tests when behavior changes.
- Do not add core dependencies without updating `docs/DEPENDENCY_INVENTORY.md` and obtaining approval when required.
- Commit messages for human contributors: Turkish, format `type:kısa açıklama` (see project git rules).

## Security

See `docs/SECURITY.md` and `rules/security.md`. Never commit secrets or log sensitive data.

# Contributing

Thank you for contributing to Omium. This repository combines framework code, architecture decisions, and AI agent rules.

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
cargo run -p omium-cli
```

CLI subcommands (run from an application root such as `examples/standard`):

```bash
cargo run -p omium-cli -- serve --config-dir config
cargo run -p omium-cli -- doctor --config-dir config
cargo run -p omium-cli -- route:list
cargo run -p omium-cli -- new my-api --frontend api --yes
cargo run -p omium-cli -- new my-web --frontend standard --yes
cargo run -p omium-cli -- make:service Product --yes
```

Generated `new` apps require a nearby `crates/` directory (monorepo layout). Overwriting existing generator output requires `--yes`.

## What stays out of git

The root `.gitignore` excludes machine-local and generated paths. Do **not** force-add these:

| Path | Reason |
|------|--------|
| `.cursor/` | Cursor rules and local agent settings |
| `.idea/` | JetBrains IDE metadata |
| `target/`, `node_modules/`, `public/build/` | Build and install output |
| `.env` (not `.env.example`) | Secrets and local config |

Project conventions for humans and agents live in `rules/`, `AGENTS.md`, and this file — not in `.cursor/`.

## Pull requests

- Keep changes small and reviewable.
- Include tests when behavior changes.
- Do not add core dependencies without updating `docs/DEPENDENCY_INVENTORY.md` and obtaining approval when required.
- Commit messages: **English**, Conventional Commits style, optional leading emoji for clarity.
  - Format: `type: emoji Short summary` (e.g. `feat: ✨ Add Vue template nav middleware`)
  - Types: `feat`, `fix`, `docs`, `chore`, `refactor`, `test`, `ci` — use `!` after type for breaking changes (`refactor!:`).
  - Body optional; explain *why* when the subject alone is not enough.

## Security

See `docs/SECURITY.md` and `rules/security.md`. Never commit secrets or log sensitive data.

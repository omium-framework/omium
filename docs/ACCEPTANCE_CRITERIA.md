# Acceptance criteria

## Product-level

- [ ] All application code is real Rust.
- [ ] Cargo is the build/package authority.
- [ ] Laravel-inspired conventions are documented.
- [ ] Generated app structure is predictable.
- [ ] Frontend mode is selected during project creation.
- [ ] Standard, Vue, React, and API modes have distinct, tested templates.
- [ ] Domain/service logic is independent of frontend mode.
- [ ] No false claims about security or performance.

## Core

- [ ] Typed configuration loads from TOML.
- [ ] Environment overrides are documented and tested.
- [ ] Startup validation fails safely.
- [ ] Axum routing and Tower middleware are integrated.
- [ ] Typed errors map to safe HTTP responses.
- [ ] Structured tracing is available.
- [ ] Graceful shutdown is implemented.

## Database

- [ ] SeaORM is the default ORM.
- [ ] MySQL is the initial first-class database.
- [ ] SeaORM Migration is integrated.
- [ ] Transactions are supported.
- [ ] SQL uses parameter binding.
- [ ] Migrations are explicit deployment operations.
- [ ] DTOs are separate from persistence entities.

## Frontend

- [ ] Standard uses Askama.
- [ ] Vue mode uses Vue 3 + TypeScript + Inertia + Vite.
- [ ] React mode uses React + TypeScript + Inertia + Vite.
- [ ] API mode has no frontend package.
- [ ] Inertia protocol compatibility is tested.
- [ ] Production assets use Vite manifest.
- [ ] SSR is not advertised in MVP.

## CLI

- [ ] New app generator works.
- [ ] Mode selection works.
- [ ] Generators produce valid Rust.
- [ ] Destructive operations protect user files.
- [ ] Help and error messages are actionable.

## AI readiness

- [ ] Root `AGENTS.md` exists.
- [ ] Canonical `rules/` directory exists.
- [ ] Cursor and GitHub Copilot entry points reference canonical rules.
- [ ] Generated apps include a copy of project-specific rules.
- [ ] AI agents are required to ask before unapproved decisions.
- [ ] AI agents are required to teach using PHP/Laravel comparisons.

## Release gate

No public stable release until:
- security review complete;
- dependency/license inventory complete;
- docs and examples complete;
- all generator modes tested;
- protocol compatibility documented;
- owner explicitly approves public name, license, versioning, and release.

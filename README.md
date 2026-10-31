# Omium

**Omium** is a Rust-native web stack for HTTP apps, server-rendered HTML, JSON APIs, and Vue/React frontends. Everything flows through one developer tool: **[`omnisan`](https://crates.io/crates/omnisan)** — install it once, then create projects, run dev servers, migrate databases, and scaffold code from any directory.

Applications are normal **Cargo projects**: compiled Rust, explicit types, `Result`-based errors. Omium libraries (`omium-core`, `omium-http`, `omium-db`, `omium-view`, `omium-inertia`) ship on **[crates.io](https://crates.io/)**; the monorepo in this repository is where those crates and the CLI are developed together.

**Shipped today:** unified app layout (all frontend modes), `omnisan new` (API, Standard, Vue, React), dev/build/migrate generators, Vue + Inertia reference template, Askama standard mode, SeaORM migrations, validation, HTTP middleware. **Roadmap:** auth, queues, extended API tooling — see [`docs/IMPLEMENTATION_ROADMAP.md`](docs/IMPLEMENTATION_ROADMAP.md).

---

## Requirements

| Tool | Purpose |
| --- | --- |
| [Rust](https://rustup.rs/) (stable, edition 2024) | Build Omium libraries and your apps |
| [Cargo](https://doc.rust-lang.org/cargo/) | Dependencies and project management |
| **`omnisan`** | Omium CLI (see below) |
| [Node.js](https://nodejs.org/) + npm | Vue/React apps (`omnisan install`) |
| MySQL (optional) | Migrations and database-backed demos |

---

## Install `omnisan` (start here)

From **[crates.io](https://crates.io/crates/omnisan)** (recommended for app developers):

```bash
cargo install omnisan
omnisan --version
```

You do **not** need to clone this repository to build user applications. Clone the repo only if you contribute to Omium itself or work from path dependencies inside the monorepo.

**Contributors** working inside this checkout can also install from source:

```bash
cd Framework
cargo install --path crates/omium-cli --bin omnisan --locked
# or: ./omnisan self-install
```

Ensure `~/.cargo/bin` is on your `PATH`. Generated apps include a local `./omnisan` wrapper that prefers the global binary when present.

Publish order and version alignment: [`docs/CRATES_IO_PUBLISH.md`](docs/CRATES_IO_PUBLISH.md).

---

## Create an application

```bash
omnisan new my-app --frontend vue --app-name "My App" --yes
cd my-app
omnisan install          # Vue/React: npm dependencies
omnisan dev
```

**Frontend modes:** `standard` (Askama HTML), `vue`, `react` (Inertia + Vite), `api` (JSON). Every mode uses the **same directory layout** — see [`docs/APP_DIRECTORY_LAYOUT.md`](docs/APP_DIRECTORY_LAYOUT.md).

Interactive prompts when flags are omitted:

```bash
omnisan new my-app
```

**Monorepo checkout:** run `omnisan new` from a path under this repo (e.g. `examples/my-app`). The generator rewrites `omium-*` dependencies to `../../crates/…` automatically. Optional: add the app and `my-app/migration` to the root [`Cargo.toml`](Cargo.toml) `[workspace.members]` if you want a single `cargo build --workspace` from the framework root.

Do not generate into `crates/omium-cli/templates/*` — use a new folder.

More detail: [`docs/CLI_AND_GENERATORS.md`](docs/CLI_AND_GENERATORS.md), [`docs/END_USER_WORKFLOW.md`](docs/END_USER_WORKFLOW.md).

---

## Quick start (clone this repo)

For framework hacking or the bundled example:

```bash
git clone <your-clone-url> Framework
cd Framework
cargo build --workspace
omnisan doctor
```

**Standard (Askama) example:**

```bash
cd examples/standart
omnisan dev
# http://127.0.0.1:8080/  (default port from config/server.toml)
```

**New Vue app from template** (any directory, crates.io deps):

```bash
omnisan new ~/Projects/demo --frontend vue --app-name Demo --yes
cd ~/Projects/demo
omnisan install
export APP_DATABASE__ENABLED=true APP_DATABASE__PASSWORD=omium   # optional
omnisan migrate
omnisan dev
```

| Service | URL (Vue dev defaults) |
| --- | --- |
| Rust HTTP | http://127.0.0.1:3002 |
| Vite (started by `omnisan dev`) | http://127.0.0.1:5173 |

Production assets: `omnisan build` then `omnisan serve --production`. Routes: `omnisan route:list`.

---

## Daily commands (`omnisan`)

Run inside an **application directory** (contains `Cargo.toml` and `config/`):

| Command | Purpose |
| --- | --- |
| `omnisan install` | Frontend dependencies (Vue/React) |
| `omnisan dev` | Dev server; Vite HMR + Rust reload where configured |
| `omnisan serve` | Same family as `dev`; `--production` for built assets |
| `omnisan build` | Frontend build; optional `--release` for Rust |
| `omnisan migrate` | SeaORM migrations in `migration/` |
| `omnisan run` | Rust server only (no Vite) |
| `omnisan route:list` | Print registered routes |
| `omnisan doctor` | Environment and config checks |
| `omnisan make:controller` / `make:model` / `make:migration` / `make:service` | Code stubs (wire routes yourself) |

---

## Application layout (all modes)

Same tree for **API**, **Standard**, **Vue**, and **React**:

```text
config/                 # app, server, logging, database (TOML)
migration/              # SeaORM migrations
resources/
  css/                  # Shared styles
  js/                   # Vite entry (Vue/React)
  views/                # HTML (Standard) or pages/layouts (Inertia)
src/
  app/                  # Bootstrap, shared state, errors
  controllers/          # HTTP handlers
  routes/               # Route registration + route:list manifest
  models/               # SeaORM entities
  services/             # Application services
  forms/                # Input / validation helpers
  middleware/           # Web and API middleware groups
  http/                 # Rendering helpers (HTML, Inertia, JSON)
  views/                # Askama template structs → resources/views
  runtime.rs            # Server lifecycle
  main.rs
omnisan                   # Local CLI wrapper (optional)
```

Full guide: [`docs/APP_DIRECTORY_LAYOUT.md`](docs/APP_DIRECTORY_LAYOUT.md).

---

## Add a feature (typical flow)

From your app directory:

1. **`omnisan make:controller Product --yes`** → `src/controllers/product.rs`; add `pub mod product;` and a route in `src/routes/mod.rs`; update `src/routes/manifest.rs`.
2. **`omnisan make:model Product --yes`** → entity under `src/models/` — see [`docs/DATABASE_AND_ORM.md`](docs/DATABASE_AND_ORM.md).
3. **`omnisan make:migration create_products_table --yes`** → implement migration, register in `migration/src/lib.rs`, then **`omnisan migrate`**.
4. **`omnisan make:service Product --yes`** (optional) → call from controllers.

Standard mode: add `resources/views/*.html` and matching `src/views/*.rs` Askama structs. Vue/React: add `resources/js/Pages/*` (and optional `resources/js/Layouts/*`) and return Inertia from the controller.

---

## Configuration

- **`config/`** — committed defaults (`app.toml`, `server.toml`, `logging.toml`, `database.toml`)
- **`.env`** — local overrides from `.env.example`; never commit secrets
- **Environment keys:** `APP_` prefix, `__` for nesting — e.g. `APP_NAME`, `APP_SERVER__PORT`, `APP_DATABASE__PASSWORD`

Database passwords belong in the environment, not in TOML. Local MySQL: [`docs/LOCAL_MYSQL.md`](docs/LOCAL_MYSQL.md).

---

## Examples in this repository

| Path | Description |
| --- | --- |
| [`examples/standart`](examples/standart/) | Standard (Askama) app — unified layout, `./omnisan dev` |

Additional reference apps are produced by **`omnisan new`** (Vue/React templates live under `crates/omium-cli/templates/`).

---

## Framework development (contributors)

```bash
cargo fmt --all
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings   # when applicable
```

Read [`AGENTS.md`](AGENTS.md) and [`CONTRIBUTING.md`](CONTRIBUTING.md). Decisions: [`docs/DECISIONS.md`](docs/DECISIONS.md). Open questions: [`docs/OPEN_DECISIONS.md`](docs/OPEN_DECISIONS.md).

After CLI or generator changes, publish to crates.io per [`docs/CRATES_IO_PUBLISH.md`](docs/CRATES_IO_PUBLISH.md).

### Workspace crates

```text
crates/
├── omium-core      # bootstrap, config, app state
├── omium-http      # HTTP server, default middleware
├── omium-db        # database integration
├── omium-view      # server-rendered views
├── omium-inertia   # Inertia adapter
├── omium-cli       # omnisan library + generator templates
└── omium-testing   # test helpers
```

Published binary crate: **[`omnisan`](https://crates.io/crates/omnisan)** (wraps `omium-cli`).

---

## Documentation

| Document | Content |
| --- | --- |
| [`docs/APP_DIRECTORY_LAYOUT.md`](docs/APP_DIRECTORY_LAYOUT.md) | Unified app folder structure |
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | Request lifecycle, boundaries |
| [`docs/CLI_AND_GENERATORS.md`](docs/CLI_AND_GENERATORS.md) | CLI and generators |
| [`docs/FRONTEND_MODES.md`](docs/FRONTEND_MODES.md) | Standard, Vue, React, API |
| [`docs/CRATES_IO_PUBLISH.md`](docs/CRATES_IO_PUBLISH.md) | Registry publish workflow |
| [`docs/SECURITY.md`](docs/SECURITY.md) | Security baseline |
| [`docs/IMPLEMENTATION_ROADMAP.md`](docs/IMPLEMENTATION_ROADMAP.md) | Planned vs shipped |
| [`rules/`](rules/) | Rules for agents and contributors |

---

## Security

See [`docs/SECURITY.md`](docs/SECURITY.md). Do not commit `.env` or credentials. Workspace forbids `unsafe` in production crates (`unsafe_code = "forbid"`).

---

## License

License terms are **not yet published** in this repository. See [`docs/OPEN_DECISIONS.md`](docs/OPEN_DECISIONS.md) until public terms are approved.

---

## Support and contribution

- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`docs/DEPENDENCY_POLICY.md`](docs/DEPENDENCY_POLICY.md)
- [`docs/PRODUCT_VISION.md`](docs/PRODUCT_VISION.md)

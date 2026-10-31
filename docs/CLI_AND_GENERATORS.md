# CLI and project generators

## CLI goals

Provide a Laravel Artisan-like workflow while remaining a Cargo-native Rust project. Clap is the selected CLI parsing library.

**Approved binaries (D-017):** `omnisan` and `rustbridge` (same commands).

### Run without global install (required workflow)

Use **`./omnisan`** only — same idea as Laravel's `php artisan` / `./artisan`. Do **not** use `cargo omnisan`; the wrapper invokes Cargo internally.

| Where you are | Example |
| --- | --- |
| Application directory | `./omnisan install` then `./omnisan dev` |
| Framework repo root | `./omnisan doctor` |

First invocation compiles the CLI once; later runs are fast. No global install on the machine or server.

## Commands (all via `./omnisan`)

```bash
./omnisan install       # npm install (Vue/Inertia apps)
./omnisan dev           # Vite HMR + cargo watch (Rust auto-restart on src changes)
./omnisan dev --no-rust-watch   # single cargo run (manual restart)
./omnisan serve         # debug Vue apps → same as dev; production use --production
./omnisan serve --production   # built assets from public/build only
./omnisan build         # npm run build (+ optional --release)
./omnisan migrate
./omnisan run           # cargo run without starting Vite
./omnisan route:list    # app routes (Vue apps with src/bin/route_list.rs)
./omnisan new my-app --frontend vue --app-name Omium
./omnisan doctor
```

### Vue local development vs production build

| Goal | Command |
| --- | --- |
| Edit `.vue` files with hot reload | `./omnisan dev` or `./omnisan serve` (debug apps auto-start Vite) |
| Test production assets | `./omnisan build` then `./omnisan serve --production` |

After `./omnisan build`, debug mode reloads the Vite manifest on each request (no Rust restart). In production (`debug = false`), restart the server after a frontend build.

### Rust hot reload (`cargo-watch`)

`./omnisan dev` runs **`cargo watch -x run …`** so controller/route/service changes recompile and restart the HTTP process. If `cargo-watch` is not installed yet, **`./omnisan dev` or `./omnisan install` runs `cargo install cargo-watch` once** (requires network).

Use `./omnisan dev --no-rust-watch` if you prefer a single long-lived `cargo run`.

## `new` command

```bash
./omnisan new examples/my-shop
```

When flags are omitted and the terminal is interactive, omnisan asks for:

1. **Application name** (default: title case from folder name, e.g. `My Shop`)
2. **Frontend mode** — `standard`, `vue` (default), `api`, or `react`

Non-interactive (CI/script):

```bash
./omnisan new examples/my-shop --frontend vue --app-name "Acme Shop" --yes --no-interaction
```

Framework `crates/` is discovered by walking up from the target path (works for `examples/*` nested apps).

Must prompt for:
1. Application name.
2. Frontend mode: Standard / Vue / React / API.
3. Database: initially MySQL only.
4. Optional starter features only when approved and implemented.

The command must:
- generate valid Rust/Cargo project;
- create the correct frontend package only for Vue/React;
- generate config files and `.env.example`;
- generate a health endpoint and one example resource;
- generate tests;
- generate `README.md` and local AI rules;
- print next steps and required tools.

## Generators

Generators must produce idiomatic, readable Rust. Generated files are starting points, not hidden magic.

- Controller generator: async handler skeleton with typed response.
- Model generator: SeaORM entity scaffold or documented entity generation workflow.
- Migration generator: SeaORM Migration scaffold.
- Service generator: plain Rust service module.
- Test generator: appropriate unit/integration test scaffold.

Do not invent a custom Rust parser or source rewriter for generators. Use templates for initial scaffolding. If AST editing is proposed, request approval.

## CLI behavior

- Errors must explain what failed and how to fix it.
- Destructive commands require confirmation unless an explicit `--yes` flag is provided.
- `--help` must be complete.
- Commands should be deterministic and testable.
- Support non-interactive CI usage.
- Never silently overwrite user files.

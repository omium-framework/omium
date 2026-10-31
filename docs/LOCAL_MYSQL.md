# Local MySQL for Phase 3 development

Use Docker Compose for a local MySQL 8.4 instance aligned with generated app defaults (`database.name=omium`, user `omium`).

## Start MySQL

From the repository root:

```bash
docker compose up -d
docker compose ps
```

Wait until the service is healthy.

## phpMyAdmin (optional UI)

The same Compose file starts phpMyAdmin on port **8081** (example apps use HTTP **8080** by default).

```bash
open http://127.0.0.1:8081
```

Sign in with the default app credentials:

| Field | Value |
| --- | --- |
| Server | `mysql` (pre-filled via `PMA_HOST`) |
| Username | `omium` |
| Password | `omium` |

Root access (local dev only): user `root`, password `root`.

Do not expose these containers to the public internet; they are for local development only.

## Enable database in an example app

1. One-time setup (Laravel `.env` style — no manual export before each `dev`/`serve`):

```bash
cd examples/standard
./omnisan install    # .env.example → .env (APP_DATABASE__PASSWORD=omium)
```

The app reads `.env` beside `config/` on startup; when a password is set, `database.enabled` turns on.

2. Apply migrations (explicit step — not run on app boot):

```bash
./omnisan migrate
```

3. Start the HTTP server:

```bash
./omnisan dev
```

The same flow applies to other templates under `examples/` or apps created with `omnisan new`.

## Optional integration tests

When MySQL is running:

```bash
export RUSTBRIDGE_TEST_MYSQL_URL=mysql://omium:omium@127.0.0.1:3306/omium
cargo test -p omium-db --test mysql_integration -- --ignored
```

## Laravel comparison

- `.env` `DB_*` values ↔ `config/database.toml` + `APP_DATABASE__*` overrides
- `php artisan migrate` ↔ `omnisan migrate` (or `cargo run --manifest-path migration/Cargo.toml`)
- Eloquent model ↔ `examples/standard/src/models/*.rs` SeaORM entities

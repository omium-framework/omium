# Local MySQL for Phase 3 development

Use Docker Compose for a local MySQL 8.4 instance aligned with sample-app defaults (`database.name=omium`, user `omium`).

## Start MySQL

From the repository root:

```bash
docker compose up -d
docker compose ps
```

Wait until the service is healthy.

## phpMyAdmin (optional UI)

The same Compose file starts phpMyAdmin on port **8081** (sample-app HTTP uses **8080**).

```bash
open http://127.0.0.1:8081
```

Sign in with sample-app credentials:

| Field | Value |
| --- | --- |
| Server | `mysql` (pre-filled via `PMA_HOST`) |
| Username | `omium` |
| Password | `omium` |

Root access (local dev only): user `root`, password `root`.

Do not expose these containers to the public internet; they are for local development only.

## Enable database in sample-app

1. One-time setup (Laravel `.env` gibi — her `dev`/`serve` öncesi export gerekmez):

```bash
cd examples/sample-app
./omnisan install    # .env.example → .env (APP_DATABASE__PASSWORD=omium)
```

Uygulama açılırken `config/` yanındaki `.env` otomatik okunur; şifre varsa `database.enabled` açılır.

2. Apply migrations (explicit deployment step — not run on app boot):

```bash
./omnisan migrate
```

3. Start the HTTP server:

```bash
./omnisan dev
```

## Optional integration tests

When MySQL is running:

```bash
export RUSTBRIDGE_TEST_MYSQL_URL=mysql://omium:omium@127.0.0.1:3306/omium
cargo test -p omium-db --test mysql_integration -- --ignored
```

## Laravel comparison

- `.env` `DB_*` values ↔ `config/database.toml` + `APP_DATABASE__*` overrides
- `php artisan migrate` ↔ `omnisan migrate` (or `cargo run --manifest-path migration/Cargo.toml`)
- Eloquent model ↔ `examples/sample-app/src/models/*.rs` SeaORM entities

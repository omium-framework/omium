# Vue + Inertia application base (Phase 7)

Reference app for Omium Vue mode. Use **`./omnisan`** for all tasks (like `php artisan`).

| Process | Port |
| --- | --- |
| Rust (via `./omnisan dev`) | **3002** |
| Vite dev (started by omnisan) | **5173** |

## CSRF (Inertia visits)

Each page includes a shared `csrfToken` prop (from the session). The root layout keeps `<meta name="csrf-token">` in sync, and `resources/js/app.ts` sends `X-XSRF-TOKEN` on every Inertia visit (POST/PUT/PATCH/DELETE). Form posts also send `_token` in the body where required.

## Development (hot reload)

```bash
cd examples/inertia-vue-app
./omnisan install
export APP_DATABASE__ENABLED=true APP_DATABASE__PASSWORD=omium
./omnisan migrate
./omnisan dev
open http://127.0.0.1:3002/
```

Edit files under `resources/js/Pages/` — Vite HMR updates the browser without restarting Rust. `./omnisan serve` in `debug = true` does the same (starts Vite automatically).

## Production assets

```bash
./omnisan build
./omnisan serve --production
```

## New app from template

```bash
cd /path/to/Framework
./omnisan new examples/my-vue-app --frontend vue --app-name Omium --yes
```

See `docs/CLI_AND_GENERATORS.md`.

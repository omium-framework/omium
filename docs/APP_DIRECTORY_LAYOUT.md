# Application directory layout (all frontend modes)

Every `omnisan new` app — **API**, **Standard**, **Vue**, or **React** — uses the same Omium application tree. Standard uses Askama under `resources/views/`; Vue/React keep frontend under `resources/js/` (Laravel + Inertia style).

```
app/
  config/                 # TOML config (app, server, database, logging)
  migration/              # SeaORM migrations (same crate layout in all modes)
  resources/
    css/                  # Shared Omium starter styles
    js/                   # Vue/React: app.ts(x), Pages/, Layouts/
    views/                # Standard (Askama): layout.html, *.html
  src/
    app/                  # Bootstrap, RuntimeState, errors
    controllers/          # HTTP actions (register in routes/)
    models/               # SeaORM entities
    routes/               # Route registration + manifest for route:list
    services/             # Domain / application services
    forms/                # Form/request helpers
    middleware/           # web / api middleware groups
    http/                 # Rendering helpers (HTML, Inertia, JSON)
    views/                # Askama Template structs → resources/views/*.html
    runtime.rs            # Thin server entry (bootstrap + serve)
    lib.rs
    main.rs
    bin/route_list.rs
  askama.toml               # Standard: Askama template search path
  omnisan                   # Local CLI wrapper
```

## Adding a page (Standard)

1. `resources/views/my_page.html` (extends `layout.html`)
2. `src/views/my_page.rs` — Askama struct
3. `src/controllers/my_page.rs` — handler
4. `src/routes/mod.rs` — `.route(..., get(my_page::show))`
5. Update `src/routes/manifest.rs` for `./omnisan route:list`

## Adding a page (Vue / React)

1. `resources/js/Pages/MyPage.vue` or `.tsx`
2. `src/controllers/my_page.rs` — `render_inertia(..., "MyPage", ...)`
3. `src/routes/mod.rs` + `manifest.rs` (same as Standard)

Use `omnisan make:controller`, `make:model`, etc. — paths always match the layout above.

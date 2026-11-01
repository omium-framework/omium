# Authentication and sessions — design proposal (O-003)

**Status:** APPROVED (**D-020**, owner 2026-11-01). Implementation proceeds **A1 → A7** in order.  
**Goal:** Laravel-familiar behavior (session login for web, CSRF, optional API tokens) without coupling business logic to Axum or Inertia.

---

## 1. Threat model (scope)

| Threat | Mitigation (planned) |
|--------|----------------------|
| Credential stuffing / brute force | Rate limit login route (Phase 9 hook; login-specific limit in auth crate) |
| Session fixation | Regenerate session ID on login |
| Session theft (XSS) | `HttpOnly` session cookie; no session id in JS |
| CSRF on browser forms / Inertia | Synchronizer token tied to session; validate on state-changing methods |
| Password database leak | Argon2id hashes; no plaintext; constant-time verify |
| IDOR / broken authz | **Out of scope for this doc** — enforced in app services (see `docs/SECURITY.md`) |
| API token leak | Hashed token storage; revoke + scoped abilities (Sanctum-style) |

**Non-goals for v1:** OAuth social login, WebAuthn/passkeys, MFA (can extend later).

---

## 2. Architecture

### New workspace crate: `omium-auth`

| Layer | Responsibility |
|-------|----------------|
| **`omium-core`** | `UserId`, auth-related config types (no crypto) |
| **`omium-auth`** | Password hashing, session store trait + DB driver, CSRF, login/logout helpers, token issue/revoke |
| **`omium-http`** | Axum middleware layers: `StartSession`, `VerifyCsrf`, `RequireAuth` (thin wrappers calling `omium-auth`) |
| **`omium-inertia`** | Shared prop `csrfToken` + optional `auth` user DTO for layouts |
| **Application** | `users` entity, routes (`/login`, `/logout`), policies in services |

Dependency rule: `omium-auth` → `omium-core`, `omium-db` (for session/user persistence). It must **not** depend on `omium-inertia` or Askama.

Update `rules/architecture.md` crate name: **`omium-auth`** (replacing provisional `rustbridge-auth`).

---

## 3. Recommended crates (owner choice)

| Concern | Recommendation | Alternatives |
|---------|----------------|--------------|
| Password hashing | **`argon2`** + **`password-hash`** (Argon2id, OWASP-aligned params) | `bcrypt` (Laravel default parity; weaker story) |
| Cookie signing | **`tower-cookies`** or **`axum-extra`** `CookieJar` + app secret | Roll own HMAC (discouraged) |
| Session payload | Server-side store keyed by opaque cookie | Encrypted cookie-only session (later driver) |
| CSRF | Session-stored token; header `X-XSRF-TOKEN` + form field `_token` (Laravel-like) | Double-submit cookie only (weaker for some flows) |
| API tokens | DB table `personal_access_tokens` (plain token shown once; store SHA-256 hash) | JWT (stateless revoke is hard) |
| Email (reset / verify) | **`lettre`** SMTP + dev **[Mailpit](https://github.com/axllent/mailpit)** (Compose service, SMTP `:1025`, UI `:8025`) | Log driver (tests only) |

No new dependency without an entry in `docs/DEPENDENCY_INVENTORY.md` after approval.

---

## 4. Session model

### Cookie

| Attribute | Value |
|-----------|--------|
| Name | `omium_session` (configurable) |
| `HttpOnly` | yes |
| `SameSite` | `Lax` (dev); `Strict` optional in prod config |
| `Secure` | `true` when `APP_ENV=production` |
| Path | `/` |

### Store (v1 drivers — owner **C2**)

**Database** (Laravel `database` driver):

- Table `sessions`: `id`, `user_id` (nullable), `ip_address`, `user_agent`, `payload`, `last_activity`.
- Cookie holds only opaque session id.

**File** (Laravel `file` driver):

- One file per session under `storage/framework/sessions/` (path configurable).
- Same cookie id; payload serialized on disk (locked writes).

Config: `config/auth.toml` → `session.driver = "database" | "file"`.

**Later drivers:** `redis`, encrypted cookie — same `SessionStore` trait.

### Signing key

- `APP_KEY` or `APP_SESSION__SECRET` (32+ bytes) from env — required in production; dev may generate with loud warning (like Laravel `key:generate` via future `omnisan key:generate`).

---

## 5. CSRF

- Issued when session starts; rotated on login.
- **Web / Standard (Askama):** hidden input `_token` + meta tag for JS.
- **Inertia (Vue/React):** shared prop `csrfToken`; axios/fetch default header `X-XSRF-TOKEN` (document in template).
- **API group:** no CSRF (stateless); use Bearer token or no auth for public routes.
- Validate on `POST`, `PUT`, `PATCH`, `DELETE` for routes in `web` middleware group.

---

## 6. Authentication flows (v1)

### Web (session)

1. `GET /login` — form (guest middleware).
2. `POST /login` — validate email/password → verify Argon2 → `session.regenerate()` → redirect intended URL.
3. `POST /logout` — invalidate session.
4. `RequireAuth` middleware — redirect to `/login` (HTML) or 401 JSON if `Accept: application/json`.

### API (optional module)

- `Authorization: Bearer {token}` — lookup hash in `personal_access_tokens`.
- Middleware `RequireApiToken` on `api` routes (configurable).
- `omnisan make:api-token` or artisan-like command later.

### Email (owner **C5**)

- **Email verification** and **password reset** flows ship in v1 (not deferred).
- **Local dev:** extend root `docker-compose.yml` with **Mailpit**; `config/mail.toml` points SMTP to `127.0.0.1:1025`.
- **Production:** real SMTP via env (`APP_MAIL__*` overrides).
- “Remember me” remains **v1.1** (optional long-lived cookie).

---

## 7. Database schema (generator migration)

**`users`** (minimal):

- `id`, `email` (unique), `password` (hash), `email_verified_at` (nullable), `created_at`, `updated_at`.

**`sessions`** — as above.

**`personal_access_tokens`** (optional feature flag in template):

- `id`, `user_id`, `name`, `token` (hash), `abilities` (text/json), `last_used_at`, `expires_at`, timestamps.

SeaORM entities live in **app** `src/models/`; `omium-auth` provides traits/helpers, not app-specific tables in the framework crate.

**Database driver:** use `omium_auth::DatabaseSessionStore` with a `sessions` table matching `omium_auth::session::database::Model` (or run equivalent migration):

```sql
CREATE TABLE sessions (
  id VARCHAR(255) PRIMARY KEY,
  user_id BIGINT NULL,
  ip_address VARCHAR(45) NULL,
  user_agent TEXT NULL,
  payload TEXT NOT NULL,
  last_activity BIGINT NOT NULL
);
```

**HTTP:** attach `omium_http::start_session` with `SessionMiddlewareState::new(store, SessionCookieConfig::default())`. Handlers use `axum::Extension<SessionHandle>`.

---

## 8. Configuration

New file **`config/auth.toml`** (generated in all web templates):

```toml
[session]
driver = "database" # or "file"
cookie = "omium_session"
lifetime_minutes = 120
file_path = "storage/framework/sessions"

[csrf]
enabled = true

[mail]
driver = "smtp"
host = "127.0.0.1"
port = 1025
from_address = "noreply@omium.local"
from_name = "Omium"
```

Env overrides: `APP_SESSION__SECRET`, `APP_AUTH__SESSION_LIFETIME`, etc. (pattern matches existing `APP_DATABASE__*`).

---

## 9. Middleware wiring (templates)

**`web` group** (order matters):

1. `inject_request_presentation` (existing)
2. `start_session`
3. `share_csrf` (extensions + Inertia props)
4. `verify_csrf` (state-changing only)
5. `authenticate` (optional per-route via route metadata or sub-groups)

**`api` group:**

1. `authenticate_api_token` (optional global or per-route)

Replace demo `web_group` logging middleware in examples/templates when implementing.

---

## 10. Implementation phases (after approval)

| Phase | Deliverable | Exit |
|-------|-------------|------|
| **A1** | `omium-auth` crate: Argon2 hash/verify | unit tests ✅ |
| **A2** | Session stores: **database + file**; cookie middleware in `omium-http` | integration tests ✅ (file + middleware) |
| **A3** | CSRF + standard login/logout routes in template | Askama form test ✅ |
| **A4** | Inertia shared `csrfToken` + login page | manual + inertia test ✅ |
| **A5** | Personal access tokens (API template) | bearer auth test ✅ |
| **A6** | Email verify + password reset + Mailpit in Compose | SMTP integration test ✅ |
| **A7** | `omnisan make:user` / auth migrations in generator | docs |

Publish: bump `omium-*` patch versions together; update `OMIUM_REGISTRY_VERSION`.

---

## 11. Owner approval checklist

**Recorded 2026-11-01:**

- [x] **C1** — **`omium-auth`** + Argon2 + signed session cookie.
- [x] **C2** — Session drivers v1 = **`database` and `file`**.
- [x] **C3** — CSRF = session synchronizer token.
- [x] **C4** — API = **`personal_access_tokens`** (not JWT).
- [x] **C5** — **Email verify + password reset** with **Mailpit** for local dev.
- [x] **C6** — Implement **sequentially** A1 → A7.

See **D-020** in `docs/DECISIONS.md`.

---

## 12. References

- `docs/SECURITY.md` — baseline requirements  
- `docs/OPEN_DECISIONS.md` — O-003  
- `docs/IMPLEMENTATION_ROADMAP.md` — Phase 9  
- Laravel: session, CSRF, Sanctum (conceptual parity only)

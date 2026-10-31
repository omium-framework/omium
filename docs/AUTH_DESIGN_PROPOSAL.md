# Authentication and sessions — design proposal (O-003)

**Status:** PROPOSED — awaiting owner approval before any auth code ships.  
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
| Email (reset / verify) | **Deferred v1.1** — trait + no-op driver; document hooks | `lettre` + queue (Phase 10) |

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

### Store (v1 driver)

**Database sessions** (Laravel `database` driver analogue):

- Table `sessions`: `id`, `user_id` (nullable), `ip_address`, `user_agent`, `payload` (encrypted JSON blob or serialized), `last_activity`.
- Cookie holds only session id (random 40+ bytes, base64 or hex).

**Later drivers:** `redis`, `cookie` (encrypted payload) — same `SessionStore` trait.

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

### Not in v1

- Registration (app can add; generator optional later).
- Email verification & password reset (design hooks only).
- “Remember me” long-lived cookie (v1.1).

---

## 7. Database schema (generator migration)

**`users`** (minimal):

- `id`, `email` (unique), `password` (hash), `email_verified_at` (nullable), `created_at`, `updated_at`.

**`sessions`** — as above.

**`personal_access_tokens`** (optional feature flag in template):

- `id`, `user_id`, `name`, `token` (hash), `abilities` (text/json), `last_used_at`, `expires_at`, timestamps.

SeaORM entities live in **app** `src/models/`; `omium-auth` provides traits/helpers, not app-specific tables in the framework crate.

---

## 8. Configuration

New file **`config/auth.toml`** (generated in all web templates):

```toml
[session]
driver = "database"
cookie = "omium_session"
lifetime_minutes = 120

[csrf]
enabled = true

[password]
# Argon2id parameters (optional overrides)
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
| **A1** | `omium-auth` crate: Argon2, `UserCredentials` verify trait | unit tests |
| **A2** | DB session store + cookie middleware in `omium-http` | integration test with MySQL |
| **A3** | CSRF + standard login/logout routes in template | Askama form test |
| **A4** | Inertia shared `csrfToken` + login page | manual + inertia test |
| **A5** | Personal access tokens (API template) | bearer auth test |
| **A6** | `omnisan make:user` / migration in generator | docs |

Publish: bump `omium-*` patch versions together; update `OMIUM_REGISTRY_VERSION`.

---

## 11. Owner approval checklist

Reply **approve** (or list changes) for each item:

- [ ] **C1** — New crate **`omium-auth`** + dependencies (`argon2`, `password-hash`, session via DB + signed cookie).
- [ ] **C2** — Session driver v1 = **database only** (Redis later).
- [ ] **C3** — CSRF = **session synchronizer token** (Laravel-style `_token` + `X-XSRF-TOKEN`).
- [ ] **C4** — API auth v1 = **`personal_access_tokens`** table (not JWT).
- [ ] **C5** — Defer **email verification / password reset / remember-me** to v1.1.
- [ ] **C6** — Start implementation at **Phase A1** (no full stack in one PR).

When approved, record as **D-020** in `docs/DECISIONS.md` and close **O-003** items tied to v1 scope.

---

## 12. References

- `docs/SECURITY.md` — baseline requirements  
- `docs/OPEN_DECISIONS.md` — O-003  
- `docs/IMPLEMENTATION_ROADMAP.md` — Phase 9  
- Laravel: session, CSRF, Sanctum (conceptual parity only)

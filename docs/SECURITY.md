# Security baseline

## Security positioning

Rust's memory-safety model prevents many classes of memory-safety bugs in safe Rust. It does not make an application automatically secure. SQL injection, broken authorization, XSS, CSRF, SSRF, insecure deserialization, leaked secrets, weak passwords, and business logic flaws remain possible.

Do not market the framework as unhackable or claim PHP applications are inherently insecure.

## Required baseline

### HTTP
- Request body size limits.
- Request timeouts.
- Safe header handling.
- Security headers with documented defaults.
- Explicit CORS allowlist.
- Rate limiting is planned; backend choice requires approval.
- Do not expose stack traces or internal error details.

### Database
- Parameterized queries.
- Least-privilege DB credentials.
- Connection and query timeouts.
- No SQL secrets in logs.
- Explicit transaction boundaries.

### Authentication
Authentication is a separate security design gate. Do not implement password/session/auth flows until owner approves:
- session model;
- cookie attributes;
- CSRF strategy;
- password hashing crate and parameters;
- password reset and email verification behavior;
- token lifecycle;
- remember-me behavior;
- account lockout/rate limiting;
- multi-device session handling.

### Authorization
- Authorization must be enforced server-side at the service/use-case boundary.
- UI hiding is not authorization.
- Test access denial, cross-user access, and object ownership checks.

### Frontend
- Escape template output by default.
- Do not inject untrusted HTML.
- Inertia props must be explicitly serialized DTOs.
- Never expose secrets, password hashes, tokens, or internal fields in props.
- Apply CSRF protections to cookie-authenticated state-changing browser requests once auth design is approved.

### Secrets and configuration
- Commit `.env.example` with placeholders only.
- Never commit `.env`.
- Redact sensitive config from debug output.
- Fail startup when required production secrets are missing.
- Do not create default production passwords or signing keys.

### Supply chain
- Review dependency licenses and maintenance.
- Run `cargo audit` or an approved equivalent in CI.
- Keep dependency versions reproducible.
- Review transitive dependency changes.
- No telemetry by default.

## Security acceptance

A release candidate must include:
- dependency audit report;
- authorization tests;
- input validation tests;
- safe error response tests;
- secret redaction tests;
- request/body limit tests;
- CORS/CSRF tests where applicable;
- documented deployment security checklist.

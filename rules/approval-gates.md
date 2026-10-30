# Approval gates

The owner must explicitly approve the following before implementation or public commitment:

1. Public framework name, CLI binary name, crate namespace, domain, logo, and license.
2. Inertia Rust adapter selection after a technical spike and protocol compatibility report.
3. Authentication/session/CSRF design and password hashing crate.
4. Database support beyond the initial approved database.
5. Public ORM conventions, model lifecycle hooks, relationship API, and query API.
6. Whether to support migrations in production with automatic execution or require explicit deployment commands.
7. SSR architecture and runtime.
8. Plugin/extension system and third-party package registry.
9. Any telemetry, analytics, update checks, or outbound network behavior.
10. Any use of unsafe Rust, FFI, native code, or custom cryptography.
11. Any change to MSRV, Rust edition, release cadence, or semantic versioning policy.
12. Any breaking public API change.
13. Any scope increase that delays the current milestone.

Approval request format:
- Decision:
- Why it matters:
- Option A:
- Option B:
- Recommendation:
- Risks:
- Exact approval question:

Do not proceed until the owner explicitly chooses or approves an option.

# Testing and quality rules

Every behavior must have tests at the appropriate level.

- Unit tests for core logic, configuration parsing, validation, and pure transformations.
- Integration tests for HTTP routing, middleware, database operations, migrations, auth, and Inertia protocol behavior.
- End-to-end tests for generated Standard, Vue, React, and API project templates.
- Use a real test database for DB integration tests; do not rely solely on mocks.
- Test both successful and failure paths.
- Test tenant/data isolation if multi-tenant helpers are ever introduced.
- Add regression tests for every fixed bug.
- Test generated project output by creating a temporary app and running its build/test commands.
- Inertia tests must cover initial HTML response, X-Inertia JSON response, asset version conflict, redirects, validation errors, partial reloads, shared props, and any advanced feature claimed as supported.
- Benchmark only after correctness tests; benchmarks must state hardware, versions, and methodology.
- A feature is not complete because it compiles. It is complete only when acceptance criteria and docs are satisfied.

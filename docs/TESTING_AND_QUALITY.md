# Testing and quality strategy

## Workspace checks

Baseline commands:
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

Add checks only after confirming tool availability and workspace feature behavior:
```bash
cargo audit
```

## Test layers

### Unit
- config parsing and precedence;
- error mapping;
- route metadata helpers;
- response construction;
- DTO conversion;
- pure utility functions.

### Integration
- Axum routes and extractors;
- Tower middleware;
- SeaORM against real MySQL test service;
- migration up/down behavior;
- Standard rendering;
- Inertia protocol adapter;
- API JSON responses.

### Generator acceptance
For every frontend mode:
1. Run CLI generator.
2. Verify expected file set.
3. Run `cargo check`.
4. Run `cargo test`.
5. For Vue/React, install frontend dependencies and run production build.
6. For API-only, verify no Node.js dependency is required.
7. Verify `.env.example` contains no real secret.

### Inertia protocol suite
- first browser request returns HTML;
- Inertia request returns JSON page object;
- correct `X-Inertia` behavior;
- asset version handling and stale asset response;
- redirects;
- validation errors;
- shared props;
- partial reloads;
- advanced features only if advertised.

## Performance

- Correctness first.
- Benchmark representative CRUD/API endpoints only after feature behavior is stable.
- Compare with equivalent application logic and deployment settings.
- Publish methodology, hardware, compiler version, dependency versions, payload, concurrency, and latency percentiles.
- Never publish cherry-picked or unreproducible numbers.

## Definition of done

A task is done only when:
- code compiles;
- relevant tests pass;
- formatting and linting pass;
- docs/examples are updated;
- no unresolved security concern is hidden;
- owner receives the required Turkish learning report.

# Frontend and Inertia rules

- Frontend selection is made at project creation: `standard`, `vue`, `react`, or `api`.
- `standard` means server-rendered HTML using Askama; it does not mean a custom JavaScript framework.
- `vue` means Vue 3 + TypeScript + official Inertia Vue client + Vite.
- `react` means React + TypeScript + official Inertia React client + Vite.
- `api` means no bundled frontend; provide versioned JSON API conventions and OpenAPI documentation.
- All modes share the same application services and domain logic.
- Never put business rules in Vue/React components.
- Inertia pages receive explicit serializable DTO props; do not expose DB models or secrets.
- Shared props must be minimal and must not contain private user/company data unless authorized.
- Implement Inertia protocol features only when covered by protocol tests.
- Do not invent a custom Inertia protocol. Follow the official protocol version selected in the decision register.
- Vite is the frontend build tool for Vue/React modes. The Rust server must read the generated manifest in production and use the Vite dev server only in development.
- Keep frontend dependencies isolated in the generated frontend package.
- Do not install Vue and React together in a generated app unless a future approved use case requires it.
- API-only projects must not require Node.js to compile or run the Rust backend.
- Do not promise SSR in the initial release. SSR is a later, separately approved milestone.

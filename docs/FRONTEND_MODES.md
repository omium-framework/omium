# Frontend modes

## Product requirement

At project creation the developer must choose one mode:

- `standard`
- `vue`
- `react`
- `api`

The chosen mode determines generated files and dependencies. The backend service/domain layer remains the same.

## Mode A — Standard

Purpose: simple websites, admin pages, forms, and server-rendered applications without a mandatory SPA frontend.

Technology:
- Askama templates.
- Axum responses.
- Optional small progressive-enhancement JavaScript only when needed.

Requirements:
- Typed view context structs.
- Shared layout and reusable partial conventions.
- Form submission and validation error rendering.
- CSRF integration only after approved auth/security design.
- Escaped output by default.
- No custom JavaScript framework.
- No Node.js requirement for the Standard app's basic backend build.

PHP/Laravel analogy: closest to Blade + controller responses, but templates are compiled/checked through Askama and Rust types.

## Mode B — Vue + Inertia

Technology:
- Vue 3.
- TypeScript.
- Official Inertia Vue client for the selected Inertia protocol version.
- Vite.
- Tailwind CSS as generated-template default.

Requirements:
- Rust route/controller owns URL and server-side data access.
- Handler returns Inertia component name and serializable props.
- Vue renders the page.
- Inertia handles client-side navigation and page visits.
- No separate client-side router.
- No duplicate REST API solely to hydrate these pages.
- API endpoints may coexist for mobile/third-party clients.
- Shared props must be minimal and authorized.
- Use DTOs; never pass secrets or unrestricted ORM entities.

PHP/Laravel analogy: Laravel + Inertia + Vue. The server is Rust and handlers are actual Rust functions.

## Mode C — React + Inertia

Same server contract as Vue mode, with:
- React.
- TypeScript.
- Official Inertia React client.
- Vite.
- Tailwind CSS default template.

Do not install Vue dependencies in React projects or vice versa.

## Mode D — API only

Purpose: independent frontend, mobile clients, third-party integrations, microservices.

Requirements:
- JSON request/response conventions.
- Versioned API route convention, provisionally `/api/v1`.
- DTOs and validation.
- Consistent error envelope.
- OpenAPI generation/documentation is a planned requirement; exact crate must be selected and approved before implementation.
- CORS is explicit configuration, never wildcard by default.
- No frontend package or Node.js requirement.

## Inertia protocol requirements

- Follow the official Inertia protocol version selected in `DECISIONS.md`.
- First visit returns HTML root view and page data.
- Inertia visits return the correct JSON page object and headers.
- Implement asset versioning and stale asset handling.
- Implement redirects and validation errors correctly.
- Support shared props and partial reloads before calling the integration production-ready.
- Advanced protocol features (deferred props, merge props, once props, history encryption, infinite scroll, SSR) must be individually tested and marked supported/unsupported.
- SSR is deferred from the initial release.
- Keep third-party adapter behind a framework-owned internal interface.

## Mode selection and switching

- The generator must not silently install multiple frontend stacks.
- Changing a project's frontend mode after creation is not a guaranteed automatic migration.
- Provide a documented manual migration path later.
- Do not claim switching is safe until a generator/migration tool has tests.

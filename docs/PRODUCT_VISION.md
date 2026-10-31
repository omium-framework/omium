# Product vision

## Working definition

Omium is a Rust-native web application framework with a Laravel-inspired developer experience. The codename is provisional and is not a public naming decision.

## Mission

Help experienced PHP developers move to Rust without requiring them to abandon familiar web application architecture and productivity patterns.

## Primary audience

1. PHP/Laravel developers who want stronger compile-time guarantees and native performance.
2. CodeIgniter/Symfony developers modernizing existing skills.
3. Teams building APIs, SaaS products, admin panels, and business applications.
4. New Rust developers who want an opinionated web application foundation.

## What this product is

- A framework for real Rust applications.
- A cohesive application platform built from mature Rust ecosystem components.
- Convention over configuration.
- Full-stack optionality: server-rendered Standard, Vue + Inertia, React + Inertia, API-only.
- A learning bridge: familiar concepts, real Rust code, clear explanations.

## What this product is not

- Not a PHP runtime.
- Not a PHP-to-Rust transpiler in the initial scope.
- Not a new programming language or PHP syntax clone.
- Not a replacement for Cargo.
- Not a promise that Rust prevents all security vulnerabilities.
- Not a frontend JavaScript framework.
- Not a reason to rewrite every existing PHP system immediately.

## Positioning

"Laravel-inspired productivity. Real Rust. Secure-by-default foundations."

Do not make unsubstantiated claims such as "10x faster", "unhackable", or "PHP is insecure". Performance claims require reproducible benchmarks. Security claims must be narrowly scoped and evidence-based.

## Product principles

1. Familiar structure, idiomatic Rust.
2. Explicit types over hidden magic.
3. Strong defaults without locking users in.
4. Shared business logic across frontend modes.
5. Mature crates over reinvented infrastructure.
6. Documentation and examples as first-class deliverables.
7. Smooth onboarding for PHP developers.
8. Security and maintainability before benchmark theater.

## Success criteria

- A Laravel developer can create and run a CRUD application quickly.
- The developer can understand generated Rust code rather than being forced to use opaque abstractions.
- The same service layer works with Standard, Vue + Inertia, React + Inertia, and API-only transports.
- A generated application has migrations, validation, tests, structured logs, and a clear configuration model.
- The framework has reproducible examples, protocol tests, security checks, and honest compatibility documentation.
